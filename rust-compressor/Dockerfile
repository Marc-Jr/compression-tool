FROM rust:1.74-slim as builder


WORKDIR /usr/src/app
COPY . . 

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/rust-compressor /usr/local/bin/rust-compressor

CMD ["rust-compressor"]
