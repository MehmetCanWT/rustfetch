#!/usr/bin/env bash
# RustFetch's reproducible local/CI verification entrypoint.
# It deliberately keeps test data out of the user's real XDG directories.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/rustfetch-test.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT
export XDG_CONFIG_HOME="$TMP_DIR/config"
export XDG_CACHE_HOME="$TMP_DIR/cache"
export NO_COLOR=1

run() {
    printf '\n==> %s\n' "$*"
    "$@"
}

run cargo fmt --all -- --check
run cargo test --workspace
run cargo clippy --workspace --all-targets -- -D warnings
run cargo build --workspace --release

run bash scripts/test-cli.sh "$TMP_DIR"
