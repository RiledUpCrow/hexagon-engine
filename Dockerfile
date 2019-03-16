FROM rust:1.33 as build

# rustfmt for checking style
RUN rustup component add rustfmt

# new project so we can cache dependencies
RUN USER=root cargo new --bin hexagon-engine
WORKDIR /hexagon-engine

# compiling dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/hexagon_engine*

# compiling actual source code
COPY src src/
RUN cargo build --release

# production build
FROM rust:1.33
WORKDIR /var/hexagon

COPY --from=build /hexagon-engine/target/release/hexagon-engine ./

CMD [ "./hexagon-engine" ]