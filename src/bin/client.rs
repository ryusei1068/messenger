use std::net::TcpStream;
use std::io::{self, BufRead};
use messenger;

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
      let request = match messenger::parse_command(&command) {
          Some(request) => request,
          None => continue,
      };
  }

  Ok(())
}


fn main() {
  let _ = send_commands();
}