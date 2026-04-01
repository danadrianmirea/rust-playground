// Main application for Rust WASM Fractal Generator

class FractalApp {
    constructor() {
        this.wasmModule = null;
        this.renderer = null;
        this.isRendering = false;
        
        // View state
        this.centerX = -0.5;
        this.centerY = 0;
        this.zoom = 1.0;
        this.fractalType = 'mandelbrot';
        this.maxIterations = 100;
        
        // Julia parameters
        this.juliaReal = -0.7;
        this.juliaImag = 0.27015;
        
        // Drag state
        this.isDragging = false;
        this.lastX = 0;
        this.lastY = 0;
        
        this.init();
    }
    
    async init() {
        try {
            console.log('Loading WebAssembly module...');
            
            // Import the WASM module
            const wasm = await import('../pkg/fractal_wasm.js');
            await wasm.default();
            
            this.wasmModule = wasm;
            console.log('WebAssembly module loaded successfully!');
            
            // Initialize renderer with canvas dimensions
            const canvas = document.getElementById('fractalCanvas');
            this.renderer = new wasm.FractalRenderer(canvas.width, canvas.height);
            
            // Hide loading message
            document.getElementById('loading').style.display = 'none';
            
            // Setup UI event listeners
            this.setupEventListeners();
            
            // Render initial fractal
            this.renderFractal();
            
            // Test greeting
            const greeting = wasm.greet('Rust Developer');
            console.log(greeting);
            
        } catch (error) {
            console.error('Failed to load WebAssembly module:', error);
            document.getElementById('loading').textContent = 
                'Failed to load WebAssembly. Please check console for errors.';
        }
    }
    
