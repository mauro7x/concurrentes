FROM rust:1.57
WORKDIR /usr/app

# Weird hacky mechanism for avoiding re-build dependencies everytime
# (Idea from: https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build)
RUN echo "fn main() {}" > dummy.rs
COPY dummy.Cargo.toml Cargo.toml
RUN cargo build --release
RUN rm dummy.rs

COPY . .
RUN cargo build --release
CMD ["target/release/app"]
