use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::sync::mpsc;
use std::thread;

const UDP_SERVER_ADDRESS: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 4096;

#[derive(Debug)]
enum Method {
    Join,
    Send,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    from: String,
    to: String,
    text: String,
}

#[derive(Debug)]
struct Client {
    name: String,
    socket_addr: SocketAddr,
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
            if client.name == request.from {
                continue;
            }
            self.sender(client.socket_addr, request.text.as_bytes());
        }
    }

    fn send_private_msg(&self, request: Request) {
        if let Some(client) = self.get_client_by_name(request.to) {
            self.sender(client.socket_addr, request.text.as_bytes());
        }
    }

    fn get_client_by_name(&self, name: String) -> Option<&Client> {
        self.clients.get(&name)
    }

    fn sender(&self, socket_addr: SocketAddr, byte: &[u8]) {
        match self.udp_socket.send_to(byte, socket_addr) {
            Ok(_) => {
                println!("\nsucceeded");
            }
            Err(e) => {
                println!("\ncloud not send a request.text: {:?}", e);
            }
        }
    }
}

fn map_method(method: &String) -> Option<Method> {
    match method.as_str() {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        _ => None,
    }
}

fn parse(buf: &[u8], socket_addr: SocketAddr) -> Result<(Request, Client), String> {
    let req_byte = match std::str::from_utf8(&buf) {
        Ok(req_byte) => req_byte,
        Err(e) => {
            println!("Error converting bytes to string: {}", e);
            ""
        }
    };

    if req_byte.len() == 0 {
        return Err("failed to parse error".into());
    }

    match serde_json::from_str::<Request>(&req_byte) {
        Ok(req) => {
            let from = req.from.clone();
            Ok((
                req,
                Client {
                    name: from,
                    socket_addr: socket_addr,
                },
            ))
        }
        Err(e) => Err(format!("failed to deserialize {:?}", e)),
    }
}

fn main() {
    let socket = UdpSocket::bind(UDP_SERVER_ADDRESS).expect("could not bind UdpSocket");
    let socket_clone = socket.try_clone().expect("could not clone of UdpSocket");

    let (tx, rx) = mpsc::channel::<(Request, Client)>();

    let mut buf = [0; MSG_SIZE];

    let mut room = Room {
        udp_socket: socket_clone,
        clients: HashMap::new(),
    };

    thread::spawn(move || {
        while let Ok(req) = rx.recv() {
            if let Some(ref method) = map_method(&req.0.method) {
                match method {
                    Method::Join => {
                        println!("\njoin: {:?}", req.1);
                        let name = req.1.name.clone();
                        room.join(name, req.1);
                    }
                    Method::Send => {
                        println!("\nsend: {:?}", req.0);
                        room.broadcast(req.0);
                    }
                }
            }
        }
    });

    loop {
        match socket.recv_from(&mut buf) {
            Ok((buf_size, socket_addr)) => {
                let buf = &mut buf[..buf_size];
                if let Ok(parsed_request) = parse(&buf, socket_addr) {
                    let _ = tx.send(parsed_request);
                }
            }
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }
    }
}
