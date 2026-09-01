# ---------- Build stage ----------
FROM rust:1.89-bookworm AS builder

WORKDIR /app

# Copy the complete project.
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the actual application.
RUN cargo build --release


# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/http /app/http

EXPOSE 8001

CMD ["/app/http"]