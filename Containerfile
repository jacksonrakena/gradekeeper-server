# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin gk-server

FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/gk-server /usr/local/bin/
RUN apt-get update && apt install -y openssl libpq-dev ca-certificates
CMD ["/usr/local/bin/gk-server"]