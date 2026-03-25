# =========================================
# Base Chef Image
# =========================================
FROM lukemathwalker/cargo-chef:latest-rust-1.94-alpine AS chef
WORKDIR /app

# =========================================
# Planner Stage
# =========================================
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# =========================================
# Build Stage
# =========================================
FROM chef AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json -p collector

COPY . .

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release -p collector
RUN strip target/release/collector

# =========================================
# Runtime Stage
# =========================================
FROM alpine:latest

RUN apk add --no-cache openssl ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/collector /app/collector
COPY --from=builder /app/collector/migrations /app/migrations

ENTRYPOINT ["/app/collector"]
