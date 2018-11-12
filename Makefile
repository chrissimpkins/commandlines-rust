doc: fmt
	cargo doc --open

fmt:
	cargo fmt --verbose

lint: fmt
	cargo clippy --all-targets --all-features

publish:
	cargo publish

test: fmt
	cargo test --verbose


.PHONY: doc fmt lint publish test