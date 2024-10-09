mod libs;
use libs::{
    net::address::{Address, AddressInput},
    node::Node,
};

fn main() {
    let key = "Key";
    let val = "Value";
    let addr = Address::new("127.0.0.1", 8080);

    let mut node = Node::new(AddressInput::Address(addr));
    node.store.set(key, val);

    println!("Keyvalue store test: {} -> {}", key, node.store.get(key));
    node.print_info();

    node.run();

    println!("Works fine!");
}
