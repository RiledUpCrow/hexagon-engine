# hexagon engine

This is the engine that will run the logic of the game. The plan is for each player to run this locally or in their own cloud. The engine will then connect to a proxy server so players can reach it from outside of their networks.

To install it you need to have [Rust](https://www.rust-lang.org) on your machine. Just clone the repository and use `cargo run` command inside. For now the engine sets up an HTTP server on port 8080.
