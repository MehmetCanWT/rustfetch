#!/usr/bin/env bash
# MangoWM / Wayland Multi-Window & Dimension Screenshot Automation Test
# This script switches to Workspace 5, opens multiple tiled Kitty windows with
# different presets and sizes, captures a desktop screenshot with grim, and measures resource usage.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT_DIR/target/release/rustfetch"
SCREENSHOT_DIR="$ROOT_DIR/scratch/screenshots"
mkdir -p "$SCREENSHOT_DIR"

if [ ! -f "$BIN" ]; then
    echo "Building release binary..."
    cargo build --release -p rustfetch
fi

echo "=========================================================="
echo "MangoWM Workspace 5 Automated UI & Geometry Test"
echo "=========================================================="

# 1. Switch to Workspace 5
echo "--> Switching to Workspace 5..."
if command -v mango-msg &>/dev/null; then
    mango-msg workspace 5 || true
elif command -v hyprctl &>/dev/null; then
    hyprctl dispatch workspace 5 || true
elif command -v swaymsg &>/dev/null; then
    swaymsg workspace 5 || true
else
    echo "Note: Generic Wayland environment detected. Please ensure Workspace 5 is active."
fi

sleep 0.5

# PIDs array to clean up after test
PIDS=()

cleanup() {
    echo "--> Cleaning up opened test terminals..."
    for pid in "${PIDS[@]}"; do
        kill "$pid" 2>/dev/null || true
    done
}
trap cleanup EXIT

# 2. Spawn 4 tiled terminals with diverse dimensions, fonts, and presets
echo "--> Spawning tiled terminals on Workspace 5..."

# Terminal 1: Top-Left (Default with distro ASCII)
kitty --title "RustFetch-Default" --override font_size=11 --hold sh -c "$BIN" &
PIDS+=($!)
sleep 0.3

# Terminal 2: Top-Right (Card preset)
kitty --title "RustFetch-Card" --override font_size=10 --hold sh -c "$BIN --preset card" &
PIDS+=($!)
sleep 0.3

# Terminal 3: Bottom-Left (Dots preset with border)
kitty --title "RustFetch-Dots" --override font_size=9 --hold sh -c "$BIN --preset dots --border" &
PIDS+=($!)
sleep 0.3

# Terminal 4: Bottom-Right (Modern / Compact preset)
kitty --title "RustFetch-Compact" --override font_size=10 --hold sh -c "$BIN --preset compact" &
PIDS+=($!)

echo "--> Waiting 2 seconds for window layout settling and rendering..."
sleep 2

# 3. Capture Screenshot with grim
TIMESTAMP="$(date +'%Y%m%d_%H%M%S')"
SCREENSHOT_PATH="$SCREENSHOT_DIR/mangowm_workspace5_${TIMESTAMP}.png"

echo "--> Capturing full screen with grim..."
if command -v grim &>/dev/null; then
    grim "$SCREENSHOT_PATH"
    echo "✓ Screenshot saved to: $SCREENSHOT_PATH"
else
    echo "Warning: grim not found. Install grim to enable automatic screenshot capture."
fi

# 4. Measure System Metrics
echo ""
echo "=== System Telemetry Profiling ==="
"$BIN" --benchmark

echo ""
echo "=== Process & Memory Stats for Spawned Windows ==="
for pid in "${PIDS[@]}"; do
    if ps -p "$pid" > /dev/null; then
        rss="$(ps -o rss= -p "$pid" | tr -d ' ')"
        rss_mb="$(awk "BEGIN {printf \"%.2f MB\", $rss / 1024}")"
        comm="$(ps -o comm= -p "$pid" | tr -d ' ')"
        echo "PID $pid ($comm): RSS = $rss_mb"
    fi
done

echo ""
echo "Test completed successfully."
echo "Keep windows open for 5 seconds for visual review..."
sleep 5
