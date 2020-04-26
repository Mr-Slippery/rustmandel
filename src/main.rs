#[macro_use]
pub extern crate custom_derive;
#[macro_use]
pub extern crate enum_derive;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use chrono::{DateTime, Local};
use num::complex::Complex;
use std::error::Error;

mod lib;
use lib::config::Config;
use lib::dyn_sys::IFS;
use lib::mandel::Mandelbrot;
use lib::*;

#[derive(Debug, Copy, Clone, FromPrimitive)]
enum ColorScheme {
    Silver = 0,
    Times2232
}

fn next(d: ColorScheme) -> ColorScheme {
    match FromPrimitive::from_u8(d as u8 + 1) {
        Some(d2) => d2,
        None => FromPrimitive::from_u8(0).unwrap(),
    }
}

custom_derive! {
    #[derive(Debug, Copy, Clone, EnumFromStr)]
    enum FractalType {
        Mandelbrot,
        Julia,
        Buddhabrot,
    }
}

enum Zoom {
    In,
    Out,
    None,
}

#[inline]
fn render_mandel(d_x: u32, d_y: u32,
                 min: Complex<f64>, max: Complex<f64>,
                 max_it: u64,
                 fractal_type: FractalType,
                 color_scheme: ColorScheme,
                 canvas: & mut im::RgbaImage) {
    let mandel = Mandelbrot::new(max_it);
    for j in 0..d_y {
        for i in 0..d_x {
            let x = min.re + (max.re - min.re) * (i as f64) / (d_x as f64);
            let y = min.im + (max.im - min.im) * (j as f64) / (d_y as f64);
            let c = Complex::new(x, y);
            let m: u64;
            match fractal_type {
                FractalType::Mandelbrot => m = mandel.iter(Complex::new(0.0, 0.0), c),
                FractalType::Julia => m = mandel.iter(c, Complex::new(-0.70, -0.33)),
                _ => unreachable!()
            }
            match color_scheme {
                ColorScheme::Silver => {
                    let col = ((m * 8) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col, 255]))
                }
                ColorScheme::Times2232 => {
                    let col = ((m * 2) % 256) as u8;
                    let col1 = ((m * 32) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col1, 255]))
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut settings = Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings"))
        .unwrap();
    let opengl = OpenGL::V3_2;
    let width = settings.get::<u32>("window.width").unwrap();
    let height = settings.get::<u32>("window.height").unwrap();
    let mut window: PistonWindow = WindowSettings::new("Mandelbrot", (width, height))
        .resizable(true)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    let size_inc = settings.get::<f64>("window.size_inc").unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    let mut o_size = window.size();

    let mut min = Complex::new(
        settings.get::<f64>("fractal.min_re").unwrap(),
        settings.get::<f64>("fractal.min_im").unwrap(),
    );
    let mut max = Complex::new(
        settings.get::<f64>("fractal.max_re").unwrap(),
        settings.get::<f64>("fractal.max_im").unwrap(),
    );
    let mut max_it: u64 = settings.get::<u64>("fractal.max_it").unwrap();
    let it_inc = settings.get::<u64>("fractal.it_inc").unwrap();
    let move_inc_rate = settings.get::<f64>("fractal.move_inc_rate").unwrap();

    let mut zoom = Zoom::None;
    let zoom_factor = settings.get::<f64>("fractal.zoom_factor").unwrap();

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    let mut fractal_type: FractalType = settings.get::<String>("fractal.type")
                                            .unwrap().parse().unwrap();
    let mut color_scheme: ColorScheme = FromPrimitive::from_u8(settings.get::<u8>("fractal.color_scheme")
                                            .unwrap()).unwrap();
    let mut draw = true;

    while let Some(e) = window.next() {
        let size = window.size();

        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g)
            });
        }

        if let Some(_) = e.resize_args() {
            if o_size != size {
                canvas = im::ImageBuffer::new(size.width as u32, size.height as u32);
                texture_context = TextureContext {
                    factory: window.factory.clone(),
                    encoder: window.factory.create_command_buffer().into(),
                };
                texture =
                    Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new())
                        .unwrap();
                draw = true;
                o_size = size
            }
        }

        if let Some(button) = e.press_args() {
            draw = true;
            match button {
                // Zoom
                Button::Mouse(MouseButton::Left) => zoom = Zoom::In,
                Button::Mouse(MouseButton::Right) => zoom = Zoom::Out,
                // Movement
                Button::Keyboard(Key::Left) => {
                    min.re -= move_inc_rate * (max.re - min.re);
                    max.re -= move_inc_rate * (max.re - min.re)
                }
                Button::Keyboard(Key::Right) => {
                    min.re += move_inc_rate * (max.re - min.re);
                    max.re += move_inc_rate * (max.re - min.re)
                }
                Button::Keyboard(Key::Up) => {
                    min.im -= move_inc_rate * (max.im - min.im);
                    max.im -= move_inc_rate * (max.im - min.im)
                }
                Button::Keyboard(Key::Down) => {
                    min.im += move_inc_rate * (max.im - min.im);
                    max.im += move_inc_rate * (max.im - min.im)
                }
                // Center on mouse cursor.
                Button::Keyboard(Key::C) => {
                    let interval = max - min;
                    let cxy = min
                        + Complex::new(x * interval.re / size.width, y * interval.im / size.height);
                    min = cxy - interval / 2.0;
                    max = cxy + interval / 2.0
                }
                // Increase/decrease maximum iterations.
                Button::Keyboard(Key::RightBracket) => {
                    max_it += it_inc;
                    println!("Increased max_it to: {}.", max_it)
                }
                Button::Keyboard(Key::LeftBracket) => {
                    if max_it >= it_inc {
                        max_it -= it_inc;
                        println!("Decreased max_it to: {}.", max_it)
                    }
                }
                // Alter fractal type.
                Button::Keyboard(Key::M) => fractal_type = FractalType::Mandelbrot,
                Button::Keyboard(Key::J) => fractal_type = FractalType::Julia,
                // Save image to file.
                Button::Keyboard(Key::S) => {
                    let now: DateTime<Local> = Local::now();
                    let filename = format!(
                        "out_{}_min_{}_max_{}_type_{:?}.png",
                        now.to_rfc3339(),
                        min,
                        max,
                        fractal_type
                    );
                    let _ = canvas.save(&filename);
                    println!("Saved: {}.", filename);
                    draw = false
                }
                // Alter the color scheme.
                Button::Keyboard(Key::D0) => color_scheme = next(color_scheme),
                // Resize window
                Button::Keyboard(Key::D1) => {
                    if size.height >= size_inc && size.width >= size_inc {
                        window.set_size(Size {
                            height: size.height - size_inc,
                            width: size.width - size_inc,
                        })
                    }
                }
                Button::Keyboard(Key::D2) => window.set_size(Size {
                    height: size.height + size_inc,
                    width: size.width + size_inc,
                }),
                _ => {}
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            x = pos[0] as f64;
            y = pos[1] as f64;
        }

        if draw {
            draw = false;

            let mult: f64;
            match zoom {
                Zoom::In => {
                    mult = 1.0 / zoom_factor;
                    max_it += it_inc
                }
                Zoom::Out => {
                    mult = zoom_factor;
                    if max_it >= it_inc {
                        max_it -= it_inc
                    }
                }
                Zoom::None => mult = 1.0,
            }
            zoom = Zoom::None;

            if mult != 1.0 {
                let interval = max - min;
                let cxy =
                    min + Complex::new(x * interval.re / size.width, y * interval.im / size.height);

                let new_interval = interval * mult;

                min = cxy - new_interval / 2.0;
                max = cxy + new_interval / 2.0
            }

            let (d_x, d_y) = (canvas.width(), canvas.height());

            match fractal_type {
                FractalType::Buddhabrot => {
                    // TBD
                }
                FractalType::Mandelbrot | FractalType::Julia => 
                    render_mandel(d_x, d_y,
                                  min, max,
                                  max_it,
                                  fractal_type,
                                  color_scheme,
                                  & mut canvas)
            }
        }
    }
    Ok(())
}
