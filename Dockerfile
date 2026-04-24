FROM rust:1.95.0-bookworm

WORKDIR /workspace

RUN rustup component add clippy rustfmt

COPY . .

RUN cargo fetch --locked

CMD ["./container/run-rust-verification.sh"]
