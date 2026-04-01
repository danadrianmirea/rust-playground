#!/bin/bash

# Build script for Rust WASM Fractal Generator

echo "🔨 Building Rust WASM Fractal Generator..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build the WASM module
echo "📦 Building WASM module..."
wasm-pack build --target web --out-dir www/pkg

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "To run the application:"
    echo "1. cd www"
    echo "2. Start a web server:"
    echo "   - Python: python -m http.server 8080"
    echo "   - Node: npx serve ."
    echo "   - Rust: cargo install basic-http-server && basic-http-server"
    echo "3. Open http://localhost:8080 in your browser"
else
    echo "❌ Build failed!"
    exit 1
fi