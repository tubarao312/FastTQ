FROM rust:1.82.0 AS builder

WORKDIR /server

COPY server/libs libs

WORKDIR /server/services/galactus

COPY server/services/galactus/Cargo.toml .
COPY server/services/galactus/.sqlx .sqlx
# Create empty src/main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY server/services/galactus/src src
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/galactus

FROM gcr.io/distroless/cc-debian12:latest as release
WORKDIR /app
COPY --from=builder /server/services/galactus/target/release/galactus .

EXPOSE 3000

CMD ["./galactus"]