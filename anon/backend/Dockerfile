# ---- Builder ----
FROM rust:1.86-slim AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml ./
# Create a dummy main to cache deps
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Now copy sources and build
COPY . .
RUN cargo build --release

# ---- Runtime ----
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Minimal runtime deps
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/app

ENV RUST_LOG=info
ENV PORT=8080
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["/usr/local/bin/app"]