    setupEventListeners() {
        const canvas = document.getElementById('fractalCanvas');
        
        // Render button
        document.getElementById('renderBtn').addEventListener('click', () => {
            this.renderFractal();
        });
        
        // Reset button
        document.getElementById('resetBtn').addEventListener('click', () => {
            this.centerX = -0.5;
            this.centerY = 0;
            this.zoom = 1.0;
            this.updateCoordinatesDisplay();
            this.renderFractal();
        });
        
        // Benchmark button
        document.getElementById('benchmarkBtn').addEventListener('click', () => {
            this.runBenchmark();
        });
        
        // Fractal type selector
        document.getElementById('fractalType').addEventListener('change', (e) => {
            this.fractalType = e.target.value;
            const juliaControls = document.querySelector('.julia-controls');
            juliaControls.style.display = this.fractalType === 'julia' ? 'block' : 'none';
            this.renderFractal();
        });
        
        // Iterations slider
        const iterationsSlider = document.getElementById('iterations');
        const iterationsValue = document.getElementById('iterationsValue');
        
        iterationsSlider.addEventListener('input', (e) => {
            this.maxIterations = parseInt(e.target.value);
            iterationsValue.textContent = this.maxIterations;
            if (this.renderer) {
                this.renderer.set_max_iterations(this.maxIterations);
            }
        });
        
        // Zoom slider
        const zoomSlider = document.getElementById('zoom');
        const zoomValue = document.getElementById('zoomValue');
        
        zoomSlider.addEventListener('input', (e) => {
            this.zoom = parseInt(e.target.value) / 100;
            zoomValue.textContent = this.zoom.toFixed(2) + 'x';
        });
        
        // Julia parameter sliders
        const juliaRealSlider = document.getElementById('juliaReal');
        const juliaRealValue = document.getElementById('juliaRealValue');
        const juliaImagSlider = document.getElementById('juliaImag');
        const juliaImagValue = document.getElementById('juliaImagValue');
        
        juliaRealSlider.addEventListener('input', (e) => {
            this.juliaReal = parseInt(e.target.value) / 100;
            juliaRealValue.textContent = this.juliaReal.toFixed(2);
        });
        
        juliaImagSlider.addEventListener('input', (e) => {
            this.juliaImag = parseInt(e.target.value) / 100;
            juliaImagValue.textContent = this.juliaImag.toFixed(2);
        });
        
        // Example buttons
        document.querySelectorAll('.example').forEach(button => {
            button.addEventListener('click', (e) => {
                const x = parseFloat(e.target.dataset.x);
                const y = parseFloat(e.target.dataset.y);
                const zoom = parseFloat(e.target.dataset.zoom);
                const type = e.target.dataset.type || 'mandelbrot';
                
                this.centerX = x;
                this.centerY = y;
                this.zoom = zoom;
                this.fractalType = type;
                
                // Update UI
                document.getElementById('fractalType').value = type;
                document.getElementById('zoom').value = Math.round(zoom * 100);
                document.getElementById('zoomValue').textContent = zoom.toFixed(2) + 'x';
                
                this.updateCoordinatesDisplay();
                this.renderFractal();
            });
        });
        
        // Mouse interactions for canvas
        canvas.addEventListener('mousedown', (e) => {
            this.isDragging = true;
            this.lastX = e.clientX;
            this.lastY = e.clientY;
            canvas.style.cursor = 'grabbing';
        });
        
        canvas.addEventListener('mousemove', (e) => {
            if (!this.isDragging) return;
            
            const dx = e.clientX - this.lastX;
            const dy = e.clientY - this.lastY;
            
            // Convert pixel movement to fractal coordinate movement
            const scale = 3.0 / this.zoom;
            const aspectRatio = canvas.width / canvas.height;
            
            this.centerX -= (dx / canvas.width) * scale * aspectRatio;
            this.centerY -= (dy / canvas.height) * scale;
            
            this.lastX = e.clientX;
            this.lastY = e.clientY;
            
            this.updateCoordinatesDisplay();
            this.renderFractal();
        });
        
        canvas.addEventListener('mouseup', () => {
            this.isDragging = false;
            canvas.style.cursor = 'crosshair';
        });
        
        canvas.addEventListener('mouseleave', () => {
            this.isDragging = false;
            canvas.style.cursor = 'crosshair';
        });
        
        // Mouse wheel for zoom
        canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            
            const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
            this.zoom *= zoomFactor;
            
            // Update zoom slider
            const zoomSlider = document.getElementById('zoom');
            const zoomValue = document.getElementById('zoomValue');
            zoomSlider.value = Math.round(this.zoom * 100);
            zoomValue.textContent = this.zoom.toFixed(2) + 'x';
            
            this.updateCoordinatesDisplay();
            this.renderFractal();
        });
    }
    
    async renderFractal() {
        if (!this.renderer || this.isRendering) return;
        
        this.isRendering = true;
        const startTime = performance.now();
        
        try {
            await this.renderer.render_to_canvas(
                'fractalCanvas',
                this.centerX,
                this.centerY,
                this.zoom,
                this.fractalType
            );
            
            const renderTime = performance.now() - startTime;
            document.getElementById('renderTime').textContent = 
                `Render time: ${renderTime.toFixed(2)} ms`;
                
        } catch (error) {
            console.error('Error rendering fractal:', error);
            document.getElementById('renderTime').textContent = 
                `Error: ${error.message}`;
        } finally {
            this.isRendering = false;
        }
    }
    
    updateCoordinatesDisplay() {
        document.getElementById('coordinates').textContent = 
            `Center: (${this.centerX.toFixed(4)}, ${this.centerY.toFixed(4)}) Zoom: ${this.zoom.toFixed(2)}x`;
    }
    
    async runBenchmark() {
        if (!this.wasmModule) return;
        
        const canvas = document.getElementById('fractalCanvas');
        const benchmarkTime = this.wasmModule.benchmark_fractal(canvas.width, canvas.height);
        
        alert(`Benchmark completed in ${benchmarkTime.toFixed(2)} ms\n` +
              `Resolution: ${canvas.width}x${canvas.height}\n` +
              `This measures only the Rust computation time.`);
    }
}

// Initialize the app when the page loads
window.addEventListener('DOMContentLoaded', () => {
    window.fractalApp = new FractalApp();
});

// Utility function to load WASM (alternative method)
async function loadWasm() {
    try {
        const response = await fetch('../pkg/fractal_wasm_bg.wasm');
        const bytes = await response.arrayBuffer();
        const { instance } = await WebAssembly.instantiate(bytes, {
            './fractal_wasm.js': {
                // Import any required functions here
            }
        });
        return instance;
    } catch (error) {
        console.error('WASM instantiation failed:', error);
        throw error;
    }
}