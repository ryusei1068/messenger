use core::str;
use std::io::{self, BufRead, BufReader, Write};
use std::net::UdpSocket;

const SERVER_ADDRESS: &str = "127.0.0.1:34254";
const MSG_SIZE: usize = 4087;
const NAME_SIZE: usize = 8;

enum Action {
    Join,
    Send,
    Quit,
}

struct Outbound {
    socket: UdpSocket,
}

impl Outbound {
    fn new(socket: UdpSocket) -> Outbound {
        Outbound { socket }
    }
}

struct CliHander {
    outbound: Outbound,
}

impl CliHander {
    pub fn new(outbound: Outbound) -> CliHander {
        CliHander { outbound }
    }

    pub fn user_input(&self, max_byte: usize) -> std::io::Result<String> {
        io::stdout().flush()?;
        let stdin = io::stdin();
        let mut buffer = BufReader::new(stdin);
        let mut input = String::new();
        buffer.read_line(&mut input)?;

        input.truncate(max_byte);

        Ok(input)
    }

    pub fn get_action(&self) -> Action {
        if let Ok(input) = self.user_input(1) {
            match input.trim().to_lowercase().as_str() {
                "1" => Action::Join,
                "2" => Action::Send,
                "0" => Action::Quit,
                _ => {
                    println!("Invalid command entered.");
                    Action::Quit
                }
            }
        } else {
            println!("Failed to get users action.");
            Action::Quit
        }
    }

    pub fn process_event(&self) -> std::io::Result<()> {
        loop {
            print!("Enter your choice: ");
            match self.get_action() {
                Action::Join => {
                    print!("Your name: ");
                    let mut prefix: String = "1".to_string();
                    if let Ok(username) = self.user_input(NAME_SIZE) {
                        prefix.push_str(username.as_str());
                        self.outbound
                            .socket
                            .send_to(prefix.as_bytes(), SERVER_ADDRESS.to_string())?;
                    } else {
                        println!("Failed to get a username. Sorry please again.")
                    }
                }
                Action::Send => {
                    print!("Write a Message: ");
                    let mut prefix: String = "2".to_string();
                    if let Ok(message) = self.user_input(MSG_SIZE) {
                        prefix.push_str(message.as_str());
                        self.outbound
                            .socket
                            .send_to(prefix.as_bytes(), SERVER_ADDRESS.to_string())?;
                    } else {
                        println!("Failed to get a message. Sorry please again.")
                    }
                }
                Action::Quit => {
                    println!("Bye");
                    break;
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

    pub fn recv_datagram(&self) -> std::io::Result<()> {
        let mut buffer = [0; MSG_SIZE];
        let (amt, _) = self.socket.recv_from(&mut buffer)?;
        println!(
            "Recived message: {}",
            String::from_utf8_lossy(&buffer[..amt])
        );

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let inbound = Inbound::new(socket.try_clone().expect("Faield to clone udp socket"));
    let outbound = Outbound::new(socket);
    let clihander = CliHander::new(outbound);

    println!("\n {:}", "=".repeat(80));
    println!("Please select an Action: ");
    println!("1. Join Room");
    println!("2. Send Message");
    println!("0. Exit");
    println!("{:} \n", "=".repeat(80));

    let _ = clihander.process_event();

    Ok(())
}
