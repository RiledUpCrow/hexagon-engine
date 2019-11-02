FROM rust:1.38 as build
WORKDIR /var/hexagon

COPY Cargo.toml Cargo.lock ./
COPY src src/
RUN cargo build --release

# production build
FROM rust:1.38
WORKDIR /var/hexagon

COPY --from=build /var/hexagon/target/release/hexagon-engine ./

CMD [ "./hexagon-engine" ]
