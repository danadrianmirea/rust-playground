// Additional fractal algorithms and utilities

pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
    
    pub fn magnitude_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    
    pub fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
    
    pub fn multiply(&self, other: &Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }
    
    pub fn square(&self) -> Complex {
        self.multiply(self)
    }
}

pub fn mandelbrot_iteration(c: &Complex, max_iterations: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    
    for i in 0..max_iterations {
        z = z.square().add(c);
        
        if z.magnitude_squared() > 4.0 {
            return i;
        }
    }
    
    max_iterations
}

pub fn julia_iteration(z: &Complex, c: &Complex, max_iterations: u32) -> u32 {
    let mut current = Complex::new(z.real, z.imag);
    
    for i in 0..max_iterations {
        current = current.square().add(c);
        
        if current.magnitude_squared() > 4.0 {
            return i;
        }
    }
    
    max_iterations
}

pub fn smooth_color(iteration: u32, z_real: f64, z_imag: f64, max_iterations: u32) -> f64 {
    if iteration == max_iterations {
        return max_iterations as f64;
    }
    
    let zn_real = z_real;
    let zn_imag = z_imag;
    let modulus = (zn_real * zn_real + zn_imag * zn_imag).sqrt();
    
    iteration as f64 + 1.0 - modulus.log10().log10() / 2.0_f64.log10()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complex_operations() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        
        let sum = c1.add(&c2);
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imag, 6.0);
        
        let product = c1.multiply(&c2);
        assert_eq!(product.real, -5.0); // 1*3 - 2*4
        assert_eq!(product.imag, 10.0); // 1*4 + 2*3
    }
    
    #[test]
    fn test_mandelbrot_inside() {
        let c = Complex::new(0.0, 0.0);
        let iterations = mandelbrot_iteration(&c, 100);
        assert_eq!(iterations, 100); // Should reach max iterations (inside set)
    }
    
    #[test]
    fn test_mandelbrot_outside() {
        let c = Complex::new(2.0, 2.0);
        let iterations = mandelbrot_iteration(&c, 100);
        assert!(iterations < 100); // Should escape quickly
    }
}