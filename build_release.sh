#!/bin/bash
# Build and package BIOS Simulator for distribution
# Usage: ./build_release.sh [windows|linux|macos|all]

set -e

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
VERSION=$(grep '^version' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
DIST_DIR="$PROJECT_DIR/dist"
BINARY_NAME="bios-simulator"

echo "=== BIOS Simulator v${VERSION} Release Build ==="
echo ""

build_windows() {
    echo "[1/3] Building Windows release..."
    cd "$PROJECT_DIR"
    cargo build --release --target x86_64-pc-windows-gnu 2>/dev/null || cargo build --release

    echo "[2/3] Packaging Windows distribution..."
    local out_dir="$DIST_DIR/windows/${BINARY_NAME}-v${VERSION}-windows"
    rm -rf "$out_dir"
    mkdir -p "$out_dir"

    cp "target/release/${BINARY_NAME}.exe" "$out_dir/" 2>/dev/null || \
    cp "target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe" "$out_dir/" 2>/dev/null || \
    cp "target/x86_64-pc-windows-msvc/release/${BINARY_NAME}.exe" "$out_dir/"

    # Copy assets
    cp -r "$PROJECT_DIR/assets" "$out_dir/"
    cp -r "$PROJECT_DIR/audio_gen" "$out_dir/"
    cp "$PROJECT_DIR/README.md" "$out_dir/" 2>/dev/null || true
    cp "$PROJECT_DIR/LICENSE" "$out_dir/" 2>/dev/null || true

    echo "[3/3] Creating ZIP archive..."
    cd "$DIST_DIR/windows"
    if command -v zip &>/dev/null; then
        zip -r "${BINARY_NAME}-v${VERSION}-windows.zip" "${BINARY_NAME}-v${VERSION}-windows/"
        echo "  Created: ${BINARY_NAME}-v${VERSION}-windows.zip"
    fi

    echo "  Windows build complete: $out_dir"
    echo "  Size: $(du -sh "$out_dir" | cut -f1)"
}

build_linux() {
    echo "[1/3] Building Linux release..."
    cd "$PROJECT_DIR"
    cargo build --release --target x86_64-unknown-linux-gnu 2>/dev/null || cargo build --release

    echo "[2/3] Packaging Linux distribution..."
    local out_dir="$DIST_DIR/linux/${BINARY_NAME}-v${VERSION}-linux"
    rm -rf "$out_dir"
    mkdir -p "$out_dir"

    cp "target/x86_64-unknown-linux-gnu/release/${BINARY_NAME}" "$out_dir/" 2>/dev/null || \
    cp "target/release/${BINARY_NAME}" "$out_dir/" 2>/dev/null || true

    cp -r "$PROJECT_DIR/assets" "$out_dir/"
    cp -r "$PROJECT_DIR/audio_gen" "$out_dir/"
    cp "$PROJECT_DIR/README.md" "$out_dir/" 2>/dev/null || true
    cp "$PROJECT_DIR/LICENSE" "$out_dir/" 2>/dev/null || true

    echo "[3/3] Creating tarball..."
    cd "$DIST_DIR/linux"
    tar czf "${BINARY_NAME}-v${VERSION}-linux.tar.gz" "${BINARY_NAME}-v${VERSION}-linux/"
    echo "  Created: ${BINARY_NAME}-v${VERSION}-linux.tar.gz"
    echo "  Size: $(du -sh "$out_dir" | cut -f1)"
}

build_macos() {
    echo "[1/3] Building macOS release..."
    cd "$PROJECT_DIR"
    cargo build --release --target x86_64-apple-darwin 2>/dev/null || cargo build --release

    echo "[2/3] Packaging macOS distribution..."
    local out_dir="$DIST_DIR/macos/${BINARY_NAME}-v${VERSION}-macos"
    rm -rf "$out_dir"
    mkdir -p "$out_dir"

    cp "target/x86_64-apple-darwin/release/${BINARY_NAME}" "$out_dir/" 2>/dev/null || \
    cp "target/release/${BINARY_NAME}" "$out_dir/" 2>/dev/null || true

    cp -r "$PROJECT_DIR/assets" "$out_dir/"
    cp -r "$PROJECT_DIR/audio_gen" "$out_dir/"
    cp "$PROJECT_DIR/README.md" "$out_dir/" 2>/dev/null || true
    cp "$PROJECT_DIR/LICENSE" "$out_dir/" 2>/dev/null || true

    echo "[3/3] Creating tarball..."
    cd "$DIST_DIR/macos"
    tar czf "${BINARY_NAME}-v${VERSION}-macos.tar.gz" "${BINARY_NAME}-v${VERSION}-macos/"
    echo "  Created: ${BINARY_NAME}-v${VERSION}-macos.tar.gz"
    echo "  Size: $(du -sh "$out_dir" | cut -f1)"
}

case "${1:-windows}" in
    windows)  build_windows ;;
    linux)    build_linux ;;
    macos)    build_macos ;;
    all)
        build_windows
        echo ""
        build_linux
        echo ""
        build_macos
        ;;
    *)
        echo "Usage: $0 [windows|linux|macos|all]"
        exit 1
        ;;
esac

echo ""
echo "=== Build complete ==="
