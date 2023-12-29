use core::str::from_utf8;
use std::net::UdpSocket;
use std::thread;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    let mut buf = [0; 4096];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    let buf = &mut buf[..amt];
                    let req_msg = from_utf8(&buf).unwrap();
                    println!("{:}", "=".repeat(80));
                    println!("buffer size: {:?}", amt);
                    println!("src address: {:?}", &src);
                    println!("request message: {:?}", req_msg);
                });
            }
            Err(e) => {
                println!("couldn't recieve request: {:?}", e);
            }
        }
    }
}
