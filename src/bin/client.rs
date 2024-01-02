use core::str;
use std::io::{self, Write};
use std::net::UdpSocket;

enum Action {
    Join,
    Send,
    Quit,
}

struct User {
    name: String,
}

struct CliHander {}

impl CliHander {
    pub fn new() -> CliHander {
        CliHander {}
    }

    pub fn user_input(&self) -> String {
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        buffer
    }

    pub fn get_action(&self) -> Action {
        io::stdout().flush().unwrap();
        print!("Enter your choice: ");
        match self.user_input().trim().to_lowercase().as_str() {
            "1" => Action::Join,
            "2" => Action::Send,
            "0" => Action::Quit,
            _ => {
                println!("Invalid command entered.");
                Action::Quit
            }
        }
    }

    pub fn process_evnet(&self, socket: UdpSocket, server_address: &str) -> std::io::Result<()> {
        let mut buffer = [0; 4096];
        loop {
            match self.get_action() {
                Action::Join => {
                    let prefix = "1";
                    print!("Your name: ");
                    let username = self.user_input();
                    socket.send_to(
                        format!("{} {}", prefix, username).as_bytes(),
                        server_address,
                    )?;
                }
                Action::Send => {
                    let prefix = "2";
                    let message = self.user_input();
                    socket.send_to(format!("{} {}", prefix, message).as_bytes(), server_address)?;
                }
                Action::Quit => {
                    println!("Bye");
                    break;
                }
            }
            //             let (amt, _) = socket.recv_from(&mut buffer)?;
            //             println!(
            //                 "Recived message: {}",
            //                 String::from_utf8_lossy(&buffer[..amt])
            //             );
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_address = "127.0.0.1:34254";
    let clihander = CliHander::new();

    println!("\n {:}", "=".repeat(80));
    println!("Please select an Action: ");
    println!("1. Join Room");
    println!("2. Send Message");
    println!("0. Exit");
    println!("{:} \n", "=".repeat(80));

    clihander.process_evnet(socket, server_address);

    Ok(())
}
