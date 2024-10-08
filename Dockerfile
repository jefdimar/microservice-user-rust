# Chef Stage: Set up the base image with cargo-chef
FROM rust:1.74 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Planner Stage: Prepare the recipe for dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage: Build the project
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Runtime Stage: Create the final, slim runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the built binary and set up the runtime environment
COPY --from=builder /app/target/release/new_project .
EXPOSE 8000
CMD ["./new_project"]
