use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::sync::mpsc;
use std::thread;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 4096;

#[derive(Debug)]
enum Method {
    Join,
    Send,
}

#[derive(Debug)]
struct Request {
    method: Option<Method>,
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

    fn broadcast(&self, request: Request) {
        for client in self.clients.values() {
            if client.name == request.name {
                continue;
            }
            match self
                .udp_socket
                .send_to(request.text.as_bytes(), client.src_addr)
            {
                Ok(_) => {
                    println!("succeeded");
                }
                Err(e) => {
                    println!("cloud not send a request.text: {:?}", e);
                }
            }
        }
    }
}

fn map_method(method: &str) -> Option<Method> {
    match method {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        _ => None,
    }
}

fn parse(buf: &[u8], src_addr: SocketAddr) -> (Request, Client) {
    let (method_byte, rest) = buf.split_at(1);
    let (name_bytes, text_bytes) = rest.split_at(7);

    (
        Request {
            method: map_method(encode_msg(method_byte)),
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

    let (tx, rx) = mpsc::channel::<(Request, Client)>();

    let mut buf = [0; MSG_SIZE];

    let mut room = Room {
        udp_socket: socket_clone,
        clients: HashMap::new(),
    };

    thread::spawn(move || {
        while let Ok(req) = rx.recv() {
            println!("{:?}", req);

            if let Some(ref method) = req.0.method {
                match method {
                    Method::Join => {
                        println!("join: {:?}", req.1);
                        room.join(req.0.name, req.1);
                    }
                    Method::Send => {
                        println!("send: {:?}", req.0);
                        room.broadcast(req.0);
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
