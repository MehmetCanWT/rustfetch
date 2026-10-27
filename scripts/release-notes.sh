#!/usr/bin/env bash
# ==============================================================================
# RustFetch — Release Notes, Changelog & Cat Breed Codename Generator
# Usage:
#   ./scripts/release-notes.sh [VERSION_OR_TAG]
#   ./scripts/release-notes.sh --codename [VERSION_OR_TAG]
#   ./scripts/release-notes.sh --title [VERSION_OR_TAG]
# ==============================================================================
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MODE="notes"
if [[ "${1:-}" == "--codename" ]]; then
    MODE="codename"
    shift
elif [[ "${1:-}" == "--title" ]]; then
    MODE="title"
    shift
fi

RAW_VER="${1:-}"
if [[ -z "$RAW_VER" ]]; then
    RAW_VER="$(grep -E '^version\s*=' Cargo.toml | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')"
fi
VERSION="${RAW_VER#v}"
VERSION="${VERSION#V}"

codename_for_version() {
    local v="$1"
    case "$v" in
        0.1.0) echo "Turkish Van" ;;
        0.1.1) echo "Turkish Angora" ;;
        0.1.2) echo "Bengal" ;;
        0.1.3) echo "Maine Coon" ;;
        0.1.4) echo "Siamese" ;;
        0.1.5) echo "Ragdoll" ;;
        0.1.6) echo "Persian" ;;
        0.1.7) echo "Birman" ;;
        0.1.8) echo "Chartreux" ;;
        0.1.9) echo "Korat" ;;
        0.2.0) echo "Abyssinian" ;;
        0.2.1) echo "British Shorthair" ;;
        0.2.2) echo "Scottish Fold" ;;
        0.2.3) echo "Russian Blue" ;;
        0.2.4) echo "Norwegian Forest" ;;
        0.2.5) echo "Siberian" ;;
        0.3.0) echo "Savannah" ;;
        0.3.1) echo "Ocicat" ;;
        0.3.2) echo "Somali" ;;
        0.4.0) echo "Sphynx" ;;
        1.0.0) echo "Anatolian" ;;
        *)
            local breeds=(
                "Turkish Van" "Turkish Angora" "Bengal" "Maine Coon"
                "Siamese" "Ragdoll" "Persian" "Birman"
                "Chartreux" "Korat" "Abyssinian" "British Shorthair"
                "Scottish Fold" "Russian Blue" "Norwegian Forest" "Siberian"
                "Savannah" "Ocicat" "Somali" "Sphynx"
                "Bombay" "Burmese" "Manx" "Anatolian"
            )
            local IFS='.'
            read -r maj min pat <<< "${v%%[-+]*}"
            maj="${maj:-0}"
            min="${min:-0}"
            pat="${pat:-0}"
            local idx=$(( (maj * 31 + min * 10 + pat) % ${#breeds[@]} ))
            echo "${breeds[$idx]}"
            ;;
    esac
}

CODENAME="$(codename_for_version "$VERSION")"

if [[ "$MODE" == "codename" ]]; then
    echo "$CODENAME"
    exit 0
elif [[ "$MODE" == "title" ]]; then
    echo "RustFetch v${VERSION} — \"${CODENAME}\""
    exit 0
fi

# Extract curated section from CHANGELOG.md if available
CURATED_NOTES=""
if [[ -f "CHANGELOG.md" ]]; then
    CURATED_NOTES="$(awk -v ver="v${VERSION}" '
        $0 ~ "^## \\[" ver "\\]" { found=1; next }
        found && /^## \[/ { exit }
        found && /^---$/ { next }
        found { print }
    ' CHANGELOG.md | sed -e '/./,$!d')"
fi

if [[ -z "$CURATED_NOTES" && "$VERSION" == "0.1.2" ]]; then
    CURATED_NOTES="$(cat <<'NOTES'
### ✨ Highlights
- **Terminal Resize Stability**: Added `SIGWINCH` debouncing, Kitty Synchronized Output (`\x1b[?2026h`), and cell-coordinate image positioning so extreme terminal shrinking/enlarging never causes overlapping logos or duplicated lines.
- **Bash Prompt Trap Fix**: Fixed `--shell-init bash` `DEBUG` trap firing during `.bashrc` initialization before the first prompt.
- **Single-Pass Image Palette & Color Cache**: Dominant accent color and 16-color palette are extracted in a single PNG decode pass and cached together in `.color`.
- **Atomic Self-Update (`--self-update`)**: Binary replacement now writes to a staging file and atomically renames over the running executable (`ETXTBSY`-safe).
- **Custom Disk Mount Persistence in 3D Mode**: Background module refreshes in `--3d` mode now preserve custom `[disk]` mount paths (`path = "/home"`).
- **Symlink-Safe Homebrew Package Counting**: Deduplicated canonical Cellar directories across `/home/linuxbrew/.linuxbrew` and `~/.linuxbrew`.
- **Static Musl Compatibility**: Replaced glibc-only `libc::statx` bindings with portable `std::fs::Metadata::created` so `x86_64-unknown-linux-musl` builds cleanly.
NOTES
)"
fi

# Determine git commit range since previous tag
PREV_TAG=""
if git rev-parse --git-dir >/dev/null 2>&1; then
    if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
        PREV_TAG="$(git describe --tags --abbrev=0 "v${VERSION}^" 2>/dev/null || true)"
        RANGE="${PREV_TAG:+${PREV_TAG}..}v${VERSION}"
    else
        PREV_TAG="$(git describe --tags --abbrev=0 HEAD 2>/dev/null || true)"
        RANGE="${PREV_TAG:+${PREV_TAG}..}HEAD"
    fi
else
    RANGE=""
fi

cat <<EOF
# 🐱 RustFetch v${VERSION} — "${CODENAME}"

> **Release Version:** \`v${VERSION}\`  
> **Codename:** **${CODENAME}**  

EOF

if [[ -n "$CURATED_NOTES" ]]; then
    cat <<EOF
${CURATED_NOTES}

---
EOF
fi

if [[ -n "$RANGE" ]] && git rev-parse --git-dir >/dev/null 2>&1; then
    COMMITS="$(git log "$RANGE" --no-merges --pretty=format:'- %s (`%h`)' 2>/dev/null || true)"
    if [[ -n "$COMMITS" ]]; then
        echo ""
        echo "### 📋 Commit History (${PREV_TAG:-initial} → v${VERSION})"
        echo ""
        echo "$COMMITS"
        echo ""
        echo "---"
    fi
fi

cat <<EOF

## 📦 Packages & Downloads

All release packages use clean distribution and architecture names (no \`unknown\` target triples):

| Distribution / Platform | Architecture | Package File | Installation Command |
|---|---|---|---|
| **Arch Linux** (CachyOS, Manjaro, EndeavourOS) | \`x86_64\` | \`rustfetch-v${VERSION}-arch-x86_64.pkg.tar.zst\` | \`sudo pacman -U rustfetch-v${VERSION}-arch-x86_64.pkg.tar.zst\` |
| **Fedora / Bazzite / RHEL / openSUSE** | \`x86_64\` | \`rustfetch-${VERSION}-1.fedora.x86_64.rpm\` | \`sudo dnf install ./rustfetch-${VERSION}-1.fedora.x86_64.rpm\` |
| **Debian / Ubuntu** (Mint, Pop!_OS, Kali) | \`x86_64\` | \`rustfetch_${VERSION}_debian_amd64.deb\` | \`sudo dpkg -i rustfetch_${VERSION}_debian_amd64.deb\` |
| **NixOS / Nix** | \`x86_64\` | \`rustfetch-v${VERSION}-nixos-x86_64.tar.gz\` | \`nix profile install github:MehmetCanWT/rustfetch\` |
| **Linux GNU** (Standard glibc) | \`x86_64\` | \`rustfetch-v${VERSION}-linux-gnu-x86_64.tar.gz\` | \`tar -xzf rustfetch-v${VERSION}-linux-gnu-x86_64.tar.gz\` |
| **Linux GNU ARM64** (aarch64) | \`aarch64\` | \`rustfetch-v${VERSION}-linux-gnu-aarch64.tar.gz\` | \`tar -xzf rustfetch-v${VERSION}-linux-gnu-aarch64.tar.gz\` |
| **Linux Musl** (Alpine, Void, Static) | \`x86_64\` | \`rustfetch-v${VERSION}-linux-musl-x86_64.tar.gz\` | \`tar -xzf rustfetch-v${VERSION}-linux-musl-x86_64.tar.gz\` |

### ⚡ Universal One-Line Install
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/MehmetCanWT/rustfetch/main/install.sh | bash
\`\`\`
EOF
