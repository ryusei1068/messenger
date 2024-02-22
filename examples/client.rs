use core::str;
use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const NAME_SIZE: usize = 8;

struct Outbound {
    socket: UdpSocket,
    receiver: Receiver<String>,
}

impl Outbound {
    fn new(socket: UdpSocket, receiver: Receiver<String>) -> Outbound {
        Outbound { socket, receiver }
    }

    fn send(&self) {
        while let Ok(msg) = self.receiver.recv() {
            match self.socket.send_to(msg.as_bytes(), SERVER_ADDRESS) {
                Ok(_) => {
                    println!("Message sent successfully.");
                }
                Err(e) => {
                    println!("Failed to send message {:?}", e);
                }
            };
        }
    }
}

struct User {
    name: String,
}

struct Handler {
    user: User,
    sender: Sender<String>,
}

impl Handler {
    fn new(sender: Sender<String>) -> Handler {
        Handler {
            user: User {
                name: "".to_string(),
            },
            sender,
        }
    }

    fn join_user(&mut self, name: String) {
        self.user.name = name;
    }

    fn send_channel(&mut self, msg: String) {
        if self.sender.send(msg).is_err() {
            println!("failed to send to the channel");
        }
    }

    fn process_events(&mut self) {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        loop {
            if let Some(Ok(action)) = lines.next() {
                match action.trim().to_lowercase().as_str() {
                    "1" => {
                        println!("Your name: ");
                        let mut prefix: String = "1".to_string();
                        if let Some(Ok(mut username)) = lines.next() {
                            // 8 byte adjust
                            username = if username.len() < 8 {
                                format!("{:<8}", username)
                            } else {
                                username.truncate(NAME_SIZE);
                                username
                            };

                            prefix.push_str(username.as_str());
                            self.join_user(username);

                            self.send_channel(prefix);
                        } else {
                            eprintln!("Failed to read username");
                        }
                    }
                    "2" => {
                        println!("Write a Message: ");
                        let mut prefix: String = "2".to_string();
                        if let Some(Ok(message)) = lines.next() {
                            prefix.push_str(self.user.name.as_str());
                            prefix.push_str(message.as_str());

                            self.send_channel(prefix);
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
    }
}

struct Inbound {
    socket: UdpSocket,
}

impl Inbound {
    fn new(socket: UdpSocket) -> Inbound {
        Inbound { socket }
    }

    fn recv_datagram(&self) {
        loop {
            let mut buffer = [0; 4096];
            match self.socket.recv_from(&mut buffer) {
                Ok((amt, _)) => {
                    println!(
                        "Received message: {}",
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
    let (sender, receiver) = mpsc::channel::<String>();

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let inbound = Inbound::new(socket.try_clone().expect("Failed to clone udp socket"));
    let outbound = Outbound::new(socket, receiver);

    let mut handler = Handler::new(sender);

    println!("{:}", "=".repeat(80));
    println!("Please select an Action: ");
    println!("1. Join Room");
    println!("2. Send Message");
    println!("0. Exit");
    println!("{:} \n", "=".repeat(80));

    thread::spawn(move || inbound.recv_datagram());
    thread::spawn(move || outbound.send());
    handler.process_events();

    Ok(())
}
