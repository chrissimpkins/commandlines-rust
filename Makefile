fmt:
	cargo fmt --verbose

publish:
	cargo publish

test: fmt
	cargo test


.PHONY: fmt publish test