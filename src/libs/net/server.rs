use std::{
    io::Error,
    net::{TcpListener, TcpStream},
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
        F: Fn(TcpStream) + Send + Copy + 'static,
        G: Fn(Error) + Send + Copy + 'static,
    {
        println!("Server running at {}", self.address);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        success_handler(stream);
                    });
                }
                Err(e) => {
                    thread::spawn(move || {
                        fail_handler(e);
                    });
                }
            }
        }
    }
}
