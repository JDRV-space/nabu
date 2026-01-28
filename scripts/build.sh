#!/bin/bash
set -e

# Install Rust if not present
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install trunk if not present
if ! command -v trunk &> /dev/null; then
    cargo install trunk
fi

# Build the project
trunk build --release
