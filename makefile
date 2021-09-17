.PHONY: build clean dbuild
build:
	@ docker build . -t raspi-rust

test: build
	@ docker run -t raspi-rust cargo t

clean:
	@ cargo clean

run: build
	@ docker run -t raspi-rust cargo r

asm:
	@ aarch64-linux-gnu-objdump -S target/aarch64-unknown-linux-gnu/debug/raspberry-pi-rust
