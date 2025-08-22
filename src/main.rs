use crate::websocket::start_server;

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8000;

mod websocket;
mod handlers;

#[tokio::main]
async fn main() {

    // TODO Config loading logic

    start_server(ADDR, PORT).await;
}