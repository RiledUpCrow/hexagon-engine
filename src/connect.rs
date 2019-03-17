use super::message::{message::Message as EngineMessage, register_data::RegisterData};
use std::sync::mpsc::channel;
use std::thread;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

pub fn connect(client_builder: &mut ClientBuilder) {
    println!("Connecting");

    let client = client_builder.connect_insecure().unwrap();

    println!("Successfully connected");

    let (mut receiver, mut sender) = client.split().unwrap();

    let (tx, rx) = channel();

    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let receive_loop = thread::spawn(move || {
        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop Error With Close: {:?}", e);
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_1.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop Error: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                _ => println!("Receive Loop Message: {:?}", message),
            }
        }
    });

    // register the server
    {
        let message = EngineMessage::Register(RegisterData {
            id: String::from("hehe"),
            admin_token: String::from("hoho"),
            version: String::from("0.1.0"),
        });
        let text = serde_json::to_string(&message).unwrap();
        let _ = tx.send(OwnedMessage::Text(text));
    }

    let _ = receive_loop.join();
    let _ = send_loop.join();

    // We're exiting
    println!("Exited");
}
