FROM rust:1.56
WORKDIR /usr/app/
COPY ./ ./
RUN cargo build --release --bin part2
CMD ["./target/release/part2"]
