# Set up an isolated environment to run Node.js + Rust + WebAssembly

FROM mcr.microsoft.com/devcontainers/javascript-node:1-20-bookworm AS workspace

# Install build dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y \
    lldb \
    vim \
    nano \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /home/node

SHELL [ "/bin/bash", "-c" ]

# Install Rust
ENV RUSTUP_HOME="/home/node/.cargo"
ENV CARGO_HOME="/home/node/.cargo"
ENV PATH="${CARGO_HOME}/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path \
    && export PATH="$HOME/.cargo/bin:$PATH" \
    && echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.bashrc

RUN rustup default stable \
    && rustup target add wasm32-unknown-unknown

RUN cargo install -f wasm-bindgen-cli --version 0.2.88
RUN cargo install cargo-watch

WORKDIR /app

############

FROM python:3.12.0-slim-bookworm AS build-wabt

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y \
    build-essential \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN git clone --recursive https://github.com/WebAssembly/wabt \
    && cd wabt \
    && git submodule update --init \ 
    && mkdir build \
    && cd build \
    && cmake .. \
    && cmake --build .

############

FROM workspace

COPY --from=build-wabt /app/wabt/build/wasm* /usr/bin/

CMD ["/bin/bash", "-c", "./install.sh && /bin/bash"]
