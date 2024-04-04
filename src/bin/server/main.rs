use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

mod method;
mod request;
use request::Request;
mod client;
mod room;
use client::Client;
mod udp;
use udp::UdpServer;
mod tcp;
use tcp::TcpServer;

fn main() {
    let udp_socket = UdpSocket::bind(udp::UDP_SERVER_ADDRESS).expect("could not bind UdpSocket");

    let room = Arc::new(Mutex::new(HashMap::<String, Client>::new()));

    let room_clone = Arc::clone(&room);
    thread::spawn(move || {
        TcpServer::run(room_clone);
    });

    UdpServer::run(room);
}
