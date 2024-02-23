use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    from: String,
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
            if client.name == request.from {
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

fn map_method(method: &String) -> Option<Method> {
    match method.as_str() {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        _ => None,
    }
}

fn parse(buf: &[u8], src_addr: SocketAddr) -> (Request, Client) {
    let req_byte = match std::str::from_utf8(&buf) {
        Ok(req_byte) => req_byte,
        Err(e) => {
            println!("Error converting bytes to string: {}", e);
            ""
        }
    };

    let req1: Request = serde_json::from_str(&req_byte).unwrap();
    let req2: Request = serde_json::from_str(&req_byte).unwrap();

    (
        req1,
        Client {
            name: req2.from,
            src_addr: src_addr,
        },
    )
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

            if let Some(ref method) = map_method(&req.0.method) {
                match method {
                    Method::Join => {
                        println!("join: {:?}", req.1);
                        let name = req.1.name.clone();
                        room.join(name, req.1);
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
