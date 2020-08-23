use std::error::Error;
use std::fs;
use num::complex::Complex;
use std::env;
use rand::prelude::*;

mod lib;
use lib::dyn_sys::IFS;
use lib::mandel::Mandelbrot;
use lib::buddha::Buddhabrot;
use lib::app_cfg::{AppConfig, Fractal, FractalType, ColorScheme};
use lib::*;

enum Zoom {
    In,
    Out,
    None,
}

use num_traits::Zero;

#[inline]
fn render_mandel(c: &AppConfig,
                 canvas: & mut im::RgbaImage) {        
    let d_x = canvas.width();
    let d_y = canvas.height();
    if c.f.max_it == 0 {
        for j in 0..d_y {
            for i in 0..d_x {
                canvas.put_pixel(i, j, im::Rgba([0, 0, 0, 255]))
            }
        }
        return;
    }
    let mandel = Mandelbrot::new(c.f.max_it);
    for j in 0..d_y {
        for i in 0..d_x {
            let x = c.f.min.re + (c.f.max.re - c.f.min.re) * (i as f64) / (d_x as f64);
            let y = c.f.min.im + (c.f.max.im - c.f.min.im) * (j as f64) / (d_y as f64);
            let p = Complex::new(x, y);
            let m1: u64;
            match c.f.fractal_type {
                FractalType::Mandelbrot => m1 = mandel.iter(Complex::zero(), p),
                FractalType::Julia => m1 = mandel.iter(p, Complex::new(-0.70, -0.33))
            }
            let m = m1 * 256 / c.f.max_it; 
            match c.f.color_scheme {
                ColorScheme::Silver => {
                    let col = ((m * 8) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col, 255]))
                }
                ColorScheme::Red => {
                    let col = ((m * 8) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, 0, 0, 255]))
                }
                ColorScheme::Times2232 => {
                    let col = ((m * 2) % 256) as u8;
                    let col1 = ((m * 32) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col1, 255]))
                }
                ColorScheme::Crazy => {
                    let col = ((m as f64 / c.f.max_it as f64).sin() * 256.0) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col / 2, col / 4, 255]))
                }
            }
        }
    }
}

#[inline]
fn render_buddha(c: &AppConfig,
                 canvas: & mut im::RgbaImage) {
    let bud = Buddhabrot::new(c.f.max_it);
    let d_x = canvas.width();
    let d_y = canvas.height();
    let mut rng = rand::thread_rng();
    for _ in 0..c.f.buddhabrot_points {
        let d_re = c.f.max.re - c.f.min.re;
        let d_im = c.f.max.im - c.f.min.im;
        let buddha_rel_size = c.f.buddhabrot_rel_size;
        let x: f64 = c.f.min.re - buddha_rel_size * d_re + (2.0 * buddha_rel_size + 1.0) * d_re * rng.gen::<f64>();
        let y: f64 = c.f.min.im - buddha_rel_size * d_im + (2.0 * buddha_rel_size + 1.0) * d_im * rng.gen::<f64>();
        let p = Complex::new(x, y);
        let m: Vec<Complex<f64>>;
        match c.f.fractal_type {
            FractalType::Mandelbrot => m = bud.iter(Complex::zero(), p),
            FractalType::Julia => m = bud.iter(p, Complex::new(-0.70, -0.33))
        }
        for z in m {
            let px = (d_x as f64 * (z.re - c.f.min.re) / d_re) as u32;
            if px >= d_x { continue }
            let py = (d_y as f64 * (z.im - c.f.min.im) / d_im) as u32;
            if py >= d_y { continue }
            let pixel = canvas.get_pixel_mut(px, py);
            let rgba = pixel.0;
            let r = if rgba[0] < 255 { rgba[0] + 1 } else { rgba[0] };
            let g = if rgba[1] < 254 { rgba[1] + 2 } else { rgba[1] };
            let b = if rgba[2] < 253 { rgba[2] + 3 } else { rgba[2] };
            *pixel = im::Rgba([r, g, b, 255]);
        }
    }
}

use std::path::PathBuf;

