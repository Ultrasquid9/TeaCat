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

fg file=testfile:
	cargo flamegraph --dev -- {{file}} --stress_test

update:
	git fetch
	git pull
	cargo update
