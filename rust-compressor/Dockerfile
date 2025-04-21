FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/rust-compressor /usr/local/bin/rust-compressor

CMD ["rust-compressor"]