fn add_file(name: &str) -> String {
    let mut src = PathBuf::from("src");
    src.push("lib");
    src.push(name);
    let contents = fs::read_to_string(src.to_str().unwrap())
        .expect(&format!("Error reading {}.", name));
    let header = format!("\n\n#// {}: ", name);
    let comment: String = "\n#".to_string();
    let comment_copy = comment.clone();
    header + &comment + &contents.replace("\n", &comment_copy)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir()?;
    let args: Vec<String> = env::args().collect();
    let mut cfg: AppConfig;
    if !args.is_empty() {
        let arg: String = args[1].clone();
        cfg = AppConfig::from(&arg)
    } else {
        cfg = AppConfig::default()
    }
    AppConfig::make_saves_dir()?;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Mandelbrot", (cfg.w.width, cfg.w.height))
        .resizable(true)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    let mut canvas = im::ImageBuffer::new(cfg.w.width, cfg.w.height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    let mut o_size = window.size();

    let mut zoom = Zoom::None;
    let zoom_factor_inc = 0.01;

    let bp_inc = 100000;
    let brs_inc = 0.1;

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    let mut draw = true;
    let mut recording = false;
    let mut recording_index = 0;

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
                // Clear
                Button::Keyboard(Key::X) => {
                    let d_x = canvas.width();
                    let d_y = canvas.height();
                    let p = im::Rgba([0,0,0,255]);
                    for i in 0..d_x {
                        for j in 0..d_y {
                            canvas.put_pixel(i, j, p)
                        }
                    }
                }
                Button::Keyboard(Key::R) => {
                    recording = true;
                }
                Button::Keyboard(Key::T) => {
                    recording = false;
                    recording_index = 0;
                }
                // Zoom control
                Button::Keyboard(Key::Slash) => {
                    cfg.f.zoom_factor += zoom_factor_inc;
                    println!("Increased zoom factor to: {}", cfg.f.zoom_factor);
                    draw = false
                }
                Button::Keyboard(Key::Z) => {
                    if cfg.f.zoom_factor > zoom_factor_inc {
                        cfg.f.zoom_factor -= zoom_factor_inc;
                        println!("Decreased zoom factor to: {}", cfg.f.zoom_factor)
                    }
                    draw = false
                }
                // Movement
                Button::Keyboard(Key::Left) => {
                    cfg.f.min.re -= cfg.f.move_inc_rate * (cfg.f.max.re - cfg.f.min.re);
                    cfg.f.max.re -= cfg.f.move_inc_rate * (cfg.f.max.re - cfg.f.min.re)
                }
                Button::Keyboard(Key::Right) => {
                    cfg.f.min.re += cfg.f.move_inc_rate * (cfg.f.max.re - cfg.f.min.re);
                    cfg.f.max.re += cfg.f.move_inc_rate * (cfg.f.max.re - cfg.f.min.re)
                }
                Button::Keyboard(Key::Up) => {
                    cfg.f.min.im -= cfg.f.move_inc_rate * (cfg.f.max.im - cfg.f.min.im);
                    cfg.f.max.im -= cfg.f.move_inc_rate * (cfg.f.max.im - cfg.f.min.im)
                }
                Button::Keyboard(Key::Down) => {
                    cfg.f.min.im += cfg.f.move_inc_rate * (cfg.f.max.im - cfg.f.min.im);
                    cfg.f.max.im += cfg.f.move_inc_rate * (cfg.f.max.im - cfg.f.min.im)
                }
                Button::Keyboard(Key::LShift) => {
                    if cfg.f.move_inc_rate > 0.0 {
                        cfg.f.move_inc_rate -= 0.1 * cfg.f.move_inc_rate;
                        println!("Decreased move_inc_rate to: {}.", cfg.f.move_inc_rate)
                    }
                    draw = false
                }
                Button::Keyboard(Key::RShift) => {
                    cfg.f.move_inc_rate += 0.1 * cfg.f.move_inc_rate;                    
                    println!("Increased move_inc_rate to: {}.", cfg.f.move_inc_rate);
                    draw = false
                }
                Button::Keyboard(Key::Tab) => {
                    if cfg.f.it_inc > 0 {
                        cfg.f.it_inc -= 1;
                        println!("Decreased it_inc to: {}.", cfg.f.it_inc)
                    }
                    draw = false
                }
                Button::Keyboard(Key::Backslash) => {
                    cfg.f.it_inc += 1;                    
                    println!("Increased it_inc to: {}.", cfg.f.it_inc);
                    draw = false
                }
                // Center on mouse cursor.
                Button::Keyboard(Key::C) => {
                    let interval = cfg.f.max - cfg.f.min;
                    let cxy = cfg.f.min
                        + Complex::new(x * interval.re / size.width, y * interval.im / size.height);
                    cfg.f.min = cxy - interval / 2.0;
                    cfg.f.max = cxy + interval / 2.0
                }
                // Increase/decrease maximum iterations.
                Button::Keyboard(Key::RightBracket) => {
                    cfg.f.max_it += cfg.f.it_inc;
                    println!("Increased max_it to: {}.", cfg.f.max_it)
                }
                Button::Keyboard(Key::LeftBracket) => {
                    if cfg.f.max_it >= cfg.f.it_inc {
                        cfg.f.max_it -= cfg.f.it_inc;
                        println!("Decreased max_it to: {}.", cfg.f.max_it)
                    } else {
                        cfg.f.max_it = 0
                    }
                }
                // Increase/decrease Buddhabrot complex interval scale size.
                Button::Keyboard(Key::P) => {
                    cfg.f.buddhabrot_rel_size += brs_inc;
                    println!("Increased buddhabrot rel size to: {}.", cfg.f.buddhabrot_rel_size);
                    draw = false
                }
                Button::Keyboard(Key::O) => {
                    if cfg.f.buddhabrot_rel_size >= brs_inc {
                        cfg.f.buddhabrot_rel_size -= brs_inc;
                        println!("Decreased buddhabrot rel size to: {}.", cfg.f.buddhabrot_rel_size)
                    } else {
                        cfg.f.buddhabrot_rel_size = 0.0
                    }
                    draw = false
                }
                // Increase/decrease Buddhabrot points.
                Button::Keyboard(Key::Period) => {
                    cfg.f.buddhabrot_points += bp_inc;
                    println!("Increased buddhabrot points to: {}.", cfg.f.buddhabrot_points);
                    draw = false
                }
                Button::Keyboard(Key::Comma) => {
                    if cfg.f.buddhabrot_points >= bp_inc {
                        cfg.f.buddhabrot_points -= bp_inc;
                        println!("Decreased buddhabrot points to: {}.", cfg.f.buddhabrot_points)
                    } else {
                        cfg.f.buddhabrot_points = 0
                    }
                    draw = false
                }
                // Alter fractal type.
                Button::Keyboard(Key::M) => cfg.f.fractal = Fractal::Mandelbrot,
                Button::Keyboard(Key::B) => cfg.f.fractal = Fractal::Buddhabrot,
                // Flip Mandelbrot/Julia fractal type.
                Button::Keyboard(Key::J) => 
                    cfg.f.fractal_type = match cfg.f.fractal_type {
                            FractalType::Julia => FractalType::Mandelbrot,
                            FractalType::Mandelbrot => FractalType::Julia
                    },
                // Save image to file.
                Button::Keyboard(Key::S) => {
                    let filename = format!(
                        "{}",
                        cfg.image_path()
                    );
                    let _ = canvas.save(&filename).expect(&format!("Failed to write: `{}/{}`.", path.display(), filename));
                    println!("Saved image: `{}/{}`.", path.display(), filename);
                    draw = false
                }
                // Save place to file.
                Button::Keyboard(Key::F) => {
                    // Save the config.

                    let out_cfg_str = format!("{}{}", toml::to_string_pretty(&cfg).unwrap(),
                    match cfg.f.fractal {
                        Fractal::Mandelbrot => add_file("mandel.rs"),
                        Fractal::Buddhabrot => add_file("buddha.rs")
                    });
                    let name = cfg.cfg_path();
                    let name1 = name.clone();
                    fs::write(name, &out_cfg_str)
                        .expect("Failed to write config!");
                    println!("Wrote config: `{}/{}`.", path.display(), name1);
                    // Save a thumbnail.
                    let thumb_size = 100;
                    let mut thumb = im::RgbaImage::new(thumb_size, thumb_size);
                    let mut smallcfg = cfg.clone();
                    smallcfg.w.width = thumb_size;
                    smallcfg.w.height = thumb_size;
                    match smallcfg.f.fractal {
                        Fractal::Mandelbrot => render_mandel(&smallcfg, & mut thumb),
                        Fractal::Buddhabrot => render_buddha(&smallcfg, & mut thumb)
                    }
                    let filename = smallcfg.thumb_path();
                    thumb.save(&filename).expect(&format!("Failed to write: `{}/{}`!", path.display(), filename));
                    draw = false
                }
                // Alter the color scheme.
                Button::Keyboard(Key::D0) => cfg.f.color_scheme = lib::app_cfg::next(cfg.f.color_scheme),
                // Resize window
                Button::Keyboard(Key::D1) => {
                    if size.height >= cfg.w.size_inc && size.width >= cfg.w.size_inc {
                        window.set_size(Size {
                            height: size.height - cfg.w.size_inc,
                            width: size.width - cfg.w.size_inc,
                        })
                    }
                }
                Button::Keyboard(Key::D2) => window.set_size(Size {
                    height: size.height + cfg.w.size_inc,
                    width: size.width + cfg.w.size_inc,
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
                    mult = 1.0 / cfg.f.zoom_factor;
                    cfg.f.max_it += cfg.f.it_inc
                }
                Zoom::Out => {
                    mult = cfg.f.zoom_factor;
                    if cfg.f.max_it >= cfg.f.it_inc {
                        cfg.f.max_it -= cfg.f.it_inc
                    }
                }
                Zoom::None => mult = 1.0,
            }
            zoom = Zoom::None;

            if mult != 1.0 {
                let interval = cfg.f.max - cfg.f.min;
                let cxy =
                    cfg.f.min + Complex::new(x * interval.re / size.width, y * interval.im / size.height);

                let new_interval = interval * mult;

                cfg.f.min = cxy - new_interval / 2.0;
                cfg.f.max = cxy + new_interval / 2.0
            }

            match cfg.f.fractal {
                Fractal::Buddhabrot => render_buddha(&cfg, & mut canvas),
                Fractal::Mandelbrot => render_mandel(&cfg, & mut canvas)
            }

            if recording {
                recording_index += 1;
                let filename = format!(
                    "{}",
                    cfg.recorded_path(recording_index)
                );
                let _ = canvas.save(&filename).expect(&format!("Failed to write: `{}/{}`.", path.display(), filename));
                println!("Saved image: `{}/{}`.", path.display(), filename);
            }
        }
    }
    Ok(())
}