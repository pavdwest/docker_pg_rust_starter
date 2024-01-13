# FROM rust:1-slim-bullseye as builder
# WORKDIR /backend/app
# COPY ./app /backend/app
# RUN cargo install --path /backend/app

# FROM debian:bullseye-slim as runner
# RUN apt-get update && \
#     apt-get dist-upgrade -y && \
#     apt-get install -y musl-tools
# COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app


FROM rust:1-slim-bullseye
WORKDIR /app
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools
RUN cargo install cargo-watch
COPY ./ ./
