FROM rust:1.74-slim AS builder

WORKDIR /usr/src/app

# Copy only the necessary files for building
COPY rust-compressor/Cargo.toml rust-compressor/Cargo.lock ./
COPY rust-compressor/src ./src

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/rust-compressor .

CMD ["./rust-compressor"]
