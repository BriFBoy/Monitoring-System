FROM rust:latest AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/monitoring-backend-rs /monitoring-system-rs
EXPOSE 8081
CMD ["/monitoring-system-rs"]
