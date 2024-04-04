use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientRequest {
  Get,
  Join { group_name: String },
  Send { message: String }
}


pub fn parse_command(line: &str) -> Option<ClientRequest> {
  let (command, rest) = get_next_token(line)?;
  if command == "send" {
      let (group, rest) = get_next_token(rest)?;
      let message = rest.trim_start().to_string();
      return Some(ClientRequest::Send { message: message.to_string() });
  } else if command == "join" {
      let (group, rest) = get_next_token(rest)?;
      if !rest.trim_start().is_empty() {
          return None;
      }
      return Some(ClientRequest::Join {
          group_name: group.to_string(),
      });
  } else if command == "get" {
    return Some(ClientRequest::Get);
  }
  else {
      eprintln!("Unrecognized command: {:?}", line);
      return None;
  }
}


fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
  input = input.trim_start();

  if input.is_empty() {
      return None;
  }

  match input.find(char::is_whitespace) {
      Some(space) => Some((&input[0..space], &input[space..])),
      None => Some((input, "")),
  }
}

