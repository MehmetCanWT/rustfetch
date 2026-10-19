#!/usr/bin/env bash
# ==============================================================================
# RustFetch — Universal One-Line Installer
# Supported: Arch, Debian, Ubuntu, Fedora, Alpine, Void, NixOS, openSUSE, etc.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/MehmetCanWT/rustfetch/main/install.sh | bash
# or locally:
#   ./install.sh
# ==============================================================================

set -e

REPO="MehmetCanWT/rustfetch"
GITHUB_URL="https://github.com/${REPO}"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}==>${NC} Installing ${GREEN}RustFetch${NC}..."

# 0. Ensure Homebrew (Bazzite) and Cargo environments are loaded if available
for dir in "/home/linuxbrew/.linuxbrew/bin" "$HOME/.cargo/bin" "$HOME/.local/bin"; do
    if [ -d "$dir" ] && [[ ":$PATH:" != *":$dir:"* ]]; then
        export PATH="$dir:$PATH"
    fi
done
if ! command -v cargo &>/dev/null && [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

# 1. Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$OS" != "linux" ]; then
    echo -e "${RED}Error:${NC} RustFetch currently supports Linux only."
    exit 1
fi

case "$ARCH" in
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        TARGET_ARCH=""
        ;;
esac

# Detect libc (musl vs gnu, e.g. Alpine Linux)
IS_MUSL=false
if [ -f /etc/alpine-release ] || (ldd --version 2>&1 | grep -iq musl); then
    IS_MUSL=true
fi

TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT

INSTALLED_FROM_BINARY=false

ensure_build_tools() {
    if ! command -v cargo &>/dev/null; then
        echo -e "${YELLOW}==>${NC} Cargo is not installed. Installing minimal Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
        export PATH="$HOME/.cargo/bin:$PATH"
        . "$HOME/.cargo/env" 2>/dev/null || true
    fi

    # Check for C compiler/linker
    if ! command -v cc &>/dev/null && ! command -v gcc &>/dev/null && ! command -v clang &>/dev/null; then
        echo -e "${YELLOW}==>${NC} C compiler not found. Attempting to install build tools..."
        if [ -f /run/ostree-booted ] || command -v rpm-ostree &>/dev/null; then
            if command -v brew &>/dev/null; then
                brew install gcc || true
            fi
            if ! command -v cc &>/dev/null && ! command -v gcc &>/dev/null && [ -c /dev/tty ]; then
                sudo </dev/tty rpm-ostree install --apply-live --idempotent -y gcc || true
            fi
        elif command -v apt-get &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty apt-get update -qq && sudo </dev/tty apt-get install -y -qq build-essential || true
        elif command -v dnf &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty dnf install -y -q gcc || true
        elif command -v pacman &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty pacman -Sy --noconfirm base-devel || true
        elif command -v apk &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty apk add gcc musl-dev || true
        fi
    fi
}

# 2. Compile binary
if [ -f "Cargo.toml" ] && grep -q "rustfetch" Cargo.toml; then
    ensure_build_tools
    BUILD_DIR="$(pwd)"
    echo -e "${BLUE}==>${NC} Compiling local release binary (LTO optimized)..."
    cargo build --release -p rustfetch
else
    echo -e "${BLUE}==>${NC} Preparing RustFetch installation..."
    ensure_build_tools

    SRC_DIR="$TEMP_DIR/source"
    echo -e "${BLUE}==>${NC} Fetching latest source code from ${GITHUB_URL}..."
    git clone --depth 1 "${GITHUB_URL}.git" "$SRC_DIR"

    echo -e "${BLUE}==>${NC} Compiling release binary (LTO optimized)..."
    (cd "$SRC_DIR" && cargo build --release -p rustfetch)
    BUILD_DIR="$SRC_DIR"
fi

# 4. Locate binary
if [ -f "$BUILD_DIR/target/release/rustfetch" ]; then
    BINARY_PATH="$BUILD_DIR/target/release/rustfetch"
elif [ -f "$BUILD_DIR/rustfetch" ]; then
    BINARY_PATH="$BUILD_DIR/rustfetch"
else
    BINARY_PATH="$(find "$BUILD_DIR" -type f -name rustfetch -perm -111 | head -n 1)"
fi

if [ -z "$BINARY_PATH" ] || [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error:${NC} Could not find compiled rustfetch binary."
    exit 1
fi

# 5. Determine installation destination
SYSTEM_BIN="/usr/local/bin"
USER_BIN="${XDG_BIN_HOME:-$HOME/.local/bin}"

if [ "$EUID" -eq 0 ] || [ -w "$SYSTEM_BIN" ]; then
    FINAL_BIN_DIR="$SYSTEM_BIN"
else
    FINAL_BIN_DIR="$USER_BIN"
fi

mkdir -p "$FINAL_BIN_DIR"
cp -f "$BINARY_PATH" "$FINAL_BIN_DIR/rustfetch"
chmod +x "$FINAL_BIN_DIR/rustfetch"

# Install rfetch binary (prefer compiled binary if present, fallback to symlink)
if [ -f "$BUILD_DIR/target/release/rfetch" ]; then
    cp -f "$BUILD_DIR/target/release/rfetch" "$FINAL_BIN_DIR/rfetch"
    chmod +x "$FINAL_BIN_DIR/rfetch"
elif [ -f "$BUILD_DIR/rfetch" ]; then
    cp -f "$BUILD_DIR/rfetch" "$FINAL_BIN_DIR/rfetch"
    chmod +x "$FINAL_BIN_DIR/rfetch"
else
    ln -sf "$FINAL_BIN_DIR/rustfetch" "$FINAL_BIN_DIR/rfetch"
fi

# 6. Create config directory
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/rustfetch"
mkdir -p "$CONFIG_DIR/logos"

# 7. Install shell completions
INSTALLED_BIN="$FINAL_BIN_DIR/rustfetch"
if [ -x "$INSTALLED_BIN" ]; then
    echo -e "${BLUE}==>${NC} Installing shell auto-completions..."
    # Bash
    BASH_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/bash-completion/completions"
    mkdir -p "$BASH_DIR"
    "$INSTALLED_BIN" --completions bash > "$BASH_DIR/rustfetch" 2>/dev/null || true
    sed 's/rustfetch/rfetch/g' "$BASH_DIR/rustfetch" > "$BASH_DIR/rfetch" 2>/dev/null || ln -sf "$BASH_DIR/rustfetch" "$BASH_DIR/rfetch" 2>/dev/null || true

    # Zsh
    ZSH_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zsh/site-functions"
    mkdir -p "$ZSH_DIR"
    "$INSTALLED_BIN" --completions zsh > "$ZSH_DIR/_rustfetch" 2>/dev/null || true
    sed 's/rustfetch/rfetch/g' "$ZSH_DIR/_rustfetch" > "$ZSH_DIR/_rfetch" 2>/dev/null || ln -sf "$ZSH_DIR/_rustfetch" "$ZSH_DIR/_rfetch" 2>/dev/null || true

    # Fish
    FISH_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions"
    mkdir -p "$FISH_DIR"
    "$INSTALLED_BIN" --completions fish > "$FISH_DIR/rustfetch.fish" 2>/dev/null || true
    sed 's/complete -c rustfetch/complete -c rfetch/g' "$FISH_DIR/rustfetch.fish" > "$FISH_DIR/rfetch.fish" 2>/dev/null || ln -sf "$FISH_DIR/rustfetch.fish" "$FISH_DIR/rfetch.fish" 2>/dev/null || true
fi

# 8. Install man pages
MAN_SRC=""
if [ -f "$BUILD_DIR/docs/rustfetch.1" ]; then
    MAN_SRC="$BUILD_DIR/docs/rustfetch.1"
elif [ -f "docs/rustfetch.1" ]; then
    MAN_SRC="docs/rustfetch.1"
fi

if [ -n "$MAN_SRC" ]; then
    echo -e "${BLUE}==>${NC} Installing man pages..."
    if [ "$FINAL_BIN_DIR" = "$SYSTEM_BIN" ] && [ -w "/usr/local/share" ]; then
        SYS_MAN="/usr/local/share/man/man1"
        mkdir -p "$SYS_MAN"
        cp -f "$MAN_SRC" "$SYS_MAN/rustfetch.1"
        ln -sf "$SYS_MAN/rustfetch.1" "$SYS_MAN/rfetch.1" 2>/dev/null || true
    else
        USER_MAN="${XDG_DATA_HOME:-$HOME/.local/share}/man/man1"
        mkdir -p "$USER_MAN"
        cp -f "$MAN_SRC" "$USER_MAN/rustfetch.1"
        ln -sf "$USER_MAN/rustfetch.1" "$USER_MAN/rfetch.1" 2>/dev/null || true
    fi
fi

# 9. Configure default display mode (Interactive selection: 3D vs Standard 2D)
CONFIG_FILE="$CONFIG_DIR/config.toml"

echo ""
echo -e "${BLUE}==>${NC} Choose default logo display mode:"
echo -e "  ${GREEN}1)${NC} Standard 2D   — Classic fastfetch-style static ASCII (~4ms, recommended for .bashrc) [Default]"
echo -e "  ${GREEN}2)${NC} 3D Animated   — Real-time spinning 3D ASCII relief logo (can also run anytime with 'rfetch --3d')"

MODE_CHOICE="2d"
if [ -n "$RUSTFETCH_MODE" ]; then
    case "$RUSTFETCH_MODE" in
        2|"3d"|"animated"|"three_d") MODE_CHOICE="3d" ;;
        *) MODE_CHOICE="2d" ;;
    esac
