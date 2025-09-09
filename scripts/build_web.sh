#!/bin/bash
# Build Nine Lives Cat Sudoku for Web
# This script builds the WASM version of the game for web deployment using Trunk

set -e  # Exit on error

echo "🐱 Building Nine Lives Cat Sudoku for Web..."

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Make sure cargo is in PATH
export PATH="/Users/austincouch/.cargo/bin:$PATH"

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "📥 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "📥 Installing Trunk..."
    cargo install trunk
fi

# Navigate to controller directory and build with Trunk
echo "📦 Building with Trunk..."
cd nine_lives_controller
trunk build --features web --release

# Copy files to project root for easy deployment
echo "📋 Copying files to project root..."
cd ..
cp -r nine_lives_controller/dist/* .

echo "✅ Web build complete!"
echo "📁 Files available in project root:"
ls -la *.html *.js *.wasm 2>/dev/null || echo "No web files found"

echo ""
echo "🚀 To test locally:"
echo "  cd nine_lives_controller && trunk serve --features web --open"
echo ""
echo "🌍 To deploy, upload index.html and related files to your web host."
