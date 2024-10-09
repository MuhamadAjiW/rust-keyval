use super::address::{Address, AddressInput};

pub struct Server {
    pub address: Address,
}
impl Server {
    pub fn new(input: AddressInput) -> Self {
        let addr: Address;
        match input {
            AddressInput::Address(address) => addr = address,
            AddressInput::IpAndPort(ip, port) => addr = Address::new(&ip, port),
        }
        return Server { address: addr };
    }
}
