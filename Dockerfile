# Use a lightweight base image with necessary tools
FROM ubuntu:20.04 AS downloader

# Install gsutil
RUN apt-get update && apt-get install -y \
    curl \
    python3 \
    python3-pip && \
    pip3 install gsutil

# Download the binary from GCS
ARG GCS_BINARY_PATH
RUN gsutil cp gs://climb-service-image-store-bucket/rust_app/climb_service /app/climb_service

# Final stage: use a minimal base image
FROM gcr.io/distroless/cc

# Copy the downloaded binary from the previous stage
COPY --from=downloader /app/climb_service /usr/local/bin/climb_service

# Run the binary
CMD ["/usr/local/bin/climb_service"]
