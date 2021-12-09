FROM rust:1.57
WORKDIR /usr/app
COPY . .
RUN cargo build --release
CMD ["./target/release/service"]
