use std::{
    io::Error,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use super::address::{Address, AddressInput};

pub struct Server {
    pub address: Address,
    pub listener: TcpListener,
}
impl Server {
    pub fn new(input: AddressInput) -> Self {
        let address: Address;
        match input {
            AddressInput::Address(addr) => address = addr,
            AddressInput::IpAndPort(ip, port) => address = Address::new(&ip, port),
        }

        let listener = TcpListener::bind((address.ip.as_str(), address.port))
            .expect("Failed to bind to address");

        return Server { address, listener };
    }

    pub fn run<F, G>(&self, success_handler: F, fail_handler: G)
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
        G: Fn(Error) + Send + Sync + 'static,
    {
        println!("Server running at {}", self.address);

        let success_handler = std::sync::Arc::new(success_handler);
        let fail_handler = std::sync::Arc::new(fail_handler);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler = Arc::clone(&success_handler);
                    thread::spawn(move || {
                        handler(stream);
                    });
                }
                Err(e) => {
                    let handler = Arc::clone(&fail_handler);
                    thread::spawn(move || {
                        handler(e);
                    });
                }
            }
        }
    }
}
