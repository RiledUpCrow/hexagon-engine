use super::message::{message::Message as EngineMessage, register_data::RegisterData};
use super::parse_message;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::thread;
use tokio::runtime::current_thread::Builder;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

pub fn connect(host: &str) -> Result<(), WebSocketError> {
    let client = ClientBuilder::new(host)?.add_protocol("rust-websocket");

    println!("Connecting");
    let mut runtime = Builder::new().build().unwrap();

    let (usr_msg, stdin_ch) = mpsc::channel(0);
    thread::spawn(move || {
        let mut sink = usr_msg.wait();

        // register the server
        let message = EngineMessage::Register(RegisterData {
            id: String::from("hehe"),
            admin_token: String::from("hoho"),
            version: String::from("0.1.0"),
        });
        let text = serde_json::to_string(&message).unwrap();
        let om = OwnedMessage::Text(text);
        let _ = sink.send(om);
        println!("Register sent");
    });

    let runner = client.async_connect(None).and_then(|(duplex, _)| {
        let (sink, stream) = duplex.split();
        stream
            .filter_map(|message| {
                println!("Received Message: {:?}", message);
                match message {
                    OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                    OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                    OwnedMessage::Text(t) => parse_message::parse_message(&t)
                        .map(|m| OwnedMessage::Text(m))
                        .ok(),
                    _ => None,
                }
            })
            .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
            .forward(sink)
    });

    runtime.block_on(runner)?;

    // We're exiting
    println!("Exited");

    Ok(())
}
