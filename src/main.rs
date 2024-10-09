mod libs;

use crate::libs::store::Store;

fn main() {
    let mut store = Store::new();
    store.set("key", "value");
    let result = store.get("key");

    println!("{result}");
    println!("Works fine!");
}
