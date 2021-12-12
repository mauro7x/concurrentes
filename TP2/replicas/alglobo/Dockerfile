# Dockerfile to avoid re-building everytime
# Idea from https://www.lpalmieri.com/posts/fast-rust-docker-builds/

# ---

# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 AS chef
WORKDIR /usr/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /usr/app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin app

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR /usr/app
COPY --from=builder /usr/app/target/release/app /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]