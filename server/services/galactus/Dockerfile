FROM rust:1.82.0 AS chef
WORKDIR /server
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /server

# Copy only dependency files to optimize caching
COPY server/libs ./libs
COPY server/services/galactus ./services/galactus/

WORKDIR /
COPY Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /server/services/galactus
COPY --from=planner /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy full source code after caching dependencies
COPY server/libs /server/libs
COPY server/services/galactus/ .

ENV SQLX_OFFLINE=true

RUN cargo build --release
RUN strip target/release/galactus

FROM gcr.io/distroless/cc-debian12:nonroot AS release
WORKDIR /app

COPY --from=builder /server/services/galactus/target/release/galactus .

EXPOSE 3000

CMD ["./galactus"]