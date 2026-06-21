.PHONY: build test run clean fmt check ci

build:
	cargo build --release

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-targets

ci: check build

clean:
	cargo clean
