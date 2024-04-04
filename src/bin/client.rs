use std::net::TcpStream;
use std::io::{self, BufRead, Write};
use messenger::{self, ClientRequest};

const TCP_SERVER_ADDRESS: &str = "127.0.0.1:8081";

fn send_commands() -> io::Result<()> {
  println!("Commands:\n\
            get\n\
            join GROUP\n\
            send GROUP MESSAGE...\n\
            Type Control-D (on Unix) or Control-Z (on Windows) \
            to close the connection.");

  let mut command_lines = io::BufReader::new(io::stdin()).lines();
  while let Some(command_result) = command_lines.next() {
      let command = command_result?;
      let _ = match messenger::parse_command(&command) {
          Some(request) => send_as_json(request),
          None => continue,
      };
  }

  Ok(())
}

fn send_as_json(packet: ClientRequest) -> Result<String, String> {
  let mut socket = match TcpStream::connect(TCP_SERVER_ADDRESS) {
    Ok(socket) => socket,
    Err(_) => return Err("failed to connect server".to_string())
  };

  let mut json = serde_json::to_string(&packet).unwrap();
  json.push('\n');

  match socket.write_all(json.as_bytes()) {
    Ok(_) => {
      Ok("Successfully send.".to_string())
    } Err(_) => {
      Err("Could not send a packet.".to_string())
    }
  }
}

fn main() {
  let _ = send_commands();
}