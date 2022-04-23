#Builder stage
FROM rust:1.59.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
# Runtime stage
FROM rust:1.59.0 AS runtime

WORKDIR /APP

COPY --from=builder /app/target/release/zerotoprod zerotoprod
COPY configuration configuration
env APP_ENVIRONMENT production

ENTRYPOINT ["./zerotoprod"]
