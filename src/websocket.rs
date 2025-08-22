use std::sync::Arc;

use futures_util::lock::Mutex;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use tokio_tungstenite::{accept_async, tungstenite};
use local_ip_address::local_ip;

use crate::handlers::message_handler;

pub async fn start_server(addr: &str, port: u16) {
    // Create a TCP listener
    let listener = TcpListener::bind(format!("{addr}:{port}"))
        .await
        .expect("Failed to bind to {ADDR}:{PORT}");
    println!("Listening on ws://{addr}:{port}");
    println!("Local IP address is {}", local_ip().unwrap());

    // Accept incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        // Accept the connection as a websocket connection
        let ws_stream = accept_async(stream)
            .await
            .expect("Failed to accept websocket connection");
        println!(
            "Accepted websocket connection from {}",
            ws_stream.get_ref().peer_addr().unwrap()
        );

        tokio::spawn(ws_connection_handler(ws_stream));
    }
}

async fn ws_connection_handler(ws_stream: WebSocketStream<TcpStream>) {
    let (outbox, mut inbox) = ws_stream.split();

    let shared_outbox: SharedOutbox = Arc::new(Mutex::new(outbox));

    while let Some(msg) = inbox.next().await {
        match msg {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    println!("> {text}");
                    message_handler::message_handler(&text, &shared_outbox).await;
                }
                Message::Close(close) => {
                    println!("Close: {:?}", close);
                    break;
                }
                _ => {}
            },
            Err(e) => {
                if let tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                ) = e
                {
                    eprintln!("Client disconnected abruptly. No closing handshake received.");
                } else {
                    eprintln!("Failed to receive message: {}", e);
                }
            }
        }
    }
}

pub type SharedOutbox = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>;
pub trait SharedOutboxExt {
    async fn quick_send(&self, msg: &str);
}

impl SharedOutboxExt for SharedOutbox {
    async fn quick_send(&self, msg: &str) {
        self.lock()
            .await
            .send(Message::Text(msg.into()))
            .await
            .expect("Failed to send message");
    }
}
