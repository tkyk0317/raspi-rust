#FROM rustlang/rust:nightly as builder
FROM rust:slim-buster

RUN apt-get update
RUN apt-get install -y g++-aarch64-linux-gnu \
    qemu-system-aarch64

WORKDIR /usr/src/raspi-rust

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./rust-toolchain ./rust-toolchain
RUN mkdir -p ./src && touch ./src/main.rs
RUN cargo fetch

RUN cargo install cargo-binutils
RUN rustup component add llvm-tools-preview

COPY ./src ./src
COPY ./.cargo ./.cargo
COPY ./build.rs ./build.rs
COPY ./linker.ld ./linker.ld

RUN --mount=type=cache,target=$HOME/.cargo \
    --mount=type=cache,target=/usr/src/raspi-rust/target \
    cargo build -v
