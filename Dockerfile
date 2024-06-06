# stage 1: Build the Rust project
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/climb_service

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock /usr/src/climb_service/

# Create a new empty shell project and build just the dependencies

# Remove the temporary src folder

# Copy the source code to the container
COPY ./src/. /usr/src/climb_service/src/

# Build the project
RUN cargo build --release

# Stage 2: Create the final image with the built binary

# Copy the binary from the builder stage

EXPOSE 8080
ENTRYPOINT /usr/src/climb_service/target/release/climb_service

