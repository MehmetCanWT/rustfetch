#!/usr/bin/env bash
# ==============================================================================
# RustFetch — Native Distribution Package Builder (.pkg.tar.zst, .deb, .rpm)
# Usage:
#   ./scripts/build-packages.sh <VERSION> <STAGE_DIR> <OUT_DIR>
# ==============================================================================
set -euo pipefail

VER="${1:?Usage: $0 <version> <stage_dir> <out_dir>}"
VER="${VER#v}"
STAGE_DIR="$(realpath "${2:?}")"
OUT_DIR="$(realpath -m "${3:?}")"
mkdir -p "$OUT_DIR"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

# ------------------------------------------------------------------------------
# 1. Populate FHS root tree shared across package formats
# ------------------------------------------------------------------------------
FHS_ROOT="$WORK_DIR/fhs"
mkdir -p \
    "$FHS_ROOT/usr/bin" \
    "$FHS_ROOT/usr/share/man/man1" \
    "$FHS_ROOT/usr/share/bash-completion/completions" \
    "$FHS_ROOT/usr/share/zsh/site-functions" \
    "$FHS_ROOT/usr/share/fish/vendor_completions.d" \
    "$FHS_ROOT/usr/share/licenses/rustfetch" \
    "$FHS_ROOT/usr/share/doc/rustfetch"

install -m 755 "$STAGE_DIR/rustfetch" "$FHS_ROOT/usr/bin/rustfetch"
install -m 755 "$STAGE_DIR/rfetch" "$FHS_ROOT/usr/bin/rfetch"

if [[ -f "$STAGE_DIR/man/rustfetch.1" ]]; then
    install -m 644 "$STAGE_DIR/man/rustfetch.1" "$FHS_ROOT/usr/share/man/man1/rustfetch.1"
    ln -sf rustfetch.1 "$FHS_ROOT/usr/share/man/man1/rfetch.1"
fi

if [[ -f "$STAGE_DIR/completions/rustfetch.bash" ]]; then
    install -m 644 "$STAGE_DIR/completions/rustfetch.bash" "$FHS_ROOT/usr/share/bash-completion/completions/rustfetch"
    sed 's/rustfetch/rfetch/g' "$STAGE_DIR/completions/rustfetch.bash" > "$FHS_ROOT/usr/share/bash-completion/completions/rfetch"
fi

if [[ -f "$STAGE_DIR/completions/_rustfetch" ]]; then
    install -m 644 "$STAGE_DIR/completions/_rustfetch" "$FHS_ROOT/usr/share/zsh/site-functions/_rustfetch"
    sed 's/rustfetch/rfetch/g' "$STAGE_DIR/completions/_rustfetch" > "$FHS_ROOT/usr/share/zsh/site-functions/_rfetch"
fi

if [[ -f "$STAGE_DIR/completions/rustfetch.fish" ]]; then
    install -m 644 "$STAGE_DIR/completions/rustfetch.fish" "$FHS_ROOT/usr/share/fish/vendor_completions.d/rustfetch.fish"
    sed 's/complete -c rustfetch/complete -c rfetch/g' "$STAGE_DIR/completions/rustfetch.fish" > "$FHS_ROOT/usr/share/fish/vendor_completions.d/rfetch.fish"
fi

[[ -f "$STAGE_DIR/LICENSE" ]] && install -m 644 "$STAGE_DIR/LICENSE" "$FHS_ROOT/usr/share/licenses/rustfetch/LICENSE"
[[ -f "$STAGE_DIR/README.md" ]] && install -m 644 "$STAGE_DIR/README.md" "$FHS_ROOT/usr/share/doc/rustfetch/README.md"
[[ -f "$STAGE_DIR/CHANGELOG.md" ]] && install -m 644 "$STAGE_DIR/CHANGELOG.md" "$FHS_ROOT/usr/share/doc/rustfetch/CHANGELOG.md"

# ------------------------------------------------------------------------------
# 2. Build Arch Linux Package (.pkg.tar.zst)
# ------------------------------------------------------------------------------
if command -v zstd &>/dev/null; then
    ARCH_ROOT="$WORK_DIR/arch"
    cp -a "$FHS_ROOT" "$ARCH_ROOT"
    INSTALLED_BYTES="$(du -sb "$ARCH_ROOT/usr" | awk '{print $1}')"
    BUILD_DATE="$(date +%s)"

    cat > "$ARCH_ROOT/.PKGINFO" <<EOF
