use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

mod method;
use method::Method;
mod request;
use request::Request;
mod room;
use room::Room;
mod client;
use client::Client;
mod udp;
use udp::UdpServer;
mod utils;

fn main() {
    let udp_socket = UdpSocket::bind(udp::UDP_SERVER_ADDRESS).expect("could not bind UdpSocket");
    let udp_socket_clone = udp_socket
        .try_clone()
        .expect("could not clone of UdpSocket");

    let (tx, rx) = mpsc::channel::<(Request, Client)>();

    let mut room = Room::new(udp_socket, HashMap::new());

    thread::spawn(move || {
        while let Ok(req) = rx.recv() {
            if let Some(ref method) = utils::map_method(&req.0.get_method()) {
                match method {
                    Method::Join => {
                        println!("\njoin: {:?}", req.1);
                        let name = req.1.get_name();
                        room.join(name, req.1);
                    }
                    Method::Send => {
                        println!("\nsend: {:?}", req.0);
                        room.broadcast(req.0);
                    }
                    Method::PvtMsg => {
                        println!("\nsend: {:?}", req.0);
                        room.private_msg(req.0);
                    }
                }
            }
        }
    });

    let buf = [0; udp::MSG_SIZE];
    let mut udp_server = UdpServer::new(udp_socket_clone, buf, tx);
    udp_server.run();
}
