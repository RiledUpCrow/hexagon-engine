FROM rust:1.38

# rustfmt for checking style
RUN rustup component add rustfmt

# this will restart the container every time source changes
RUN cargo install cargo-watch

# new project so we can cache dependencies
RUN USER=root cargo new --bin hexagon-engine
WORKDIR /hexagon-engine
COPY Cargo.toml Cargo.lock ./

# build dev target
RUN cargo build

RUN rm src/* target/debug/hexagon-engine* target/debug/deps/hexagon_engine*

CMD [ "cargo", "watch", "-x", "run" ]
