# Build stage
FROM rust:latest As builder

WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create project and build app dependencies
RUN USER=root cargo new haiku
WORKDIR /usr/src/haiku
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy src files
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/haiku .
USER 1000
CMD ["./haiku", "-a", "0.0.0.0", "-p", "3000"]