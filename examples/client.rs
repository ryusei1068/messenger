use core::str;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::io::{self};
use std::net::UdpSocket;
use std::thread;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const NAME_SIZE: usize = 8;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    from: String,
    text: String,
}

struct User {
    name: String,
    joined: bool,
}

impl User {
    fn join(&mut self) -> Option<Request> {
        let name = self.name.clone();
        let mut req = Request {
            method: "".into(),
            from: name,
            text: "".into(),
        };

        match input("\nDo you enter the room? (yes:1 / no:0)".into()) {
            Ok(input) => {
                if input == "1" {
                    self.joined = true;
                    req.method = "1".into();
                    Some(req)
                } else {
                    println!("Ok. Bye bye");
                    None
                }
            }
            Err(_) => {
                println!("Please try again:");
                None
            }
        }
    }

    fn enter_messege(&self) -> Option<Request> {
        let name = self.name.clone();
        let mut req = Request {
            method: "2".into(),
            from: name,
            text: "".into(),
        };

        match input("\nWrite messege: ".into()) {
            Ok(input) => {
                req.text = input;
                Some(req)
            }
            Err(_) => {
                println!("Please try again");
                None
            }
        }
    }
}

struct Handler {
    user: User,
    socket: UdpSocket,
}

impl Handler {
    fn new(user: User, socket: UdpSocket) -> Handler {
        Handler { user, socket }
    }

    fn process_events(&mut self) {
        loop {
            if !self.user.joined {
                if let Some(req) = self.user.join() {
                    self.send(req);
                } else {
                    break;
                }
            }
            match input("\nSend messege to your room? (yes:1 / no:0)".into()) {
                Ok(input) => {
                    if input == "1" {
                        if let Some(req) = self.user.enter_messege() {
                            self.send(req);
                        }
                    }
                }
                Err(_) => {
                    println!("Please try again");
                }
            }
            match input("\nLeave your room? (yes:1 / no:0)".into()) {
                Ok(input) => {
                    if input == "1" {
                        println!("Bye bye.");
                        break;
                    }
                }
                Err(_) => {
                    println!("Please try again");
                }
            }
        }
    }

    fn send(&self, req: Request) {
        match self.socket.send_to(
            serde_json::to_string(&req).unwrap().as_bytes(),
            SERVER_ADDRESS,
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to send message {:?}", e);
            }
        };
    }
}

fn trim_input(mut text: String, limit_length: usize) -> String {
    if text.len() < limit_length {
        format!("{:<8}", text)
    } else {
        text.truncate(limit_length);
        text
    }
}

fn input(prompt_msg: String) -> Result<String, String> {
    print!("{}: ", prompt_msg);

    match io::stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("failed to flush stdout: {}", e));
        }
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().into()),
        Err(e) => Err(format!("error reading input: {}", e)),
    }
}

fn main() {
    let user_name = trim_input(
        input("\nplease your name".into()).expect("cloud not read your name. Please try again."),
        NAME_SIZE,
    );
    println!("Your name is {}", user_name);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("cloud not bind UdpSocket");
    let socket_clone = socket.try_clone().expect("failed to clone UdpSocket");

    println!("\n{:}", "=".repeat(80));
    println!("Hi, {}. Welcome to the CLI Chat!!!!", user_name);
    println!("Please select an Action: ");
    println!("1. Join Room");
    println!("2. Send Message");
    println!("0. Exit");
    println!("{:} \n", "=".repeat(80));

    let user = User {
        name: user_name,
        joined: false,
    };
    let mut handler = Handler::new(user, socket);

    thread::spawn(move || loop {
        let mut buffer = [0; 4096];
        match socket_clone.recv_from(&mut buffer) {
            Ok((amt, _)) => {
                println!(
                    "\nReceived message: {}",
                    String::from_utf8_lossy(&buffer[..amt])
                );
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    });

    handler.process_events();
}
