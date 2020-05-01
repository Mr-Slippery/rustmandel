use std::error::Error;
use std::fs;
use num::complex::Complex;

mod lib;
use lib::dyn_sys::IFS;
use lib::mandel::Mandelbrot;
use lib::app_cfg::*;
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
    let mandel = Mandelbrot::new(c.f.max_it);
    let d_x = canvas.width();
    let d_y = canvas.height();
    for j in 0..d_y {
        for i in 0..d_x {
            let x = c.f.min.re + (c.f.max.re - c.f.min.re) * (i as f64) / (d_x as f64);
            let y = c.f.min.im + (c.f.max.im - c.f.min.im) * (j as f64) / (d_y as f64);
            let p = Complex::new(x, y);
            let m: u64;
            match c.f.fractal_type {
                FractalType::Mandelbrot => m = mandel.iter(Complex::zero(), p),
                FractalType::Julia => m = mandel.iter(p, Complex::new(-0.70, -0.33)),
                _ => unreachable!()
            }
            match c.f.color_scheme {
                ColorScheme::Silver => {
                    let col = ((m * 8) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col, 255]))
                }
                ColorScheme::Times2232 => {
                    let col = ((m * 2) % 256) as u8;
                    let col1 = ((m * 32) % 256) as u8;
                    canvas.put_pixel(i, j, im::Rgba([col, col, col1, 255]))
                }
                ColorScheme::Crazy => {
                    let col = ((m as f64 / c.f.max_it as f64).sin() * 256.0) as u8;
                    let col1 = ((m as f64 / c.f.max_it as f64).cos() * 256.0) as u8;
                    let col2 = 2 * col * col1;
                    canvas.put_pixel(i, j, im::Rgba([col, col1, col2, 255]))
                }

            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg_filename = "Settings.toml";
    let app_config_str = fs::read_to_string(cfg_filename)
        .expect(&format!("Something went wrong reading {}", cfg_filename));
    let mut cfg: AppConfig = toml::from_str(&app_config_str).unwrap();
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

    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

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
                    }
                }
                // Alter fractal type.
                Button::Keyboard(Key::M) => cfg.f.fractal_type = FractalType::Mandelbrot,
                Button::Keyboard(Key::J) => cfg.f.fractal_type = FractalType::Julia,
                // Save image to file.
                Button::Keyboard(Key::S) => {
                    let filename = format!(
                        "{}.png",
                        cfg.image_path()
                    );
                    let _ = canvas.save(&filename).expect(&format!("Failed to write {}.", filename));
                    println!("Saved image: {}.", filename);
                    draw = false
                }
                // Save place to file.
                Button::Keyboard(Key::B) => {
                    // Save the config.
                    let out_cfg_str = toml::to_string_pretty(&cfg).unwrap();
                    let name = cfg.cfg_path();
                    let name1 = name.clone();
                    fs::write(name, &out_cfg_str)
                        .expect("Failed to write config!");
                    println!("Wrote config: {}.", name1);
                    // Save a thumbnail.
                    let thumb_size = 100;
                    let mut thumb = im::RgbaImage::new(thumb_size, thumb_size);
                    let mut smallcfg = cfg.clone();
                    smallcfg.w.width = thumb_size;
                    smallcfg.w.height = thumb_size;
                    render_mandel(&smallcfg, & mut thumb);
                    let filename = cfg.thumb_path();
                    thumb.save(&filename).expect(&format!("Failed to write {}!", filename))
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

            match cfg.f.fractal_type {
                FractalType::Buddhabrot => {
                    // TBD
                }
                FractalType::Mandelbrot | FractalType::Julia => 
                    render_mandel(&cfg, & mut canvas)
            }
        }
    }
    Ok(())
}