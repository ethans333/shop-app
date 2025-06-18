FROM rust:latest AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

FROM debian:bookworm
WORKDIR /app
COPY --from=builder /app/target/release/api /app/
CMD ["./api"]