#!/usr/bin/env bash
# End-to-end checks for public CLI behavior. Invoked by scripts/test.sh.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if [[ $# -ne 1 ]]; then
    printf 'usage: %s <temporary-directory>\n' "$0" >&2
    exit 64
fi

TMP_DIR="$1"
BIN="target/debug/rustfetch"

# Keep standalone invocations as isolated as the full test runner.
export XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$TMP_DIR/config}"
export XDG_CACHE_HOME="${XDG_CACHE_HOME:-$TMP_DIR/cache}"
export NO_COLOR="${NO_COLOR:-1}"

printf '\n==> cargo build -p rustfetch\n'
cargo build -p rustfetch

DEFAULT_CONFIG="$TMP_DIR/default-config.toml"
JSON_OUTPUT="$TMP_DIR/output.json"
COMPLETION_OUTPUT="$TMP_DIR/rustfetch.bash"
FRAME_OUTPUT="$TMP_DIR/image-frame.txt"

printf '\n==> %s --print-default-config\n' "$BIN"
"$BIN" --print-default-config > "$DEFAULT_CONFIG"
test -s "$DEFAULT_CONFIG"

printf '\n==> %s --json --no-logo\n' "$BIN"
"$BIN" --json --no-logo > "$JSON_OUTPUT"
test -s "$JSON_OUTPUT"
rg -q '^\[' "$JSON_OUTPUT"
rg -q '"name"' "$JSON_OUTPUT"

printf '\n==> %s --no-logo --border --border-image-position frame\n' "$BIN"
"$BIN" --no-logo --border --border-title "System" \
    --border-image images/arch.png --border-image-width 4 \
    --border-image-position frame > "$FRAME_OUTPUT"
rg -q 'System' "$FRAME_OUTPUT"
rg -q 'OS' "$FRAME_OUTPUT"

for preset in default minimal clean dots card brackets neofetch retro modern compact; do
    printf '\n==> %s --preset %s --json --no-logo\n' "$BIN" "$preset"
    "$BIN" --preset "$preset" --json --no-logo > /dev/null
done

printf '\n==> %s --completions bash\n' "$BIN"
"$BIN" --completions bash > "$COMPLETION_OUTPUT"
rg -q 'rustfetch' "$COMPLETION_OUTPUT"

if "$BIN" --preset does-not-exist --no-logo > /dev/null 2>&1; then
    printf 'unknown presets must return a non-zero status\n' >&2
    exit 1
fi
