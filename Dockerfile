# Builder stage
FROM rust:latest AS builder
WORKDIR /app
RUN apt update&&apt install lld clang -y
COPY .  .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM rust:bullseye-slim AS runtime
WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]










# FROM rust:latest
# WORKDIR /app
# RUN apt update && apt install lld clang -y
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build --release
# ENV APP_ENVIRONMENT production
# ENTRYPOINT ["./target/release/zero2prod"]