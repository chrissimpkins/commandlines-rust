doc: fmt
	cargo doc --open

fmt:
	cargo fmt --verbose

lint: fmt
	cargo clippy

publish:
	cargo publish

test: fmt
	cargo test


.PHONY: doc fmt lint publish test