elif [ "$CI" = "1" ] || [ "$NONINTERACTIVE" = "1" ] || [ "$DEBIAN_FRONTEND" = "noninteractive" ]; then
    MODE_CHOICE="2d"
elif [ -t 0 ] || [ -c /dev/tty ]; then
    read -rp "Select mode [1-2] (default: 1): " USER_INPUT </dev/tty 2>/dev/null || USER_INPUT="1"
    case "$USER_INPUT" in
        2|"3d"|"animated"|"three_d")
            MODE_CHOICE="3d"
            ;;
        *)
            MODE_CHOICE="2d"
            ;;
    esac
else
    MODE_CHOICE="2d"
fi

if [ "$MODE_CHOICE" = "3d" ]; then
    echo -e "${GREEN}==>${NC} Configured default mode: ${BLUE}3D Animated${NC} (3d = true)"
    if [ ! -f "$CONFIG_FILE" ]; then
        "$FINAL_BIN_DIR/rustfetch" --print-default-config > "$CONFIG_FILE" 2>/dev/null || true
    fi
    if [ -f "$CONFIG_FILE" ]; then
        if grep -q "\[general\.logo\.three_d\]" "$CONFIG_FILE"; then
            sed -i '/\[general\.logo\.three_d\]/,/^\[/ s/enabled = .*/enabled = true/' "$CONFIG_FILE"
        fi
    fi
