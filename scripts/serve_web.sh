#!/bin/bash
# Serve Nine Lives Cat Sudoku Web Version
# This script starts a local web server for testing the web build

set -e  # Exit on error

echo "🚀 Starting local web server for Nine Lives Cat Sudoku..."

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Check if web build exists
if [ ! -f "web/nine_lives.js" ]; then
    echo "❌ Web build not found!"
    echo "Run './scripts/build_web.sh' first to build the web version."
    exit 1
fi

echo "📁 Serving from 'web/' directory"
echo "🌍 Open your browser to: http://localhost:8080"
echo "🛑 Press Ctrl+C to stop the server"
echo ""

# Start Python HTTP server in the web directory
cd web
python3 -m http.server 8080
