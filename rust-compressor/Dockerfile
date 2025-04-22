FROM rust:1.74-slim AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/src/app/target/release/rust-compressor /usr/local/bin/rust-compressor

ENTRYPOINT ["/usr/local/bin/rust-compressor"]
