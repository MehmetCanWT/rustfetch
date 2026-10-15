#!/usr/bin/env bash
# Automated Terminal Geometry & Dimension Matrix Tester for RustFetch
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT_DIR/target/release/rustfetch"

if [ ! -f "$BIN" ]; then
    cargo build --release -p rustfetch
fi

OUT_DIR="$ROOT_DIR/scratch/dimension_tests"
mkdir -p "$OUT_DIR"

SIZES=(
    "45x15:ultra-compact"
    "60x20:quarter-tile"
    "80x24:vt100-standard"
    "100x30:half-tile"
    "140x40:wide-tile"
    "200x55:fullscreen-1080p"
)

PRESETS=("default" "card" "dots" "minimal" "compact" "modern" "retro")

echo "=========================================================="
echo "RustFetch Terminal Geometry & Memory Benchmark Suite"
echo "=========================================================="

printf "%-22s %-12s %-10s %-12s %-10s\n" "Dimension (Cols x Rows)" "Preset" "Status" "RAM (RSS)" "Latency"
printf "%-22s %-12s %-10s %-12s %-10s\n" "----------------------" "------" "------" "---------" "-------"

for entry in "${SIZES[@]}"; do
    dim="${entry%%:*}"
    label="${entry##*:}"
    cols="${dim%x*}"
    rows="${dim#*x}"

    for preset in "${PRESETS[@]}"; do
        out_file="$OUT_DIR/${dim}_${preset}.txt"
        
        # Use /usr/bin/time to capture maximum RSS and wall clock
        # Emulate terminal columns and lines using stty/pty wrapper
        TIME_LOG="$OUT_DIR/time_${dim}_${preset}.log"
        
        /usr/bin/time -v env COLUMNS="$cols" LINES="$rows" TERM="xterm-256color" \
            "$BIN" --preset "$preset" > "$out_file" 2> "$TIME_LOG" || true

        # Extract metrics
        rss="$(grep 'Maximum resident set size' "$TIME_LOG" | awk '{print $NF}')"
        if [ -n "$rss" ]; then
            rss_mb="$(awk "BEGIN {printf \"%.2f MB\", $rss / 1024}")"
        else
            rss_mb="N/A"
        fi

        # Check for line overruns
        max_col_detected=0
        while IFS= read -r line; do
            # Strip ANSI escape codes to measure true printable character column width
            plain="$(printf '%s' "$line" | sed 's/\x1b\[[0-9;]*[a-zA-Z]//g')"
            len="${#plain}"
            if [ "$len" -gt "$max_col_detected" ]; then
                max_col_detected="$len"
            fi
        done < "$out_file"

        status="OK"
        if [ "$max_col_detected" -gt "$cols" ]; then
            status="WRAP (${max_col_detected}>${cols})"
        fi

        printf "%-22s %-12s %-10s %-12s\n" "${cols}x${rows} ($label)" "$preset" "$status" "$rss_mb"
    done
done

echo ""
echo "Output files saved to: $OUT_DIR"
