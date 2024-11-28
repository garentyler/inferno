FROM rust:1.82.0-alpine3.20
RUN apk add --no-cache musl-dev=1.2.5-r0 git=2.45.2-r0
RUN cargo install cargo-watch --locked --version 8.5.3
WORKDIR /app
RUN git config --global --add safe.directory /app
EXPOSE 3001
CMD ["cargo", "watch", "-x", "run"]