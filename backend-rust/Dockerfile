# Cambia rust:1.80-slim por rust:latest para soportar edition2024
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Imagen final ligera con soporte de utilidades de red (iproute2/tc)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y iproute2 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/marketplace-backend /app/marketplace-backend
EXPOSE 8080
CMD ["./marketplace-backend"]