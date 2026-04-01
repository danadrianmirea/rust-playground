@echo off
echo 🔨 Building Rust WASM Fractal Generator...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if errorlevel 1 (
    echo ❌ wasm-pack is not installed. Installing...
    cargo install wasm-pack
)

REM First, check if dependencies are installed
echo 📦 Checking dependencies...
cargo check
if %errorlevel% neq 0 (
    echo ⚠️  Dependencies need to be installed...
    cargo build
)

REM Build the WASM module
echo 📦 Building WASM module...
wasm-pack build --target web --out-dir www/pkg

if %errorlevel% equ 0 (
    echo ✅ Build successful!
    echo.
    echo To run the application:
    echo 1. cd www
    echo 2. Start a web server:
    echo    - Python: python -m http.server 8080
    echo    - Node: npx serve .
    echo    - Rust: cargo install basic-http-server ^&^& basic-http-server
    echo 3. Open http://localhost:8080 in your browser
) else (
    echo ❌ Build failed!
    echo.
    echo Troubleshooting tips:
    echo 1. Make sure Rust is installed: rustc --version
    echo 2. Make sure wasm-pack is installed: wasm-pack --version
    echo 3. Try running: cargo clean ^&^& cargo build
    exit /b 1
)
