extern crate hyper;
extern crate serde;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

mod generate_map;
mod settings;
mod tile;

use settings::Settings;

const SETTINGS: Settings = Settings {
    width: 128,
    height: 80,
};

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let handler = |_req: Request<Body>| -> Result<Response<Body>, hyper::Error> {
        let tiles = generate_map::log(&SETTINGS);
        let json = serde_json::to_string(&tiles).unwrap();
        Ok(Response::new(Body::from(json)))
    };

    let server = Server::bind(&addr)
        .serve(move || service_fn(handler))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
