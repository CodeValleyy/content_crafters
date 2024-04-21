FROM rust:1.77
WORKDIR /usr/src/content_crafters
COPY . .
RUN cargo build --release
CMD ["./target/release/api"]