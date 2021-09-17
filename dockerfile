FROM rustlang/rust:nightly as builder

RUN apt-get update
RUN apt-get install -y g++-aarch64-linux-gnu \
    qemu-system-aarch64

WORKDIR /usr/src/raspi-rust

COPY ./src ./src
COPY ./.cargo ./.cargo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./build.rs ./build.rs
COPY ./linker.ld ./linker.ld

RUN cargo b -vv