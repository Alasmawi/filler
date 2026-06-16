.PHONY: build test run clean fmt

build:
	cargo build --release

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

clean:
	cargo clean
