extern crate futures;
extern crate serde;
extern crate tokio;
extern crate websocket;

use engine::Engine;
use std::env;

mod connect;
mod engine;
mod game_manager;
mod generate_map;
mod identity;
mod message;
mod parse_message;
mod settings;
mod tile;

fn main() {
    let host = env::var("TWA_HOST").unwrap_or("wss://theworldanew.com/socket".to_owned());
    let mut engine = Engine::new().unwrap();
    loop {
        let _ = connect::connect(&host, &mut engine);
        println!("Connection dropped, repeating in 5s");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
