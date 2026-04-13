FROM rust:1.89 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/mandelbrot-distributed /usr/local/bin/mandelbrot-distributed

ENTRYPOINT ["mandelbrot-distributed"]