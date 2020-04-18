pub mod mandel;

use mandel::IFS;
use mandel::Mandelbrot;
use num::complex::Complex;

extern crate piston_window;
extern crate image as im;

use piston_window::*;

enum MandelType {
    Mandelbrot,
    Julia
}

fn main() {
    
    let opengl = OpenGL::V3_2;
    let (width, height) = (500, 500);
    let mut window: PistonWindow =
        WindowSettings::new("Mandelbrot", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut draw = true;

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();
    
    let mut o_size = window.size();
    
    let mut min = Complex::new(-4.0, -4.0);
    let mut max = Complex::new(4.0, 4.0);

    let mut max_it: u64 = 31;

    let mut zoom_in = false;
    let mut zoom_out = false;

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    let it_inc = 16;

    let mut mandel_type: MandelType = MandelType::Mandelbrot; 

    while let Some(e) = window.next() {
        let size = window.size();

        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }

        if let Some(_) = e.resize_args() {
            if o_size != size {
                canvas = im::ImageBuffer::new(size.width as u32, size.height as u32);
                texture_context = TextureContext {
                    factory: window.factory.clone(),
                    encoder: window.factory.create_command_buffer().into()
                };
                texture = Texture::from_image(
                        &mut texture_context,
                        &canvas,
                        &TextureSettings::new()
                    ).unwrap();
                draw = true;
                o_size = size;
            }
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                zoom_in = true;
                zoom_out = false;
            } else if button == Button::Mouse(MouseButton::Right) {
                zoom_out = true;
                zoom_in = false;
            } else {            
                if button == Button::Keyboard(Key::RightBracket) {
                    max_it += it_inc;
                    println!("Increased max_it to: {}.", max_it);
                } else if button == Button::Keyboard(Key::LeftBracket) {
                    if max_it >= it_inc {
                        max_it -= it_inc;
                        println!("Decreased max_it to: {}.", max_it);
                    }
                } else if button == Button::Keyboard(Key::M) {
                    mandel_type = MandelType::Mandelbrot;
                } else if button == Button::Keyboard(Key::J) {
                    mandel_type = MandelType::Julia;
                }
                draw = true;
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            x = pos[0] as f64;
            y = pos[1] as f64;
        }

        if zoom_in || zoom_out {
            let interval = max - min;
            let cxy = min + Complex::new(x * interval.re / size.width,
                                         y * interval.im / size.height);
            let zoom = 1.25;
            let mut mult = 1.0;

            if zoom_in {
                mult = 1.0 / zoom;
                max_it += it_inc
            } else if zoom_out {
                mult = zoom;
                if max_it >= it_inc {
                    max_it -= it_inc
                }
            }

            let new_interval = interval * mult;

            min = cxy - new_interval / 2.0;
            max = cxy + new_interval / 2.0;

            zoom_in = false;
            zoom_out = false;
            draw = true;
        }

        if draw {
            draw = false;
            let (d_x, d_y) = (canvas.width(), canvas.height());
            let mandel = Mandelbrot::new(max_it);
            for j in 0..d_y {
                for i in 0..d_x {
                    let x = min.re + (max.re - min.re) * (i as f64) / (d_x as f64);
                    let y = min.im + (max.im - min.im) * (j as f64) / (d_y as f64);
                    let c = Complex::new(x, y);
                    let m: u64;
                    match mandel_type {
                        MandelType::Mandelbrot => {
                            m = mandel.iter(Complex::new(0.0, 0.0), c);
                        }
                        MandelType::Julia => {
                            m = mandel.iter(c, Complex::new(-0.70, -0.33));
                        }
                    }
                    let col = ((m * 8) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col, 255]));
                }
            }
        }
    }
}