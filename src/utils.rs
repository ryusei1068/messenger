use std::net::SocketAddr;

use crate::client::Client;
use crate::method::Method;
use crate::request::Request;

pub fn map_method(method: &String) -> Option<Method> {
    match method.as_str() {
        "1" => Some(Method::Join),
        "2" => Some(Method::Send),
        "3" => Some(Method::PvtMsg),
        _ => None,
    }
}

pub fn parse(buf: &[u8], udp_socket_addr: SocketAddr) -> Result<(Request, Client), String> {
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
