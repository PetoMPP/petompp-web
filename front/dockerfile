FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked
VOLUME /front
WORKDIR /front
COPY . .
EXPOSE 8080
CMD trunk serve --release --address 0.0.0.0