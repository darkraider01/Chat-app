use tokio:: {
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::broadcast::{self, Sender, Receiver},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
///this is an attribute instructing the compiler to auto generate implementation for the 4 traits

struct ChatMessage{
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType{
    UserMessage,
    SystemNotification,

}

//Tokio is an async runtime for rust language 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //bind the server to a specified IP and port
    let listener = TcpListener::bind("127.0.0.1:8082").await?;

    //Display server startup Message with formatting
    println!(r#"
 ____  _____ ____  _____  ___  ____    _
|  _ \| ____/ ___||_   _||_ _||  _ \  / \
| |_) |  _| \___ \  | |   | | | | | |/ _ \
|  _ <| |___ ___) | | |   | | | |_| / ___ \
|_| \_\_____|____/  |_|  |___||____/_/   \_\
"#);
    println!("Server listening on 127.0.0.1:8082");

    //Create a broadcast channel for message distribution
    let (tx, _) = broadcast::channel::<String>(100);
    loop {
        //Accepting the connection
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        //Display the connection info
        println!("[{}] New Connection", Local::now().format("%H:%M:%S"));
        println!("Address: {}", addr);
        //Clone sender for this connection and subsscribe a receiever
        let tx: Sender<String> = tx.clone();
        let rx: Receiver<String> = tx.subscribe();

        tokio::spawn(async move {
            // The handle_connection function is called here to manage the new connection.
            // We pass the socket and the broadcast channel sender/receiver.
            handle_connection(socket, tx, rx).await;
        });
    }
    //Function to handle the client connection
    async fn handle_connection(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>, 
    mut rx: broadcast::Receiver<String>, 
) {
    //Splitting the socket into a reader and a writer half
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut username: String = String::new();
    // read the username sent by the client
    reader.read_line(&mut username).await.unwrap();
    let username = username.trim().to_string();

    //Send a system notification indication the user has joined the chat
    let join_msg: ChatMessage = ChatMessage { username: username.clone(), content: "joined the chat".to_string(), timestamp: Local::now().format("%H:%M:%S").to_string(), message_type: MessageType::SystemNotification };
    let join_json = serde_json::to_string(&join_msg).unwrap();
    tx.send(join_json).unwrap();

    //initialise a buffer for incoming messages from the client
    let mut line: String = String::new();
    loop{
        tokio::select!{
            result = reader.read_line(&mut line) => {
                if result.unwrap()== 0 {
                    break;
                }
                //Create and broadcast user message
                let msg = ChatMessage{
                    username: username.clone(),
                    content: line.trim().to_string(),
                    timestamp: Local::now().format("%H:%M:%S").to_string(),
                    message_type: MessageType::UserMessage,
                };
                let json = serde_json::to_string(&msg).unwrap();
                tx.send(json).unwrap();
                line.clear();
            }
            //Handle the incoming broadcasts and sends them to the client
            result = rx.recv() => {
                let msg = result.unwrap();
                // We must write the message and a newline for the client's `read_line` to work.
                writer.write_all(msg.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
            }
            
        }
    }
}

}
