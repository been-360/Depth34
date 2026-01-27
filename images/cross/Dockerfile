FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y \
        libudev-dev:arm64 \
        libc6-dev:arm64 \
        pkg-config \
        git \
        make \
        gcc-aarch64-linux-gnu \
        binutils-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/joan2937/pigpio.git && \
    cd pigpio && \
    make CC=aarch64-linux-gnu-gcc STRIP=aarch64-linux-gnu-strip && \
    make install PREFIX=/usr && \
    ldconfig
