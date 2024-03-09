use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::Sender;

use crate::client::Client;
use crate::Request;

pub const UDP_SERVER_ADDRESS: &str = "127.0.0.1:8080";
pub const MSG_SIZE: usize = 4096;

#[derive(Debug)]
pub struct UdpServer {
    socket: UdpSocket,
    buf: [u8; MSG_SIZE],
    tx: Sender<(Request, Client)>,
}

impl UdpServer {
    pub fn new(socket: UdpSocket, buf: [u8; MSG_SIZE], tx: Sender<(Request, Client)>) -> Self {
        UdpServer { socket, buf, tx }
    }

    pub fn run(&mut self) {
        loop {
            match self.socket.recv_from(&mut self.buf) {
                Ok((buf_size, udp_socket_addr)) => {
                    let buf = &mut self.buf[..buf_size];
                    if let Ok(parsed_request) = UdpServer::parse(&buf, udp_socket_addr) {
                        let _ = self.tx.send(parsed_request);
                    }
                }
                Err(e) => {
                    println!("couldn't receive request: {:?}", e);
                }
            }
        }
    }

    fn parse(buf: &[u8], udp_socket_addr: SocketAddr) -> Result<(Request, Client), String> {
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
                let from = req.get_from().clone();
                Ok((req, Client::new(from, udp_socket_addr)))
            }
            Err(e) => Err(format!("failed to deserialize {:?}", e)),
        }
    }
}
