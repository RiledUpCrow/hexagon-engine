FROM rust:1.33 as base

# rustfmt for checking style
RUN rustup component add rustfmt

# new project so we can cache dependencies
RUN USER=root cargo new --bin hexagon-engine
WORKDIR /hexagon-engine
COPY Cargo.toml Cargo.lock ./

# development image
FROM base as dev

# this will restart the container every time source changes
RUN cargo install cargo-watch

# build dev target
RUN cargo build

CMD [ "cargo", "watch", "-x", "run" ]

FROM base as build

# compiling dependencies for caching
RUN cargo build --release

# compiling actual source code
COPY src src/
RUN cargo build --release

# production build
FROM rust:1.33
WORKDIR /var/hexagon

COPY --from=build /hexagon-engine/target/release/hexagon-engine ./

CMD [ "./hexagon-engine" ]