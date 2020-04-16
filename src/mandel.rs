extern crate num;

use num::complex::Complex;

pub struct IFS
{
    max_iter: u64
}

// Discrete Dynamical System
pub trait DDS<State> {
    fn cont(&self, z: State) -> bool;
    fn next(&self, z: State, c: State) -> State;
}

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for IFS {
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= 4.0
    }

    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}

// Iterated Function System
impl IFS
{
    pub fn new(max_iter: u64) -> Self {
        Self {
            max_iter: max_iter
        }
    }

    pub fn iter(&self, c: Complex<f64>) -> u64 {
        let mut i: u64 = 0;
        let mut z = c;
        while i < self.max_iter && self.cont(z) {
            z = self.next(z, c);
            i += 1;
        }
        if i < self.max_iter { return self.max_iter - i; }
        0
    }
}