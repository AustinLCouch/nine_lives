#!/usr/bin/env bash
# Build Nine Lives Cat Sudoku for Web using Trunk

set -e  # Exit on error

echo "🐱 Building Nine Lives Cat Sudoku for Web..."

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Make sure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

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

# Build with Trunk using the configured GitHub Pages public URL
echo "📦 Building with Trunk..."
trunk build --release --features web --public-url /nine_lives/

echo "✅ Web build complete!"
echo "📁 Files available in dist/:"
ls -la dist/

echo ""
echo "🚀 To test locally:"
echo "  trunk serve --features web --open"
echo "or:"
echo "  cd dist && python3 -m http.server 8080"
echo "  then visit: http://localhost:8080"
echo ""
echo "🌍 To deploy to GitHub Pages, push to main branch."
echo "    The CI will build and deploy to: https://austinlcouch.github.io/nine_lives/"
