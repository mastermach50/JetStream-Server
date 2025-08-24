use crate::{mdns::start_mdns_responder, websocket::start_server};

mod websocket;
mod handlers;
mod config_loader;
mod mdns;

#[tokio::main]
async fn main() {

    let config = config_loader::get_config();

    // Dropping _service unregisters the service
    let _service = start_mdns_responder(config.server_name.clone(), config.port);
    start_server(&config.addr, config.port).await;
}