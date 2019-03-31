extern crate futures;
extern crate serde;
extern crate tokio;
extern crate websocket;

use engine::Engine;
use std::{env, path::PathBuf, str::FromStr};

mod connect;
mod engine;
mod game;
mod game_manager;
mod generate_map;
mod identity;
mod message;
mod parse_message;
mod settings;
mod tile;

const VERSION: &str = "0.1.0";

fn main() {
    println!("Starting Engine version {}", VERSION);

    let data_url = env::var("TWA_DATA").unwrap_or(String::from("./engine-data"));
    let host = env::var("TWA_HOST").unwrap_or(String::from("wss://theworldanew.com/socket"));

    let path = PathBuf::from_str(&data_url).unwrap();
    let mut engine = Engine::new(VERSION, &path).unwrap();

    loop {
        let _ = connect::connect(&host, &mut engine);
        println!("Connection dropped, repeating in 5s");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
