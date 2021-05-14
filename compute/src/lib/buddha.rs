use num::complex::Complex;

// use fast;

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
            let r: Vec<Complex<f64>> = vec![];
            r
        }
    }
}

use num_traits::Zero;
// use std::ops::Rem;
// use crate::num_traits::MulAdd;

// This implementation corresponds to the Mandelbrot fractal.
impl DDS<Complex<f64>> for Buddhabrot {
    #[inline]
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= self.base.max_norm
    }

    #[inline]
    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        if z.is_zero() {
            return c
        }
        match (171.5 * z.norm_sqr()) as u64 % 4 {
            0 => z * z + c,
            1 => z * z * 0.6 + c,
            2 => z * z * 0.4 + c,
            3 => z * z * 0.2 + c,
            _ => Complex::zero()
        }
    }
 }

// static mut C: u64 = 0;
// static mut X: f64 = 0.1;
// static mut Y: f64 = 0.1;

// fn f_polar(a: f64, b: f64) -> Complex<f64>{
//     let d = Complex::new(a, b).to_polar();
//     Complex::new(d.0, d.1)
// }

// fn c_polar(a: Complex<f64>) -> Complex<f64>{
//     let d = a.to_polar();
//     Complex::new(d.0, d.1)
// }

// fn flip(a: Complex<f64>) -> Complex<f64> {
//     Complex::new(a.im, a.re)
// }
