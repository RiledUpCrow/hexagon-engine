extern crate hyper;
extern crate serde;

use generate_map::generate;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use settings::Settings;

mod generate_map;
mod settings;
mod tile;

const SETTINGS: Settings = Settings {
    width: 128,
    height: 80,
};

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let handler = |_req: Request<Body>| -> Result<Response<Body>, hyper::Error> {
        let tiles = generate(&SETTINGS);
        let json = serde_json::to_string(&tiles).unwrap();
        Ok(Response::new(Body::from(json)))
    };

    let server = Server::bind(&addr)
        .serve(move || service_fn(handler))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
