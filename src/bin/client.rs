use core::str;
use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const SERVER_ADDRESS: &str = "127.0.0.1:34254";
const MSG_SIZE: usize = 4087;
const NAME_SIZE: usize = 8;

struct Outbound {
    socket: UdpSocket,
}

impl Outbound {
    fn new(socket: UdpSocket) -> Outbound {
        Outbound { socket }
    }
}

struct User {
    name: String,
}

struct Hander {
    outbound: Outbound,
    user: User,
}

impl Hander {
    pub fn new(outbound: Outbound) -> Hander {
        Hander {
            outbound,
            user: User {
                name: "".to_string(),
            },
        }
    }

    pub fn join_user(&mut self, name: String) {
        self.user.name = name;
    }

    pub fn process_event(&mut self) -> std::io::Result<()> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        loop {
            if let Some(Ok(action)) = lines.next() {
                match action.trim().to_lowercase().as_str() {
                    "1" => {
                        println!("Your name: ");
                        let mut prefix: String = "1".to_string();
                        if let Some(Ok(mut username)) = lines.next() {
                            println!("Username received: {}", username);

                            username.truncate(NAME_SIZE);
                            prefix.push_str(username.as_str());
                            self.join_user(username);
                            self.outbound
                                .socket
                                .send_to(prefix.as_bytes(), SERVER_ADDRESS)?;
                        } else {
                            eprintln!("Failed to read username");
                        }
                    }
                    "2" => {
                        println!("Write a Message: ");
                        let mut prefix: String = "2".to_string();
                        if let Some(Ok(message)) = lines.next() {
                            println!("Message received: {}", message);

                            prefix.push_str(self.user.name.as_str());
                            prefix.push_str(message.as_str());
                            self.outbound
                                .socket
                                .send_to(prefix.as_bytes(), SERVER_ADDRESS)?;
                        } else {
                            eprintln!("Failed to read message");
                        }
                    }
                    "0" => {
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }
}

struct Inbound {
    socket: UdpSocket,
}

impl Inbound {
    pub fn new(socket: UdpSocket) -> Inbound {
        Inbound { socket: socket }
    }

    pub fn recv_datagram(&self) {
        loop {
            let mut buffer = [0; 4096];
            match self.socket.recv_from(&mut buffer) {
                Ok((amt, _)) => {
                    println!(
                        "Recived message: {}",
                        String::from_utf8_lossy(&buffer[..amt])
                    );
                }
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let inbound = Inbound::new(socket.try_clone().expect("Faield to clone udp socket"));
    let outbound = Outbound::new(socket);
    let (sender, receiver) = mpsc::channel::<String>();
    let mut hander = Hander::new(outbound);

    println!("\n {:}", "=".repeat(80));
    println!("Please select an Action: ");
    println!("1. Join Room");
    println!("2. Send Message");
    println!("0. Exit");
    println!("{:} \n", "=".repeat(80));

    thread::spawn(move || inbound.recv_datagram());
    hander.process_event();

    Ok(())
}
