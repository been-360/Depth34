
use tokio::net::{TcpStream};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};

// TODO: Complete reflect websockets

pub async fn handle_reflect(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text("()".into())).await;
}