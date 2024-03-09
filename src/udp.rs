use std::net::UdpSocket;
use std::sync::mpsc::Sender;

use crate::client::Client;
use crate::utils;
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
                    if let Ok(parsed_request) = utils::parse(&buf, udp_socket_addr) {
                        let _ = self.tx.send(parsed_request);
                    }
                }
                Err(e) => {
                    println!("couldn't receive request: {:?}", e);
                }
            }
        }
    }
}
