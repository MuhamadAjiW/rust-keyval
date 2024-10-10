use std::{
    io::{Error, Write},
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

        let success_handler = move |mut stream: TcpStream| {
            println!(
                "Accepted a connection from: {:?}",
                stream.peer_addr().unwrap()
            );

            loop {
                let request = Request::parse(&mut stream);
                if request.is_none() {
                    println!("Connection ended: {:?}", stream.peer_addr().unwrap());
                    break;
                }

                let request = request.unwrap();
                let response: String;

                match request.reqtype {
                    RequestType::BAD => {
                        response = format!("Bad command received\n");
                    }
                    RequestType::GET => {
                        response = format!("{}\n", store_clone.read().unwrap().get(&request.key));
                    }
                    RequestType::SET => {
                        store_clone
                            .write()
                            .unwrap()
                            .set(&request.key.clone(), &request.val.clone());
                        response = "OK\n".to_string();
                    }
                    RequestType::REMOVE => {
                        store_clone.write().unwrap().remove(&request.key);
                        response = "OK\n".to_string();
                    }
                    RequestType::TERMINATE => {
                        println!(
                            "Termination command received: {:?}",
                            stream.peer_addr().unwrap()
                        );
                        response = "Termination command received...\n".to_string();
                    }
                }

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Error received: {}", e);
                    break;
                }

                if request.reqtype == RequestType::TERMINATE {
                    return;
                }
            }
        };

        let fail_handler = |e: Error| {
            eprintln!("Failed to accept connection: {}", e);
        };

        self.server.run(success_handler, fail_handler);
    }
}
