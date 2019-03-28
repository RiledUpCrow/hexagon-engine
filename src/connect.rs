use super::engine::Engine;
use super::parse_message;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::thread;
use tokio::runtime::current_thread::Builder;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

pub fn connect(host: &str, engine: &mut Engine) -> Result<(), WebSocketError> {
    let client = ClientBuilder::new(host)?.add_protocol("rust-websocket");

    println!("Connecting to {}", host);
    let mut runtime = Builder::new().build().unwrap();

    let (usr_msg, stdin_ch) = mpsc::channel(0);
    thread::spawn(move || {
        let mut _sink = usr_msg.wait();
        println!("Thread ready");
    });

    let runner = client.async_connect(None).and_then(|(duplex, _)| {
        let (sink, stream) = duplex.split();
        stream
            .filter_map(|message| match message {
                OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                OwnedMessage::Text(t) => Some(
                    parse_message::parse_message(&t, &engine.identity)
                        .map(|m| OwnedMessage::Text(m))
                        .unwrap_or_else(|err| {
                            dbg!(err);
                            OwnedMessage::Text(String::from("{}"))
                        }),
                ),
                _ => None,
            })
            .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
            .forward(sink)
    });

    runtime.block_on(runner)?;

    Ok(())
}
