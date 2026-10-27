.PHONY: test test-unit test-cli fmt lint build release release-notes

test:
	./scripts/test.sh

test-unit:
	cargo test --workspace

test-cli:
	@tmp="$$(mktemp -d)"; trap 'rm -rf "$$tmp"' EXIT; ./scripts/test-cli.sh "$$tmp"

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

build:
	cargo build --workspace

release:
	cargo build --workspace --release

release-notes:
	./scripts/release-notes.sh
