ARG BUILD_FROM

FROM ghcr.io/home-assistant/amd64-base:latest

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/sfgame-reader
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM ${BUILD_FROM}

COPY --from=builder /usr/src/sfgame-reader/target/release/sfgame-reader /usr/bin/sfgame-reader
COPY run.sh /run.sh

RUN chmod a+x /run.sh /usr/bin/sfgame-reader

CMD ["/run.sh"]
