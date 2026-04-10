# 1. Use a version that supports Rust 2024 (1.85+)
FROM lukemathwalker/cargo-chef:latest-rust-1.85.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependency tree stays the same, all layers are cached.
COPY . .
# Ensure SQLx doesn't try to connect to a live DB during build
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin email_newsletter

# 2. Use bookworm-slim for a modern, small runtime environment
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/email_newsletter email_newsletter
# Copy the configuration folder required by your get_configuration() function
COPY configuration configuration

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./email_newsletter"]