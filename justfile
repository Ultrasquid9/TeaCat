testfile := "test.tcat"
targetfile := "index.html"

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

out file=testfile target=targetfile:
	cargo run -- {{file}} --out {{target}}

update:
	git fetch
	git pull
	cargo update
