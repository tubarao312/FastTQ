# Stage 1: Install cargo-chef and build dependencies
FROM rust:1.82.0 AS chef
WORKDIR /server
RUN cargo install cargo-chef

WORKDIR /server/services/galactus
# Copy everything needed for dependency resolution and recipe creation
COPY server/libs /server/libs
COPY server/services/galactus/Cargo.toml .
COPY server/services/galactus .

# Generate the cargo-chef recipe
RUN cargo chef prepare --recipe-path recipe.json
# Cook the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 2: Build the application
FROM chef AS builder
WORKDIR /server/services/galactus
ENV SQLX_OFFLINE=true
# Combine COPY commands
COPY --from=chef /usr/local/cargo /usr/local/cargo
COPY --from=chef /server/libs /server/libs
COPY server/services/galactus/ .

# Rebuild the application with the full source
RUN cargo build --release
RUN strip target/release/galactus

# Stage 3: Create the runtime image
FROM gcr.io/distroless/cc-debian12:nonroot AS release
WORKDIR /app
# Copy the final binary
COPY --from=builder /server/services/galactus/target/release/galactus .
EXPOSE 3000
CMD ["./galactus"]
