.PHONY: check
check:
	@cargo check
	@cargo clippy -- -D warnings

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: cargo-publish
cargo-publish:
	@cargo publish --dry-run

.PHONY: check-version
check-version:
	@scripts/check-version.py
