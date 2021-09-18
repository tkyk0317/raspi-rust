.PHONY: build clean run test
build:
	@ docker build . -t raspi-rust

test: build
	@ docker run -t raspi-rust cargo t

clean:
	@ cargo clean

run: build
	@ docker run -t raspi-rust cargo r
