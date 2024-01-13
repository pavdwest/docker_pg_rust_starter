FROM rust:1-slim-bullseye
WORKDIR /app
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    apt-get install -y libpq-dev
RUN cargo install cargo-watch && \
    cargo install diesel_cli --no-default-features --features postgres
COPY ./app ./
