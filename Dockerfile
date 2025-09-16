FROM rust:1.82 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rustinance /usr/local/bin/app

CMD [ "app" ]