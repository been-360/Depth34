FROM debian:bookworm

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        curl build-essential \
        gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu \
        libc6-dev:arm64 libudev-dev:arm64 pkg-config git ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable \
 && . "$HOME/.cargo/env" \
 && rustup target add aarch64-unknown-linux-gnu

ENV PATH="/root/.cargo/bin:${PATH}"
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig

WORKDIR /project
