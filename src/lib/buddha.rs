use num::complex::Complex;

use crate::lib::dyn_sys::DDS;
use crate::lib::dyn_sys::IFS;

pub struct Buddhabrot {
    max_iter: u64,
}

// Iterated Function System
impl Buddhabrot {
    pub fn new(max_iter: u64) -> Self {
        Buddhabrot { max_iter: max_iter }
    }
}

impl IFS<Complex<f64>, Vec<Complex<f64>>> for Buddhabrot {
    #[inline]
    fn iter(&self, start: Complex<f64>, c: Complex<f64>) -> Vec<Complex<f64>> {
        let mut res: Vec<Complex<f64>> = vec![];
        let mut i: u64 = 0;
        let mut z = start;
        res.push(z);
        while i < self.max_iter && self.cont(z) {
            z = self.next(z, c);
            res.push(z);
            i += 1;
        }
        if i < self.max_iter {
            res
        } else {
            let r: Vec<Complex<f64>> = vec![];
            r
        }
    }
}

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for Buddhabrot {
    #[inline]
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= 4.0
    }

    #[inline]
    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}
