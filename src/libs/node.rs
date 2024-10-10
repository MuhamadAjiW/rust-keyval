use std::{
    io::Error,
    net::TcpStream,
    sync::{Arc, RwLock},
};

use crate::libs::net::request::{Request, RequestType};

use super::{
    app::store::Store,
    net::{address::AddressInput, client::Client, server::Server},
};

pub struct Node {
    client: Client,
    server: Server,
    pub store: Arc<RwLock<Store>>,
}

impl Node {
    pub fn new(input: AddressInput, thread_count: usize) -> Self {
        return Node {
            client: Client::new(),
            server: Server::new(input, thread_count),
            store: Arc::new(RwLock::new(Store::new())),
        };
    }

    pub fn print_info(&self) {
        println!("Node info:");
        println!("Address: {}", self.server.address.to_string())
    }

    pub fn run(&self) {
        let store_clone = Arc::clone(&self.store);

        let success_handler = move |stream: TcpStream| {
            println!(
                "Accepted a connection from: {:?}",
                stream.peer_addr().unwrap()
            );

            if let Some(request) = Request::parse(stream) {
                match request.reqtype {
                    RequestType::GET => {
                        let value = store_clone.read().unwrap().get(&request.key);
                        println!("{value}");
                    }
                    RequestType::SET => {
                        store_clone.write().unwrap().set(
                            &request.key.clone(),
                            &request.val.clone().unwrap_or_default(),
                        );
                    }
                    RequestType::REMOVE => {
                        store_clone.write().unwrap().remove(&request.key);
                    }
                }
            } else {
                eprintln!("Bad command received");
            }
        };

        let fail_handler = |e: Error| {
            eprintln!("Failed to accept connection: {}", e);
        };

        self.server.run(success_handler, fail_handler);
    }
}
