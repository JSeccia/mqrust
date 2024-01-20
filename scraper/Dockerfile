# Use the official Rust image as a base
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin scraper
WORKDIR /scraper

# Install cmake (required for rdkafka)
RUN apt-get update && apt-get install -y cmake

# Copy your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now copy your source code
COPY ./src ./src

# Build your application
RUN rm ./target/release/deps/scraper*
RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates
RUN apt-get update && apt-get install -y libssl-dev
# Copy the binary from the builder stage
COPY --from=builder /scraper/target/release/scraper .

# Set the startup command to run your binary
CMD ["./scraper"]
