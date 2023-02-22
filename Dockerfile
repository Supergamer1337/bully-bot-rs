FROM rust:latest as builder

# Prepare the build environment
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Install cargo-build-dependencies to cache dependencies
RUN cargo install cargo-build-dependencies
RUN cd /tmp && USER=root cargo new --bin bully_bot_rs
WORKDIR /tmp/bully_bot_rs

COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

# Build the final image
FROM scratch

WORKDIR /app

COPY --from=builder /tmp/bully_bot_rs/target/x86_64-unknown-linux-musl/release/bully_bot_rs ./bully_bot_rs

CMD ["./bully_bot_rs"]