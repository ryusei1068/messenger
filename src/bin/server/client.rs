use std::net::SocketAddr;

#[derive(Debug)]
pub struct Client {
    name: String,
    udp_socket_addr: SocketAddr,
}

impl Client {
    pub fn new(name: String, udp_socket_addr: SocketAddr) -> Self {
        Client {
            name,
            udp_socket_addr,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_udp_socket_addr(&self) -> SocketAddr {
        self.udp_socket_addr
    }
}
