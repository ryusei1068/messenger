use std::net::{SocketAddr, UdpSocket};
use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};

use crate::client::Client;
use crate::Request;

pub const UDP_SERVER_ADDRESS: &str = "127.0.0.1:8080";
pub const MSG_SIZE: usize = 4096;

#[derive(Debug)]
pub struct UdpServer {}

impl UdpServer {
    pub fn run(room: Arc<Mutex<HashMap<String, Client>>>) {
        let udp_socket = UdpSocket::bind(UDP_SERVER_ADDRESS).expect("could not bind UdpSocket");
        let mut buf = [0; MSG_SIZE];

        println!("Listen UDP");
        loop {
            match udp_socket.recv_from(&mut buf) {
                Ok((buf_size, udp_socket_addr)) => {
                    let buf = &mut buf[..buf_size];
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
