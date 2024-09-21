FROM rust:1.74 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Use Debian Bookworm as the runtime base
FROM debian:bookworm-slim AS runtime

# Install necessary runtime libraries
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/new_project .
EXPOSE 8000
CMD ["./new_project"]