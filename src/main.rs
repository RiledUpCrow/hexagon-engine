extern crate serde;
extern crate websocket;

use websocket::client::ClientBuilder;

mod connect;
mod generate_map;
mod message;
mod settings;
mod tile;

const CONNECTION: &'static str = "ws://localhost";

fn main() {
    let mut client = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket");

    loop {
        connect::connect(&mut client);
        println!("Connection dropped, repeating in 5s");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("Reconnecting!");
    }
}
