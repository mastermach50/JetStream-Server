use crate::websocket::start_server;

mod websocket;
mod handlers;
mod config_loader;

#[tokio::main]
async fn main() {

    let config = config_loader::get_config();

    start_server(&config.addr, config.port).await;
}