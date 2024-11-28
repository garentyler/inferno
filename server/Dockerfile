FROM rust:1.82.0-alpine3.20 AS base
RUN apk add --no-cache musl-dev=1.2.5-r0 git=2.45.2-r0
RUN cargo install cargo-chef --locked --version 0.1.68
RUN cargo install cargo-watch --locked --version 8.5.3
WORKDIR /app
RUN git config --global --add safe.directory /app

FROM base AS dev
EXPOSE 3001
CMD ["cargo", "watch", "-x", "run"]

FROM base AS planner
COPY server/Cargo.toml .
COPY server/Cargo.lock .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY server .
COPY .git .git
RUN cargo build --release
RUN strip target/release/inferno

FROM alpine:3.20 AS prod
WORKDIR /app
COPY --from=builder /app/target/release/inferno .
EXPOSE 3001
CMD ["/app/inferno"]
