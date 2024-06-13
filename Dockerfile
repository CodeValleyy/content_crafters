FROM rust:1.77-slim-bullseye
RUN apt-get update && apt-get install -y libssl-dev pkg-config 
WORKDIR /usr/src/content_crafters_api
COPY . .
RUN cargo build --release
CMD ["./target/release/api"]