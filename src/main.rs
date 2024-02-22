use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::sync::mpsc;
use std::thread;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 4096;

#[derive(Debug)]
enum Request {
    Join,
    Send,
}

#[derive(Debug)]
struct Message {
    request: Option<Request>,
    name: String,
    text: String,
}

#[derive(Debug)]
struct Client {
    name: String,
    src_addr: SocketAddr,
}

#[derive(Debug)]
struct Room {
    udp_socket: UdpSocket,
    clients: HashMap<String, Client>,
}

impl Room {
    fn join(&mut self, name: String, client: Client) {
        self.clients.insert(name, client);
    }

    fn broadcast(&self, message: Message) {
        for client in self.clients.values() {
            match self
                .udp_socket
                .send_to(message.text.as_bytes(), client.src_addr)
            {
                Ok(_) => {
                    println!("succeeded");
                }
                Err(e) => {
                    println!("cloud not send a message: {:?}", e);
                }
            }
        }
    }
}

fn map_request(request: &str) -> Option<Request> {
    match request {
        "1" => Some(Request::Join),
        "2" => Some(Request::Send),
        _ => None,
    }
}

fn parse(buf: &[u8], src_addr: SocketAddr) -> (Message, Client) {
    let (request_byte, rest) = buf.split_at(1);
    let (name_bytes, text_bytes) = rest.split_at(7);

    (
        Message {
            request: map_request(encode_msg(request_byte)),
            name: encode_msg(name_bytes).into(),
            text: encode_msg(text_bytes).into(),
        },
        Client {
            name: encode_msg(name_bytes).into(),
            src_addr: src_addr,
        },
    )
}

fn encode_msg(buf: &[u8]) -> &str {
    match str::from_utf8(&buf) {
        Ok(msg) => msg,
        Err(e) => {
            println!("could not encode: {:?}", e);
            ""
        }
    }
}

fn main() {
    let socket = UdpSocket::bind(SERVER_ADDRESS).expect("could not bind UdpSocket");
    let socket_clone = socket.try_clone().expect("could not clone of UdpSocket");

    let (tx, rx) = mpsc::channel::<(Message, Client)>();

    let mut buf = [0; MSG_SIZE];

    let mut room = Room {
        udp_socket: socket_clone,
        clients: HashMap::new(),
    };

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("{:?}", msg);

            if let Some(ref request) = msg.0.request {
                match request {
                    Request::Join => {
                        println!("join: {}", msg.0.name);
                        room.join(msg.0.name, msg.1);
                    }
                    Request::Send => {
                        println!("send: {:?}", msg.0);
                        room.broadcast(msg.0);
                    }
                }
            }
        }
    });

    loop {
        match socket.recv_from(&mut buf) {
            Ok((buf_size, src_addr)) => {
                let buf = &mut buf[..buf_size];
                let parsed_request = parse(&buf, src_addr);
                let _ = tx.send(parsed_request);
            }
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }
    }
}
