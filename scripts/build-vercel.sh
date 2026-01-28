#!/bin/bash
set -e

echo "=== Starting Nabu build for Vercel ==="

# Set HOME to /vercel to fix rustup directory issues
export HOME=/vercel

# Install Rust if not present
if [ ! -f "$HOME/.cargo/env" ]; then
    echo "=== Installing Rust ==="
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
fi

# Source cargo environment
. "$HOME/.cargo/env"

echo "=== Rust version: $(rustc --version) ==="
echo "=== Cargo version: $(cargo --version) ==="

# Add wasm target
echo "=== Adding wasm32-unknown-unknown target ==="
rustup target add wasm32-unknown-unknown

# Install trunk via cargo (compatible with Vercel's glibc)
echo "=== Installing trunk ==="
cargo install trunk --locked

# Build with trunk
echo "=== Building with trunk ==="
trunk build --release

echo "=== Build complete ==="
ls -la dist/
