#!/bin/bash
set -euo pipefail

readonly RUSTUP_VERSION="1.29.0"
readonly RUST_TOOLCHAIN_VERSION="1.97.1"
readonly TRUNK_INSTALLER="./scripts/install-trunk.sh"
TRUNK_VERSION="$("$TRUNK_INSTALLER" --version)"
readonly TRUNK_VERSION

export CARGO_HOME="${CARGO_HOME:-${HOME}/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-${HOME}/.rustup}"
export PATH="${CARGO_HOME}/bin:${PATH}"

# Install the pinned Rust toolchain manager if not present
if ! command -v rustup &> /dev/null; then
    export RUSTUP_VERSION
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
        sh -s -- -y --no-modify-path --default-toolchain none
fi

# Install the compiler and target declared in rust-toolchain.toml
rustup toolchain install "$RUST_TOOLCHAIN_VERSION" \
    --profile minimal \
    --target wasm32-unknown-unknown

# Install and copy browser parser assets
npm ci
npm run prepare:assets

# Install and verify the exact Trunk release used by deploy builds
if [[ "$(trunk --version 2>/dev/null || true)" != "trunk ${TRUNK_VERSION}" ]]; then
    "$TRUNK_INSTALLER"
fi

# Build the project
env NO_COLOR=true trunk build --release --locked
