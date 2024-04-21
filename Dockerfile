FROM rust:1.77
WORKDIR /usr/src/content_crafters_api
COPY . .
RUN cargo build --release
CMD ["./target/release/content_crafters"]