use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let server_address = "127.0.0.1:34254";

    let message = "1Hello, server!";
    socket.send_to(message.as_bytes(), server_address)?;

    let mut buffer = [0; 4096];
    let (amt, _) = socket.recv_from(&mut buffer)?;

    println!("Received: {}", String::from_utf8_lossy(&buffer[..amt]));

    Ok(())
}
