use super::{
    app::store::Store,
    net::{
        address::{self, Address, AddressInput},
        client::Client,
        server::Server,
    },
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
}
