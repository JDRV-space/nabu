#!/bin/bash
set -euo pipefail

echo "=== Starting Nabu build for Vercel ==="

readonly RUSTUP_VERSION="1.29.0"
readonly RUST_TOOLCHAIN_VERSION="1.97.1"
readonly TRUNK_INSTALLER="./scripts/install-trunk.sh"
TRUNK_VERSION="$("$TRUNK_INSTALLER" --version)"
readonly TRUNK_VERSION

export CARGO_HOME="${CARGO_HOME:-/vercel/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-/vercel/.rustup}"
export PATH="${CARGO_HOME}/bin:${PATH}"

# Install the pinned Rust toolchain manager if not present
if ! command -v rustup &> /dev/null; then
    echo "=== Installing rustup ${RUSTUP_VERSION} ==="
    export RUSTUP_VERSION
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
        sh -s -- -y --no-modify-path --default-toolchain none
fi

echo "=== Installing Rust ${RUST_TOOLCHAIN_VERSION} with wasm target ==="
rustup toolchain install "$RUST_TOOLCHAIN_VERSION" \
    --profile minimal \
    --target wasm32-unknown-unknown
echo "=== Rust version: $(rustc +"$RUST_TOOLCHAIN_VERSION" --version) ==="
echo "=== Cargo version: $(cargo +"$RUST_TOOLCHAIN_VERSION" --version) ==="

# Install and copy browser parser assets
echo "=== Installing browser parser assets ==="
npm ci
npm run prepare:assets

# Install and verify the exact Trunk release artifact
if [[ "$(trunk --version 2>/dev/null || true)" != "trunk ${TRUNK_VERSION}" ]]; then
    echo "=== Installing Trunk ${TRUNK_VERSION} ==="
    "$TRUNK_INSTALLER"
fi

# Build with trunk
echo "=== Building with trunk ==="
env NO_COLOR=true trunk build --release --locked

echo "=== Build complete ==="
ls -la dist/