pkgname = rustfetch
pkgbase = rustfetch
pkgver = ${VER}-1
pkgdesc = Blazing-fast, memory-safe system fetch tool written in Rust
url = https://github.com/MehmetCanWT/rustfetch
builddate = ${BUILD_DATE}
packager = MehmetCanWT <https://github.com/MehmetCanWT/rustfetch>
size = ${INSTALLED_BYTES}
arch = x86_64
license = MIT
provides = rustfetch
provides = rfetch
conflict = rustfetch-git
conflict = rustfetch-bin
depend = gcc-libs
optdepend = kitty: Kitty graphics protocol image rendering
optdepend = wireplumber: Sound module support
optdepend = playerctl: MPRIS media module support
EOF

    ARCH_PKG="$OUT_DIR/rustfetch-v${VER}-arch-x86_64.pkg.tar.zst"
    tar --zstd -C "$ARCH_ROOT" -cf "$ARCH_PKG" .PKGINFO usr
    echo "==> Built Arch package: $ARCH_PKG"
fi

# ------------------------------------------------------------------------------
# 3. Build Debian / Ubuntu Package (.deb)
# ------------------------------------------------------------------------------
if command -v dpkg-deb &>/dev/null; then
    DEB_ROOT="$WORK_DIR/deb"
    cp -a "$FHS_ROOT" "$DEB_ROOT"
    mkdir -p "$DEB_ROOT/DEBIAN"
    INSTALLED_KB="$(du -sk "$DEB_ROOT/usr" | awk '{print $1}')"

    cat > "$DEB_ROOT/DEBIAN/control" <<EOF
Package: rustfetch
Version: ${VER}
Section: utils
Priority: optional
Architecture: amd64
Installed-Size: ${INSTALLED_KB}
Maintainer: MehmetCanWT <https://github.com/MehmetCanWT/rustfetch>
Homepage: https://github.com/MehmetCanWT/rustfetch
Provides: rustfetch, rfetch
Description: Blazing-fast, memory-safe system fetch tool written in Rust
 RustFetch gathers system telemetry directly from Linux kernel interfaces
 (/proc and /sys) with zero subprocess forks, supporting Kitty, Sixel,
 Halfblock graphics, and a real-time 3D Z-buffer ASCII relief engine.
EOF

    DEB_PKG="$OUT_DIR/rustfetch_${VER}_debian_amd64.deb"
    dpkg-deb --root-owner-group --build "$DEB_ROOT" "$DEB_PKG"
    echo "==> Built Debian package: $DEB_PKG"
fi

# ------------------------------------------------------------------------------
# 4. Build Fedora / RHEL / Bazzite / openSUSE Package (.rpm)
# ------------------------------------------------------------------------------
if command -v rpmbuild &>/dev/null; then
    RPM_TOP="$WORK_DIR/rpmbuild"
    mkdir -p "$RPM_TOP"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

    SPEC_FILE="$RPM_TOP/SPECS/rustfetch-bin.spec"
    cat > "$SPEC_FILE" <<EOF
%global debug_package %{nil}
%define _build_id_links none

Name:           rustfetch
Version:        ${VER}
Release:        1.fedora
Summary:        Blazing-fast, memory-safe system fetch tool written in Rust
License:        MIT
URL:            https://github.com/MehmetCanWT/rustfetch
Provides:       rustfetch = %{version}-%{release}
Provides:       rfetch = %{version}-%{release}

%description
RustFetch is a high-performance, modular system information tool written in pure
Rust with Kitty, Sixel, Halfblock, and 3D ASCII relief rendering.

%install
cp -a ${FHS_ROOT}/usr %{buildroot}/usr

%files
/usr/bin/rustfetch
/usr/bin/rfetch
/usr/share/man/man1/rustfetch.1*
/usr/share/man/man1/rfetch.1*
/usr/share/bash-completion/completions/rustfetch
/usr/share/bash-completion/completions/rfetch
/usr/share/zsh/site-functions/_rustfetch
/usr/share/zsh/site-functions/_rfetch
/usr/share/fish/vendor_completions.d/rustfetch.fish
/usr/share/fish/vendor_completions.d/rfetch.fish
/usr/share/licenses/rustfetch/LICENSE
/usr/share/doc/rustfetch/*
EOF

    rpmbuild --define "_topdir $RPM_TOP" -bb "$SPEC_FILE"
    RPM_BUILT="$(find "$RPM_TOP/RPMS" -name "*.rpm" | head -n 1)"
    if [[ -n "$RPM_BUILT" && -f "$RPM_BUILT" ]]; then
        RPM_OUT="$OUT_DIR/rustfetch-${VER}-1.fedora.x86_64.rpm"
        cp -f "$RPM_BUILT" "$RPM_OUT"
        echo "==> Built Fedora/RPM package: $RPM_OUT"
    fi
fi
