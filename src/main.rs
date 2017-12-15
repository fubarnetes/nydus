#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod network;
use network::node::Node;

fn main() {
    trace!("starting up");
    let node = Node::new();
    println!("UUID: {}",node.uuid);
}