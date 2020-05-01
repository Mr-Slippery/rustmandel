use std::fs;
use std::path::PathBuf;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::{Serialize, Deserialize};
use num::complex::Complex;

#[derive(Debug, Copy, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum ColorScheme {
    Silver = 0,
    Times2232,
    Crazy
}

pub fn next(d: ColorScheme) -> ColorScheme {
    match FromPrimitive::from_u8(d as u8 + 1) {
        Some(d2) => d2,
        None => FromPrimitive::from_u8(0).unwrap(),
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    Buddhabrot,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub size_inc: f64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct FractalConfig {
    pub min: Complex<f64>,
    pub max: Complex<f64>,
    pub max_it: u64,
    pub it_inc: u64,
    pub move_inc_rate: f64,
    pub zoom_factor: f64,
    pub fractal_type: FractalType,
    pub color_scheme: ColorScheme,    
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AppConfig {
    pub w: WindowConfig,
    pub f: FractalConfig, 
}

const SAVE_DIR: &str = "save";
const CFG_DIR: &str = "cfg";
const IMAGE_DIR: &str  = "image";
const THUMB_DIR: &str= "thumb";

impl AppConfig {
    fn name(self) -> String {
        format!("min_re_{}_min_im_{}_max_re_{}_max_im_{}_max_it_{}_fractal_type_{:?}_color_scheme_{:?}",
            self.f.min.re, self.f.min.im, self.f.max.re, self.f.max.im,
            self.f.max_it,
            self.f.fractal_type, self.f.color_scheme
        )
    }

    pub fn make_saves_dir() -> std::io::Result<()> {
        let mut cfg_dir = PathBuf::from(SAVE_DIR);
        cfg_dir.push(CFG_DIR);
        let mut thumb_dir = PathBuf::from(SAVE_DIR);
        thumb_dir.push(THUMB_DIR);
        let mut image_dir = PathBuf::from(SAVE_DIR);
        image_dir.push(IMAGE_DIR);
        fs::create_dir_all(image_dir)?;
        fs::create_dir_all(thumb_dir)?;
        fs::create_dir_all(cfg_dir)?;
        Ok(())
    }

    fn path(self, root: String, sub: String, name: String) -> String {
        let mut p = PathBuf::new();
        p.push(root);
        p.push(sub);
        p.push(name);
        p.to_str().unwrap().to_string()
    }
    
    pub fn image_path(self) -> String {
        self.path(SAVE_DIR.to_string(), IMAGE_DIR.to_string(), format!("{}.png", self.name()))
    }

    pub fn cfg_path(self) -> String {
        self.path(SAVE_DIR.to_string(), CFG_DIR.to_string(), format!("{}.cfg", self.name()))
    }

    pub fn thumb_path(self) -> String {
        self.path(SAVE_DIR.to_string(), THUMB_DIR.to_string(), format!("{}_thumb.png", self.name()))
    }
}