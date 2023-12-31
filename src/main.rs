use core::str::{self, from_utf8};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, SystemTime};

pub struct Client {
    username: String,
    last_send: SystemTime,
    src: SocketAddr,
}

impl Client {
    pub fn new(username: String, src: SocketAddr) -> Client {
        Client {
            username,
            last_send: SystemTime::now(),
            src,
        }
    }

    pub fn is_active(&self) -> bool {
        let now = SystemTime::now();
        match now.duration_since(self.last_send) {
            Ok(duration) => {
                if duration >= Duration::from_secs(10 * 60) {
                    true
                } else {
                    false
                }
            }
            Err(_) => true,
        }
    }

    pub fn update_last_send(&mut self) {
        self.last_send = SystemTime::now();
        println!("updated: {:?} \n last_send: {:?}", self.src, self.last_send);
    }
}

pub struct ChatRoom {
    clients: HashMap<String, Client>,
}

impl ChatRoom {
    pub fn new() -> ChatRoom {
        ChatRoom {
            clients: HashMap::new(),
        }
    }

    pub fn join(&mut self, username: String, client: Client) {
        self.clients.insert(username.to_string(), client);
    }

    pub fn bloadcast(&mut self, buf: &[u8], socket: UdpSocket, sender_name: String) {
        for (username, client) in self.clients.iter() {
            if *username == sender_name {
                continue;
            }
            socket
                .send_to(buf, &client.src)
                .expect("Failed to send data back");
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut chat_room = ChatRoom::new();

    let handle = thread::spawn(move || loop {
        let mut buf = [0; 4096];

        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                // 1byte read
                // if 1 join to chat_room (urf8 49 = 1)
                // if 2 send message (utf8 50 = 2)
                let cmd = buf[0];
                let mut buf_clone = buf.clone();

                let username_bytes = &mut buf_clone[1..9];
                let username = from_utf8(&username_bytes).expect("Failed to receive data");

                if cmd == 49 {
                    if username.len() <= 8 {
                        chat_room
                            .join(username.to_string(), Client::new(username.to_string(), src));
                        println!("{:}", "=".repeat(80));
                        println!("joined: {:?}", src);
                        println!("current clients: {:?}", chat_room.clients.len());
                        println!("{:}", "=".repeat(80));
                    } else {
                        let error_message = format!("Username byte length exceeded. Max length: 8");
                        socket.send_to(error_message.as_bytes(), &src);
                        println!("{:?}", error_message);
                    }
                } else if cmd == 50 {
                    let message_bytes = &mut buf[9..amt];
                    let socket_clone = socket.try_clone().unwrap();
                    chat_room.bloadcast(message_bytes, socket_clone, username.to_string());
                }

                println!("{:}", "=".repeat(80));
                println!("buffer size: {:?}", amt);
                println!("src address: {:?}", &src);
                println!("{:}", "=".repeat(80));
            }
            Err(e) => {
                println!("couldn't recieve request: {:?}", e);
            }
        }
    });

    handle.join().expect("Failed to join thread");
    Ok(())
}
