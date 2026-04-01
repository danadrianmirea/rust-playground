# Rust WASM Fractal Generator

A WebAssembly fractal generator built with Rust, featuring interactive Mandelbrot and Julia sets.

## Features

- **Mandelbrot Set** - The classic fractal with real-time rendering
- **Julia Set** - Interactive Julia fractals with adjustable parameters
- **Interactive Controls** - Pan, zoom, and adjust parameters in real-time
- **Performance Metrics** - See render times for Rust computation
- **Beautiful UI** - Modern, responsive interface with gradient colors
- **WebAssembly** - All fractal computation happens in Rust, compiled to WASM

## Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **wasm-pack** - Install with: `cargo install wasm-pack`
3. **Node.js** (optional) - For serving the web page locally

## Building the Project

### 1. Build the WASM module

```bash
# Install wasm-bindgen-cli if not already installed
cargo install wasm-bindgen-cli

# Build the project
wasm-pack build --target web --out-dir www/pkg
```

### 2. Serve the web page

You can use any static file server. Here are a few options:

**Option A: Using Python (simplest)**
```bash
cd www
python -m http.server 8080
```

**Option B: Using Node.js with http-server**
```bash
npm install -g http-server
cd www
http-server
```

**Option C: Using Rust's basic-http-server**
```bash
cargo install basic-http-server
cd www
basic-http-server
```

### 3. Open in browser

Navigate to `http://localhost:8080` (or whatever port your server uses)

## Project Structure

```
fractal-wasm/
├── Cargo.toml           # Rust dependencies and configuration
├── src/
│   └── lib.rs          # Main Rust/WASM code
├── www/                 # Web frontend
│   ├── index.html      # Main HTML page
│   ├── style.css       # Styling
│   ├── app.js          # JavaScript application logic
│   └── pkg/            # Generated WASM package (after build)
└── README.md           # This file
```

## How It Works

### Rust Side (`src/lib.rs`)
- **FractalRenderer struct** - Main rendering engine
- **Mandelbrot/Julia algorithms** - Pure Rust implementations
- **Color palette generation** - Creates rainbow gradients
- **WASM bindings** - Exposes Rust functions to JavaScript

### JavaScript Side (`www/app.js`)
- **FractalApp class** - Manages application state
- **UI event handling** - Controls, mouse interactions
- **Canvas rendering** - Displays the fractal
- **WASM module loading** - Loads and initializes Rust code

### Web Interface (`www/index.html`)
- **Interactive controls** - Sliders, buttons, dropdowns
- **Canvas element** - Where the fractal is drawn
- **Responsive design** - Works on desktop and mobile

## Key Rust Concepts Demonstrated

1. **WebAssembly Integration**
   - Using `wasm-bindgen` for Rust↔JavaScript interop
   - Memory management between Rust and JavaScript
   - Performance optimization for WASM

2. **Algorithm Implementation**
   - Complex number arithmetic
   - Iterative fractal algorithms
   - Color mapping and palette generation

3. **Error Handling**
   - Result types for fallible operations
   - Proper error propagation

4. **Performance**
   - Efficient loops and calculations
   - Minimal allocations in hot paths
   - Benchmarking capabilities

## Interactive Features

- **Click and drag** to pan around the fractal
- **Mouse wheel** to zoom in/out
- **Adjust sliders** for iterations and zoom level
- **Switch between** Mandelbrot and Julia sets
- **Quick examples** for interesting fractal locations
- **Benchmark** to test Rust computation speed

## Performance Tips

1. **Lower iteration counts** render faster
2. **Higher zoom levels** show more detail but render slower
3. **The benchmark** shows pure Rust computation time (excluding canvas rendering)

## Learning Resources

- [The Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [MDN WebAssembly Documentation](https://developer.mozilla.org/en-US/docs/WebAssembly)

## License

MIT License - Feel free to use, modify, and distribute this project.

## Contributing

This is a learning project. Feel free to:
- Add new fractal types (Burning Ship, Newton's Method, etc.)
- Implement multi-threading with Web Workers
- Add color palette options
- Improve the UI/UX
- Optimize the algorithms further