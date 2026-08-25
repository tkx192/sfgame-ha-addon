FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/sfgame-reader

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo fetch
RUN cargo build --release

FROM ghcr.io/home-assistant/base:latest

COPY --from=builder /usr/src/sfgame-reader/target/release/sfgame-reader /usr/bin/sfgame-reader
COPY run.sh /run.sh

RUN chmod +x /run.sh

CMD ["/run.sh"]
