FROM rust:slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
FROM debian:trixie-slim as runner
RUN apt-get update -y
RUN apt-get install -y cronie
RUN rm -rf /var/lib/apt/lists/*
RUN mkdir -p /var/log
COPY crontab /etc/crontab
COPY scripts/ /usr/sbin/
# RUN crontab /etc/crontab
COPY --from=builder /usr/local/cargo/bin/mqd /usr/local/bin/mqd
CMD crond -f & mqd