FROM rust:1.76-bookworm as builder

WORKDIR /build
ADD . .
RUN cargo build --release

FROM debian:bookworm 

WORKDIR /opt
RUN apt update && apt install -y libssl-dev ca-certificates
COPY --from=builder /build/target/release/indexer .
COPY --from=builder /build/target/release/scanner .
COPY --from=builder /build/target/release/updater .
