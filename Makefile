PANDOC ?= pandoc

.PHONY: check
check:
	@cargo check
	@cargo clippy -- -D warnings

.PHONY: build
build:
	@SHELL_COMPLETIONS_DIR=scripts/completion cargo build
	@git diff --exit-code

.PHONY: manual
manual:
	@$(PANDOC) --standalone --to man docs/UserManual.md -o docs/ea.1

.PHONY: test
test:
	@cargo test

.PHONY: cargo-publish
cargo-publish:
	@cargo publish --dry-run

.PHONY: check-version
check-version:
	@scripts/check-version.py
