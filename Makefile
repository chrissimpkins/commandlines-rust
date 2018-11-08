doc: fmt
	cargo doc --open

fmt:
	cargo fmt --verbose

publish:
	cargo publish

test: fmt
	cargo test


.PHONY: doc fmt publish test