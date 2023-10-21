use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("new client: {:?}", addr);

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {

    //      while let Some(frame) = connection.read_frame().await.unwrap() {
    //         let response = match Command::from_frame(frame).unwrap() {
    //             Set(cmd) => {
    //                 let mut db = db.lock().unwrap();
    //                 db.insert(cmd.key().to_string(), cmd.value().to_vec().into());
    //                 Frame::Simple("OK".to_string())
    //             }
    //             Get(cmd) => {
    //                 let mut db = db.lock().unwrap();
    //                 if let Some(value) = db.get(cmd.key()) {
    //                     Frame::Bulk(value.clone().into())
    //                 } else {
    //                     Frame::Null
    //                 }
    //             }
    //             cmd => panic!("unimplemented {:?}", cmd),
    //         };
    //
    //         connection.write_frame(&response).await.unwrap();
    //     }
}
