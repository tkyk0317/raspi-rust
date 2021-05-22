.PHONY: build clean dbuild
build:
	@ cargo b -vv

clean:
	@ cargo clean

run:
	@ cargo r

asm:
	@ aarch64-linux-gnu-objdump -S target/aarch64-unknown-linux-gnu/debug/raspberry-pi-rust
