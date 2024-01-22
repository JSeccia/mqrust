# Use the official Rust image as a base
FROM rust:latest as rust-builder

# Create a new empty shell project
RUN USER=root cargo new --bin consumer
WORKDIR /consumer

# Install cmake (required for rdkafka)
RUN apt-get update && apt-get install -y cmake

# Copy your manifests
COPY ./consumer/Cargo.toml ./Cargo.toml
COPY ./consumer/Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now copy your source code
COPY consumer/src ./src

# Build your Rust application
RUN rm ./target/release/deps/consumer*
RUN cargo build --release

# Use Node image to build the React app
FROM node:latest as react-builder

WORKDIR /frontend

# Copy your frontend files and install dependencies
COPY ./frontend/package.json ./package.json
COPY ./frontend/package-lock.json ./package-lock.json
RUN npm install

# Build the React app
COPY ./frontend .
RUN npm run build

# Final Stage
FROM debian:bookworm-slim

# Copy the Rust binary from the rust-builder stage
COPY --from=rust-builder /consumer/target/release/consumer .

# Copy the React build from the react-builder stage
COPY --from=react-builder /frontend/build ./static

# Set the startup command to run your binary
CMD ["./consumer"]
