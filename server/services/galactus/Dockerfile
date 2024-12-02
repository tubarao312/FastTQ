# Stage 1: Install cargo-chef
FROM rust:1.82.0 AS chef
WORKDIR /server
RUN cargo install cargo-chef

# Stage 2: Plan the build with cargo-chef
FROM chef AS planner
WORKDIR /server/services/galactus
# Copy only the manifest and libs for dependency resolution
COPY server/libs /server/libs
COPY server/services/galactus/Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Generate the cargo-chef recipe
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies
FROM chef AS cacher
WORKDIR /server/services/galactus
# Copy the recipe and shared libs
COPY --from=planner /server/services/galactus/recipe.json .
COPY server/libs /server/libs
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 4: Build the application
FROM rust:1.82.0 AS builder
WORKDIR /server/services/galactus
# Copy dependencies from previous stages
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY --from=cacher /server/libs /server/libs
COPY server/services/galactus/Cargo.toml .
COPY server/services/galactus/src src
COPY server/services/galactus/.sqlx .sqlx
# Rebuild the application with the full source
RUN cargo build --release
RUN strip target/release/galactus

# Stage 5: Create the runtime image
FROM gcr.io/distroless/cc-debian12:latest AS release
WORKDIR /app
# Copy the final binary
COPY --from=builder /server/services/galactus/target/release/galactus .
EXPOSE 3000
CMD ["./galactus"]
