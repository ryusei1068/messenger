use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use std::{str, usize};

const SERVER_ADDRESS: &str = "127.0.0.1:34254";
const MSG_SIZE: usize = 4096;

enum ChannelMessage {
    Join(Client),
    Send(UserMessage),
}

struct UserMessage {
    name: String,
    message: String,
}

#[derive(Debug)]
pub struct Client {
    username: String,
    last_send: SystemTime,
    src: SocketAddr,
}

impl Client {
    fn new(username: String, src: SocketAddr) -> Client {
        Client {
            username,
            last_send: SystemTime::now(),
            src,
        }
    }
}

struct ChatRoom {
    clients: HashMap<String, Client>,
}

impl ChatRoom {
    fn new() -> ChatRoom {
        ChatRoom {
            clients: HashMap::new(),
        }
    }

    fn join(&mut self, client: Client) {
        self.clients.insert(client.username.to_string(), client);
    }

    fn bloadcast(&mut self, buf: &[u8], socket: UdpSocket, sender_name: String) {
        for (username, client) in self.clients.iter() {
            if *username == sender_name {
                continue;
            }
            match socket.send_to(buf, &client.src) {
                Ok(_) => {
                    println!("Message sent successfully.");
                }
                Err(e) => {
                    println!("Failed to send message {:?}", e);
                }
            }
        }
    }

    fn update_last_send(&mut self, username: String) {
        match self.clients.get_mut(&username) {
            Some(client) => {
                client.last_send = SystemTime::now();
                println!("updated client: {:?} ", client);
            }
            None => println!("Not found client: {:?}", username),
        };
    }

    fn is_active(&self, client: Client) -> bool {
        match SystemTime::now().duration_since(client.last_send) {
            Ok(duration) => {
                if duration >= Duration::from_secs(10 * 60) {
                    true
                } else {
                    false
                }
            }
            Err(_) => true,
        }
    }
}

struct Inbound {
    socket: UdpSocket,
    sender: Sender<ChannelMessage>,
}

impl Inbound {
    fn new(socket: UdpSocket, sender: Sender<ChannelMessage>) -> Inbound {
        Inbound { socket, sender }
    }

    fn receive_message(&self) {
        loop {
            let mut buf = [0; MSG_SIZE];

            match self.socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    // 1 join to chat_room (urf8 49 = 1)
                    // 2 send message (utf8 50 = 2)
                    let cmd = buf[0];
                    let mut buf_clone = buf.clone();

                    let username_bytes = &mut buf_clone[1..9];
                    let username = match str::from_utf8(username_bytes) {
                        Ok(name) => name,
                        Err(e) => {
                            println!("Failed to convert to &str {:?}", e);
                            ""
                        }
                    };

                    if username.is_empty() {
                        continue;
                    }

                    if cmd == 49 {
                        if self
                            .sender
                            .send(ChannelMessage::Join(Client::new(username.to_string(), src)))
                            .is_err()
                        {
                            println!("failed to send to the channel");
                        }
                    } else if cmd == 50 {
                        let message_bytes = &mut buf[9..amt];
                        let msg = match str::from_utf8(message_bytes) {
                            Ok(msg) => msg,
                            Err(e) => {
                                println!("Failed to convert to &str {:?}", e);
                                ""
                            }
                        };

                        if msg.is_empty() {
                            continue;
                        }

                        if self
                            .sender
                            .send(ChannelMessage::Send(UserMessage {
                                message: msg.to_string(),
                                name: username.to_string(),
                            }))
                            .is_err()
                        {
                            println!("failed to send to the channel");
                        }
                    }

                    println!("{:}", "=".repeat(80));
                    println!("buffer size: {:?}", amt);
                    println!("src address: {:?}", &src);
                    println!("{:}", "=".repeat(80));
                }
                Err(e) => {
                    println!("couldn't recieve request: {:?}", e);
                }
            }
        }
    }
}

struct EventsHander {
    receiver: Receiver<ChannelMessage>,
    chat_room: ChatRoom,
    socket: UdpSocket,
}

impl EventsHander {
    fn new(
        receiver: Receiver<ChannelMessage>,
        chat_room: ChatRoom,
        socket: UdpSocket,
    ) -> EventsHander {
        EventsHander {
            receiver,
            chat_room,
            socket,
        }
    }

    fn process_events(&mut self) {
        while let Ok(message) = self.receiver.recv() {
            match message {
                ChannelMessage::Join(client) => {
                    self.chat_room.join(client);
                }
                ChannelMessage::Send(user_msg) => {
                    if let Ok(clone_socket) = self.socket.try_clone() {
                        self.chat_room.bloadcast(
                            user_msg.message.as_bytes(),
                            clone_socket,
                            user_msg.name,
                        );
                    } else {
                        println!("Failed to clone udp socket.");
                    }
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(SERVER_ADDRESS)?;
    let chat_room = ChatRoom::new();

    let (sender, receiver) = mpsc::channel::<ChannelMessage>();
    let inbound = Inbound::new(
        socket.try_clone().expect("Failed to clone udp socket"),
        sender,
    );
    let mut events_hander = EventsHander::new(receiver, chat_room, socket);

    thread::spawn(move || {
        events_hander.process_events();
    });

    inbound.receive_message();

    Ok(())
}
