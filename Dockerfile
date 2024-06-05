# stage 1: Build the Rust project
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/climb_service

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project and build just the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Remove the temporary src folder
RUN rm -rf src

# Copy the source code to the container
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Create the final image with the built binary
FROM debian:buster-slim

# Copy the binary from the builder stage
COPY --from=builder /usr/src/climb_service/target/release/climb_service /usr/local/bin/climb_service

# Set the command to run the binary
CMD ["myapp"]
