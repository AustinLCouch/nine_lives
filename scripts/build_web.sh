#!/bin/bash
# Build Nine Lives Cat Sudoku for Web
# This script builds the WASM version of the game for web deployment

set -e  # Exit on error

echo "🐱 Building Nine Lives Cat Sudoku for Web..."

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Make sure rustup cargo is in PATH first (needed for WASM compilation)
export PATH="/Users/austincouch/.cargo/bin:$PATH"

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli --version 0.2.100
fi

# Build the Rust project for WASM
echo "📦 Building Rust project for WASM..."
cargo build --release --target wasm32-unknown-unknown --bin nine_lives --features web

# Generate JavaScript bindings
echo "🔧 Generating JavaScript bindings..."
wasm-bindgen --out-dir web --target web --no-typescript target/wasm32-unknown-unknown/release/nine_lives.wasm

echo "✅ Web build complete!"
echo "📁 Files generated in 'web/' directory:"
ls -la web/

echo ""
echo "🚀 To test locally, run:"
echo "  ./scripts/serve_web.sh"
echo ""
echo "🌍 To deploy, upload the 'web/' directory to your web host."
