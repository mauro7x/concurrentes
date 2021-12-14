FROM rust:1.57
WORKDIR /usr/app

# Weird hacky mechanism for avoiding re-build dependencies everytime
# (Idea from: https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build)
RUN echo "fn main() {}" > dummy.rs
COPY dummy.Cargo.toml Cargo.toml
RUN cargo build --release
RUN rm dummy.rs

COPY . .
COPY ./shared/alglobo_directory_protocol.rs ./src/lib/protocol/directory.rs
COPY ./shared/alglobo_generic-service_protocol.rs ./src/lib/protocol/data.rs
RUN cargo build --release --bin manual
CMD ["target/release/manual"]
