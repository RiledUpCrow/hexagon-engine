extern crate futures;
extern crate serde;
extern crate tokio;
extern crate websocket;

use std::env;

mod connect;
mod generate_map;
mod message;
mod settings;
mod tile;
mod parse_message;

fn main() {
    let host = env::var("TWA_HOST").unwrap_or("wss://theworldanew.com/socket".to_owned());
    loop {
        let _ = connect::connect(&host);
        println!("Connection dropped, repeating in 5s");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
