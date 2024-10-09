use std::{io::Error, net::TcpStream};

use super::{
    app::store::Store,
    net::{address::AddressInput, client::Client, server::Server},
};

pub struct Node {
    client: Client,
    server: Server,
    pub store: Store,
}

impl Node {
    pub fn new(input: AddressInput) -> Self {
        return Node {
            client: Client::new(),
            server: Server::new(input),
            store: Store::new(),
        };
    }

    pub fn print_info(&self) {
        println!("Node info:");
        println!("Address: {}", self.server.address.to_string())
    }

    pub fn run(&self) {
        fn success_handler(stream: TcpStream) {
            println!(
                "Accepted a connection from: {:?}",
                stream.peer_addr().unwrap()
            );
        }
        fn fail_handler(e: Error) {
            eprintln!("Failed to accept connection: {}", e);
        }

        self.server.run(success_handler, fail_handler);
    }
}
