use core::str::from_utf8;
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
}

pub struct ChatRoom {
    clients: Vec<Client>,
}

impl ChatRoom {
    pub fn new() -> ChatRoom {
        ChatRoom {
            clients: Vec::new(),
        }
    }

    pub fn join(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn bloadcast(&self, buf: &[u8], socket: UdpSocket, inbound: SocketAddr) {
        for client in &self.clients {
            if inbound == client.src {
                continue;
            }
            socket
                .send_to(buf, &client.src)
                .expect("Failed to send data back");
        }
    }

    pub fn joined_clinets_info(&self) {
        for client in &self.clients {
            println!(
                "username: {:?} \n last_send: {:?} \n src: {:?}",
                client.username, client.last_send, client.src
            );
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
                let buf = &mut buf[1..amt];
                let req_msg = from_utf8(&buf).expect("Failed to receive data");

                if cmd == 49 {
                    chat_room.join(Client::new(req_msg.to_string(), src));
                } else if cmd == 50 {
                    let socket_clone = socket.try_clone().unwrap();
                    chat_room.bloadcast(buf, socket_clone, src);
                }

                println!("{:}", "=".repeat(80));
                println!("buffer size: {:?}", amt);
                println!("src address: {:?}", &src);
                println!("request message: {:?}", req_msg);
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