else
    echo -e "${GREEN}==>${NC} Configured default mode: ${BLUE}Standard 2D${NC} (3d = false)"
    if [ ! -f "$CONFIG_FILE" ]; then
        "$FINAL_BIN_DIR/rustfetch" --print-default-config > "$CONFIG_FILE" 2>/dev/null || true
    fi
    if [ -f "$CONFIG_FILE" ]; then
        if grep -q "\[general\.logo\.three_d\]" "$CONFIG_FILE"; then
            sed -i '/\[general\.logo\.three_d\]/,/^\[/ s/enabled = .*/enabled = false/' "$CONFIG_FILE"
        fi
        if grep -q "3d = true" "$CONFIG_FILE"; then
            sed -i 's/3d = true/3d = false/' "$CONFIG_FILE"
        fi
    fi
fi

# 10. Install automatic responsive shell integration hook (Tiling WMs: Hyprland, MangoWM, Sway, etc.)
if [ -x "$FINAL_BIN_DIR/rustfetch" ]; then
    echo -e "${BLUE}==>${NC} Installing automatic responsive shell hook for tiling window managers..."
    "$FINAL_BIN_DIR/rustfetch" --install-hook 2>/dev/null || true
fi

echo ""
echo -e "${GREEN}==> Installation completed successfully!${NC} 🚀"
echo -e "Binaries installed to: ${BLUE}${FINAL_BIN_DIR}${NC}"
echo -e "You can now run '${GREEN}rustfetch${NC}' or '${GREEN}rfetch${NC}' in your terminal."

if [ "$FINAL_BIN_DIR" = "$USER_BIN" ]; then
    case ":$PATH:" in
        *":$USER_BIN:"*) ;;
        *)
            echo ""
            echo -e "${YELLOW}Note:${NC} '${USER_BIN}' is not in your \$PATH."
            echo -e "Add it by adding this line to your ~/.bashrc or ~/.zshrc:"
            echo -e "  ${BLUE}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
            ;;
    esac
fi
echo ""
