mod libs;
use libs::{
    net::address::{Address, AddressInput},
    node::Node,
};

fn main() {
    let addr = Address::new("127.0.0.1", 8080);
    let node = Node::new(AddressInput::Address(addr));
    node.print_info();
    node.run();

    println!("Works fine!");
}
