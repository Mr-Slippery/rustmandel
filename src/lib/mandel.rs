use num::complex::Complex;

use crate::lib::dyn_sys::DDS;
use crate::lib::dyn_sys::IFS;

use super::app_cfg::{default_max_norm, default_power};

pub struct Mandelbrot {
    max_iter: u64,
    power: i32,
    max_norm: f64,
}

// Iterated Function System
impl Mandelbrot {
    pub fn new(max_iter: u64) -> Self {
        Mandelbrot { max_iter: max_iter, power: default_power(), max_norm: default_max_norm(), /* ThreadPool::new() */ }
    }

    pub fn new_power_norm(max_iter: u64, power: i32, max_norm: f64) -> Self {
        let mut m = Mandelbrot::new(max_iter);
        m.power = power;
        m.max_norm = max_norm;
        m
    }
}

impl IFS<Complex<f64>, u64> for Mandelbrot {
    #[inline]
    fn iter(&self, start: Complex<f64>, c: Complex<f64>) -> u64 {
        let mut i: u64 = 0;
        let mut z = start;
        while i < self.max_iter && self.cont(z) {
            z = self.next(z, c);
            i += 1;
        }
        if i < self.max_iter {
            return self.max_iter - i;
        }
        0
    }
}

// use crate::lib::num_traits::Zero;
// use std::ops::Rem;
// use crate::lib::num_traits::MulAdd;

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for Mandelbrot {
    #[inline]
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= self.max_norm
    }

    #[inline]
    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}
