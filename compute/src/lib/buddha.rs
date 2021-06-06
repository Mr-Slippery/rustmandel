use num::complex::Complex;

// use fast;

use crate::func;
use crate::dyn_sys::DDS;
use crate::dyn_sys::IFS;
use crate::defaults::{default_power, default_max_norm};

use super::mandel_base::MandelBase;

pub struct Buddhabrot {
    base: MandelBase,
}

// Iterated Function System
impl Buddhabrot {
    pub fn new(max_iter: u64) -> Self {
        Buddhabrot {
            base: MandelBase {
                max_iter: max_iter,
                power: default_power(),
                max_norm: default_max_norm(),
            },
        }
    }

    pub fn new_power_norm(max_iter: u64, power: i32, max_norm: f64) -> Self {
        let mut m = Buddhabrot::new(max_iter);
        m.base.power = power;
        m.base.max_norm = max_norm;
        m
    }
}

impl IFS<Complex<f64>, Vec<Complex<f64>>> for Buddhabrot {
    #[inline]
    fn iter(&self, start: Complex<f64>, c: Complex<f64>) -> Vec<Complex<f64>> {
        let mut res: Vec<Complex<f64>> = vec![];
        let mut i: u64 = 0;
        let mut z = start;
        res.push(z);
        while i < self.base.max_iter && self.cont(z) {
            z = self.next(z, c);
            res.push(z);
            i += 1;
        }
        if i < self.base.max_iter {
            res
        } else {
            vec![]
        }
    }
}

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for Buddhabrot {
    #[inline]
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= self.base.max_norm
    }

    #[inline]
    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        func::next(z, c)
    }
}

