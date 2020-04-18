use num::complex::Complex;

use crate::lib::dyn_sys::IFS;
use crate::lib::dyn_sys::DDS;

pub struct Mandelbrot
{
    max_iter: u64
}

// Iterated Function System
impl Mandelbrot
{
    pub fn new(max_iter: u64) -> Self {
        Mandelbrot {
            max_iter: max_iter
        }
    }
}

impl IFS<Complex<f64>> for Mandelbrot {
    fn iter(&self, start: Complex<f64>, c: Complex<f64>) -> u64 {
        let mut i: u64 = 0;
        let mut z = start;
        while i < self.max_iter && self.cont(z) {
            z = self.next(z, c);
            i += 1;
        }
        if i < self.max_iter { return self.max_iter - i; }
        0
    }    
}

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for Mandelbrot {
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= 4.0
    }

    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}