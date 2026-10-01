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

# 2. Check for local build first (if running inside repository)
if [ -f "Cargo.toml" ] && grep -q "rustfetch" Cargo.toml; then
    BUILD_DIR="$(pwd)"
    echo -e "${BLUE}==>${NC} Compiling local release binary (LTO optimized)..."
    cargo build --release -p rustfetch
elif [ -n "$TARGET_ARCH" ] && [ -z "$RUSTFETCH_BUILD_FROM_SOURCE" ]; then
    # Try downloading pre-compiled release binary from GitHub Releases (instant: <2s)
    if [ "$IS_MUSL" = true ]; then
        RELEASE_TARGET="${TARGET_ARCH}-unknown-linux-musl"
    else
        RELEASE_TARGET="${TARGET_ARCH}-unknown-linux-gnu"
    fi

    TARBALL_URL="${GITHUB_URL}/releases/latest/download/rustfetch-${RELEASE_TARGET}.tar.gz"
    echo -e "${BLUE}==>${NC} Checking for pre-compiled binary for ${RELEASE_TARGET}..."

    if curl -sSfL "$TARBALL_URL" -o "$TEMP_DIR/rustfetch.tar.gz" 2>/dev/null; then
        echo -e "${GREEN}==>${NC} Downloaded pre-compiled release binary successfully."
        tar -xzf "$TEMP_DIR/rustfetch.tar.gz" -C "$TEMP_DIR"
        BIN_FOUND="$(find "$TEMP_DIR" -type f -name rustfetch -perm -111 | head -n 1)"
        if [ -n "$BIN_FOUND" ]; then
            BUILD_DIR="$(dirname "$BIN_FOUND")"
            INSTALLED_FROM_BINARY=true
        fi
    fi
fi

# 3. Fallback: Build from source with Cargo if binary not found
if [ -z "$BUILD_DIR" ]; then
    echo -e "${YELLOW}==>${NC} Building from source via Cargo..."

    if ! command -v cargo &>/dev/null; then
        if [ -f "$HOME/.cargo/env" ]; then
            . "$HOME/.cargo/env"
        fi
    fi

    if ! command -v cargo &>/dev/null; then
        echo -e "${YELLOW}==>${NC} Cargo is not installed. Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
        . "$HOME/.cargo/env" 2>/dev/null || true
    fi

    # Check for C compiler/linker
    if ! command -v cc &>/dev/null && ! command -v gcc &>/dev/null && ! command -v clang &>/dev/null; then
        echo -e "${YELLOW}==>${NC} C compiler not found. Attempting to install build tools..."
        if command -v apt-get &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty apt-get update -qq && sudo </dev/tty apt-get install -y -qq build-essential || true
        elif command -v dnf &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty dnf install -y -q gcc || true
        elif command -v pacman &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty pacman -Sy --noconfirm base-devel || true
        elif command -v apk &>/dev/null && [ -c /dev/tty ]; then
            sudo </dev/tty apk add gcc musl-dev || true
        fi
    fi

    SRC_DIR="$TEMP_DIR/source"
    echo -e "${BLUE}==>${NC} Cloning repository into ${SRC_DIR}..."
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
ln -sf "$FINAL_BIN_DIR/rustfetch" "$FINAL_BIN_DIR/rfetch"
chmod +x "$FINAL_BIN_DIR/rustfetch"

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
    ln -sf "$BASH_DIR/rustfetch" "$BASH_DIR/rfetch" 2>/dev/null || true

    # Zsh
    ZSH_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zsh/site-functions"
    mkdir -p "$ZSH_DIR"
    "$INSTALLED_BIN" --completions zsh > "$ZSH_DIR/_rustfetch" 2>/dev/null || true
    ln -sf "$ZSH_DIR/_rustfetch" "$ZSH_DIR/_rfetch" 2>/dev/null || true

    # Fish
    FISH_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions"
    mkdir -p "$FISH_DIR"
    "$INSTALLED_BIN" --completions fish > "$FISH_DIR/rustfetch.fish" 2>/dev/null || true
    ln -sf "$FISH_DIR/rustfetch.fish" "$FISH_DIR/rfetch.fish" 2>/dev/null || true
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
if [ -t 0 ] || [ -c /dev/tty ]; then
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
    if [ -f "$CONFIG_FILE" ]; then
        if grep -q "\[general\.logo\.three_d\]" "$CONFIG_FILE"; then
            sed -i '/\[general\.logo\.three_d\]/,/^\[/ s/enabled = .*/enabled = true/' "$CONFIG_FILE"
        else
            cat << 'EOF' >> "$CONFIG_FILE"

[general.logo.three_d]
enabled = true
EOF
        fi
    else
        cat << 'EOF' > "$CONFIG_FILE"
[general]
separator = ":"
padding = 1
center = true
icons = true
border = false

[general.colors]
enabled = true
symbol = "●"
block = false

[general.logo]
enabled = true
distro = "auto"
3d = true

[general.logo.three_d]
enabled = true
speed = 1.0
shading_mode = "ascii"
EOF
    fi
else
    echo -e "${GREEN}==>${NC} Configured default mode: ${BLUE}Standard 2D${NC} (3d = false)"
    if [ -f "$CONFIG_FILE" ]; then
        if grep -q "\[general\.logo\.three_d\]" "$CONFIG_FILE"; then
            sed -i '/\[general\.logo\.three_d\]/,/^\[/ s/enabled = .*/enabled = false/' "$CONFIG_FILE"
        fi
        if grep -q "3d = true" "$CONFIG_FILE"; then
            sed -i 's/3d = true/3d = false/' "$CONFIG_FILE"
        fi
    else
        cat << 'EOF' > "$CONFIG_FILE"
[general]
separator = ":"
padding = 1
center = true
icons = true
border = false

[general.colors]
enabled = true
symbol = "●"
block = false

[general.logo]
enabled = true
distro = "auto"
3d = false

[general.logo.three_d]
enabled = false
EOF
    fi
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
