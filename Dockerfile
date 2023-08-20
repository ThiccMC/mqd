FROM rust:1.61.0 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mqd /usr/local/bin/mqd
CMD ["mqd"]