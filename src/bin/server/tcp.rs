use std::io::BufReader;
use std::{
    collections::HashMap,
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

const TCP_SERVER_ADDRESS: &str = "127.0.0.1:8081";
pub const MSG_SIZE: usize = 4096;

use messenger::ClientRequest;

use crate::client::{self, Client};

#[derive(Debug)]
pub struct TcpServer {}

impl TcpServer {
    pub fn run(room: Arc<Mutex<HashMap<String, Client>>>) {
        let listener = TcpListener::bind(TCP_SERVER_ADDRESS).expect("Failed to bind");
        println!("Listen TCP");

        for stream in listener.incoming() {
            match stream {
                Ok(socket) => {
                    println!("connection established {:?}", socket);
                    let room_clone = Arc::clone(&room);

                    thread::spawn(move || {
                        Self::serve(socket, room_clone);
                    });
                }
                Err(_) => {
                    println!("error");
                }
            }
        }
    }

    fn serve(socket: TcpStream, room: Arc<Mutex<HashMap<String, Client>>>) {
        let buffered = BufReader::new(socket);
        let client_request = Self::receive_as_json(buffered);

        if let Some(req) = client_request {
            match req {
                ClientRequest::Get => {
                    let room_names = room.lock().unwrap().keys();
                    println!("Get")
                }
                ClientRequest::Join { group_name } => {
                    println!("Join {}", group_name)
                }
                ClientRequest::Send { message } => {
                    println!("Send {}", message)
                }
            }
        }
    }

    fn receive_as_json(mut buffered: BufReader<TcpStream>) -> Option<messenger::ClientRequest> {
        let mut buf = Vec::new();
        let from_client = match buffered.read_to_end(&mut buf) {
            Ok(_) => Some(buf),
            Err(_) => None,
        };

        if let Some(request_byte) = from_client {
            match serde_json::from_str::<messenger::ClientRequest>(&String::from_utf8_lossy(
                &request_byte,
            )) {
                Ok(req) => Some(req),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
