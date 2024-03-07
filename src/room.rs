use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;

use crate::client::Client;
use crate::Request;

#[derive(Debug)]
pub struct Room {
    udp_socket: UdpSocket,
    clients: HashMap<String, Client>,
}

impl Room {
    pub fn new(udp_socket: UdpSocket, clients: HashMap<String, Client>) -> Self {
        Room {
            udp_socket,
            clients,
        }
    }

    pub fn join(&mut self, name: String, client: Client) {
        self.clients.insert(name, client);
    }

    pub fn broadcast(&self, request: Request) {
        for client in self.clients.values() {
            if client.get_name() == request.get_from() {
                continue;
            }
            self.sender(client.get_udp_socket_addr(), request.get_text().as_bytes());
        }
    }

    pub fn private_msg(&self, request: Request) {
        if let Some(client) = self.get_client_by_name(request.get_to()) {
            self.sender(client.get_udp_socket_addr(), request.get_text().as_bytes());
        }
    }

    pub fn get_client_by_name(&self, name: String) -> Option<&Client> {
        self.clients.get(&name)
    }

    pub fn sender(&self, udp_socket_addr: SocketAddr, byte: &[u8]) {
        match self.udp_socket.send_to(byte, udp_socket_addr) {
            Ok(_) => {
                println!("\nsucceeded");
            }
            Err(e) => {
                println!("\ncloud not send a request.text: {:?}", e);
            }
        }
    }
}
