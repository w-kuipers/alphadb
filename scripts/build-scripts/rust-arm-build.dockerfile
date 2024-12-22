FROM rust:latest

# Install dependencies for cross-compiling to aarch64 (ARM 64-bit)
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libssl-dev:arm64 \
    pkg-config \
    qemu-user-static && \
    rustup target add aarch64-unknown-linux-gnu

WORKDIR /app/src/alphadb
COPY . .

# Set environment variable for OpenSSL
ENV OPENSSL_DIR=/usr/aarch64-linux-gnu

CMD ["cargo", "build", "--release", "--target=aarch64-unknown-linux-gnu"]

