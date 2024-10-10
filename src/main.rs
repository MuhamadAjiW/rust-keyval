mod libs;
use libs::{
    net::address::{Address, AddressInput},
    node::Node,
};

fn main() {
    let addr = Address::new("127.0.0.1", 8080);
    let thread_count = 4;

    let node = Node::new(AddressInput::Address(addr), thread_count);
    node.print_info();
    node.run();

    println!("Works fine!");
}
