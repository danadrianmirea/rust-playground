use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Date;

// Include our fractal algorithms module
mod fractal;
use fractal::{Complex, mandelbrot_iteration, julia_iteration};

#[wasm_bindgen]
pub struct FractalRenderer {
    width: u32,
    height: u32,
    max_iterations: u32,
    color_palette: Vec<[u8; 3]>,
}

#[wasm_bindgen]
impl FractalRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> FractalRenderer {
        console::log_1(&format!("Creating FractalRenderer {}x{}", width, height).into());
        
        // Create a color palette (rainbow colors)
        let color_palette = Self::create_rainbow_palette(256);
        
        FractalRenderer {
            width,
            height,
            max_iterations: 100,
            color_palette,
        }
    }
    
    #[wasm_bindgen]
    pub fn set_max_iterations(&mut self, iterations: u32) {
        self.max_iterations = iterations;
        console::log_1(&format!("Max iterations set to {}", iterations).into());
    }
    
    #[wasm_bindgen]
    pub fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }
    
    #[wasm_bindgen]
    pub fn render_mandelbrot(&self, center_x: f64, center_y: f64, zoom: f64) -> Vec<u8> {
        console::time_with_label("render_mandelbrot");
        
        let mut pixels = vec![0; (self.width * self.height * 4) as usize];
        let aspect_ratio = self.width as f64 / self.height as f64;
        
        // Calculate bounds based on center and zoom
        let scale = 3.0 / zoom;
        let x_min = center_x - scale * aspect_ratio;
        let x_max = center_x + scale * aspect_ratio;
        let y_min = center_y - scale;
        let y_max = center_y + scale;
        
        for y in 0..self.height {
            let cy = y_min + (y as f64 / self.height as f64) * (y_max - y_min);
            
            for x in 0..self.width {
                let cx = x_min + (x as f64 / self.width as f64) * (x_max - x_min);
                
                let c = Complex::new(cx, cy);
                let iterations = mandelbrot_iteration(&c, self.max_iterations);
                let color = self.get_color(iterations);
                
                let idx = ((y * self.width + x) * 4) as usize;
                pixels[idx] = color[0];     // R
                pixels[idx + 1] = color[1]; // G
                pixels[idx + 2] = color[2]; // B
                pixels[idx + 3] = 255;      // A (fully opaque)
            }
        }
        
        console::time_end_with_label("render_mandelbrot");
        console::log_1(&format!("Rendered {} pixels", self.width * self.height).into());
        
        pixels
    }
    
    #[wasm_bindgen]
    pub fn render_julia(&self, c_real: f64, c_imag: f64, center_x: f64, center_y: f64, zoom: f64) -> Vec<u8> {
        console::time_with_label("render_julia");
        
        let mut pixels = vec![0; (self.width * self.height * 4) as usize];
        let aspect_ratio = self.width as f64 / self.height as f64;
        
        // Calculate bounds based on center and zoom
        let scale = 3.0 / zoom;
        let x_min = center_x - scale * aspect_ratio;
        let x_max = center_x + scale * aspect_ratio;
        let y_min = center_y - scale;
        let y_max = center_y + scale;
        
        let c = Complex::new(c_real, c_imag);
        
        for y in 0..self.height {
            let zy = y_min + (y as f64 / self.height as f64) * (y_max - y_min);
            
            for x in 0..self.width {
                let zx = x_min + (x as f64 / self.width as f64) * (x_max - x_min);
                
                let z = Complex::new(zx, zy);
                let iterations = julia_iteration(&z, &c, self.max_iterations);
                let color = self.get_color(iterations);
                
                let idx = ((y * self.width + x) * 4) as usize;
                pixels[idx] = color[0];
                pixels[idx + 1] = color[1];
                pixels[idx + 2] = color[2];
                pixels[idx + 3] = 255;
            }
        }
        
        console::time_end_with_label("render_julia");
        pixels
    }
    
    #[wasm_bindgen]
    pub fn render_to_canvas(&self, canvas_id: &str, center_x: f64, center_y: f64, zoom: f64, fractal_type: &str) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect(&format!("canvas with id '{}' not found", canvas_id))
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a canvas"))?;
        
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        
        // Get canvas dimensions
        let width = canvas.width();
        let _height = canvas.height(); // Prefix with underscore to avoid unused warning
        
        // Render the fractal
        let pixels = match fractal_type {
            "mandelbrot" => self.render_mandelbrot(center_x, center_y, zoom),
            "julia" => self.render_julia(-0.7, 0.27015, center_x, center_y, zoom), // Default Julia parameters
            _ => return Err(JsValue::from_str("Unknown fractal type")),
        };
        
        // Create ImageData and put it on canvas
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(
            wasm_bindgen::Clamped(&pixels),
            width,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create ImageData: {:?}", e)))?;
        
        context.put_image_data(&image_data, 0.0, 0.0)
            .map_err(|e| JsValue::from_str(&format!("Failed to put image data: {:?}", e)))?;
        
        Ok(())
    }
}

impl FractalRenderer {
    fn get_color(&self, iteration: u32) -> [u8; 3] {
        if iteration == self.max_iterations {
            return [0, 0, 0]; // Black for points inside the set
        }
        
        // Map iteration to color palette
        let color_idx = (iteration % self.color_palette.len() as u32) as usize;
        self.color_palette[color_idx]
    }
    
    fn create_rainbow_palette(size: usize) -> Vec<[u8; 3]> {
        let mut palette = Vec::with_capacity(size);
        
        for i in 0..size {
            let t = i as f64 / size as f64;
            
            // Create rainbow colors using sine waves
            let r = (0.5 * (2.0 * std::f64::consts::PI * t).sin() + 0.5) * 255.0;
            let g = (0.5 * (2.0 * std::f64::consts::PI * (t + 1.0/3.0)).sin() + 0.5) * 255.0;
            let b = (0.5 * (2.0 * std::f64::consts::PI * (t + 2.0/3.0)).sin() + 0.5) * 255.0;
            
            palette.push([r as u8, g as u8, b as u8]);
        }
        
        palette
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to the Fractal Generator!", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn benchmark_fractal(width: u32, height: u32) -> f64 {
    let renderer = FractalRenderer::new(width, height);
    
    let start = Date::now();
    let _pixels = renderer.render_mandelbrot(-0.5, 0.0, 1.0);
    let end = Date::now();
    
    end - start
}