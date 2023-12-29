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

    pub fn bloadcast(&self, buf: &[u8], socket: UdpSocket) {
        for client in &self.clients {
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
                chat_room.join(Client::new("test".to_string(), src));

                let buf = &mut buf[..amt];
                let req_msg = from_utf8(&buf).expect("Failed to receive data");
                println!("{:}", "=".repeat(80));
                println!("buffer size: {:?}", amt);
                println!("src address: {:?}", &src);
                println!("request message: {:?}", req_msg);

                let socket_clone = socket.try_clone().unwrap();
                chat_room.bloadcast(buf, socket_clone);

                // socket.send_to(buf, &src).expect("Failed to send data back");
            }
            Err(e) => {
                println!("couldn't recieve request: {:?}", e);
            }
        }
    });

    handle.join().expect("Failed to join thread");
    Ok(())
}
