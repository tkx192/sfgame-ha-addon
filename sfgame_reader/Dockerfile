FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/sfgame-reader

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src

RUN cargo build --release

FROM ghcr.io/home-assistant/base:latest

COPY --from=builder /usr/src/sfgame-reader/target/release/sfgame-reader /usr/bin/sfgame-reader
COPY run.sh /run.sh

RUN chmod +x /run.sh

CMD ["/run.sh"]
