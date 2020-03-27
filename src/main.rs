extern crate num;

use num::complex::Complex;

fn mandel(c: Complex<f64>) -> i64 {
    let max = 256;
    let mut i: i64 = 0;
    let mut z = c;
    while i < max && z.norm_sqr() < 4.0 {
        z = z * z + c;
        i += 1;
    }
    if i < max { return max - i; }
    0
}

fn main() {
    let min = Complex::new(-2.0, -2.0);
    let max = Complex::new(2.0, 2.0);
    let d_x = 80;
    let d_y = 50;
    
    for j in 0..d_y-1 {
        for i in 0..d_x-1 {
            let x = min.re + (max.re - min.re) * (i as f64) / (d_x as f64);
            let y = min.im + (max.im - min.im) * (j as f64) / (d_y as f64);
            let c = Complex::new(x, y);
            let m = mandel(c);
            let mut ch = ' ';
            if m > 0 && m < 64 {
                ch = '.';
            }
            if m >= 64 && m < 128 {
                ch = 'o';
            }
            if m >= 128 && m < 192 {
                ch = '*';
            }
            if m >= 192 && m < 256 {
                ch = '0';
            }        
            print!("{}", ch);
        }
        println!("")
    }
}