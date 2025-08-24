use libmdns::{Responder, Service};

pub fn start_mdns_responder(server_name: String, port: u16) -> Service {
    let responder = Responder::new().expect("Failed to start mDNS responder");
    let service = responder.register(
        "_jetstream._tcp".to_string(),
        server_name,
        port,
        &[]
    );
    println!("Started mDNS responder");

    return service;
}