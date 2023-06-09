use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{
    accept_async,
};

async fn accept_connection(stream: TcpStream, _peer: SocketAddr) {
    let websocket_stream = accept_async(stream)
        .await
        .expect("Could not accept connection");
    let (mut write, mut read) = websocket_stream.split();
    while let Some(msg) = read.next().await {
        let data = msg.unwrap().into_text();
        let text_data = data.unwrap();
        println!("{}", text_data.clone());
        let ok_message = Message::Text(String::from(text_data.clone()));
        write.send(ok_message).await.expect("Could not send");
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // Start a listener
    let listener = TcpListener::bind("0.0.0.0:4000").await.expect("");

    // Handle the messages
    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, peer));
    }
}
