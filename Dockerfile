FROM rust:latest as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/haiku /
CMD ["./haiku"]