FROM rust:latest as builder
WORKDIR /tmp/build
COPY ./ ./
RUN cargo build --release

FROM rust:latest
WORKDIR /opt/discord-embed-links
COPY --from=builder /tmp/build/target/release/discord-embed-links .
ENTRYPOINT ./discord-embed-links