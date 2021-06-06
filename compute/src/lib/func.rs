use num::complex::Complex;

// use fast;
use num_traits::Zero;
// use num_traits::One;
use std::ops::Rem;
// use num_traits::MulAdd;

pub fn next(z: Complex<f64>, c: Complex<f64>, power: i32) -> Complex<f64> {
    if z.is_zero() {
        return c       
    }
    z * c + z.rem(c + z).powi(power)
}

// static mut C: u64 = 0;
// static mut X: f64 = 0.01;
// static mut Y: f64 = 0.01;

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