testfile := "test.wc"

default: fmt clippy test (run testfile)

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test --workspace

run file=testfile:
	cargo run -- {{file}}
