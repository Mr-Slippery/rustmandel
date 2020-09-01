use std::fs;
use std::path::PathBuf;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use num::complex::Complex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum ColorScheme {
    Silver = 0,
    Times2232,
    Crazy,
    Red,
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
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Fractal {
    Mandelbrot,
    Buddhabrot,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub size_inc: f64,
}

fn default_buddhabrot_points() -> u64 {
    500000
}

fn default_buddhabrot_rel_size() -> f64 {
    2.0
}

pub fn default_power() -> i32 {
    1
}

pub fn default_max_norm() -> f64 {
    4.0
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct FractalConfig {
    pub min: Complex<f64>,
    pub max: Complex<f64>,
    pub max_it: u64,
    pub it_inc: u64,
    pub move_inc_rate: f64,
    pub zoom_factor: f64,
    pub fractal: Fractal,
    pub fractal_type: FractalType,
    pub color_scheme: ColorScheme,
    #[serde(default = "default_power")]
    pub power: i32,
    #[serde(default = "default_max_norm")]
    pub max_norm: f64,
    #[serde(default = "default_buddhabrot_rel_size")]
    pub buddhabrot_rel_size: f64,
    #[serde(default = "default_buddhabrot_points")]
    pub buddhabrot_points: u64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AppConfig {
    pub w: WindowConfig,
    pub f: FractalConfig,
}

pub const SAVE_DIR: &str = "save";
pub const CFG_DIR: &str = "cfg";
pub const IMAGE_DIR: &str = "image";
pub const RECORDING_DIR: &str = "rec";
pub const THUMB_DIR: &str = "thumb";
pub const DEFAULT_CFG: &str = "Settings.toml";
const THUMB_SUFFIX: &str = "_thumb.png";
const IMAGE_SUFFIX: &str = ".png";
const CFG_SUFFIX: &str = ".cfg";

impl AppConfig {
    fn name(self) -> String {
        format!("min_re_{}_min_im_{}_max_re_{}_max_im_{}_max_it_{}_fractal_{:?}_fractal_type_{:?}_color_scheme_{:?}",
            self.f.min.re, self.f.min.im, self.f.max.re, self.f.max.im,
            self.f.max_it,
            self.f.fractal, self.f.fractal_type,
            self.f.color_scheme
        )
    }

    pub fn make_saves_dir() -> std::io::Result<()> {
        let mut cfg_dir = PathBuf::from(SAVE_DIR);
        cfg_dir.push(CFG_DIR);
        let mut thumb_dir = PathBuf::from(SAVE_DIR);
        thumb_dir.push(THUMB_DIR);
        let mut image_dir = PathBuf::from(SAVE_DIR);
        image_dir.push(IMAGE_DIR);
        let mut rec_dir = PathBuf::from(SAVE_DIR);
        rec_dir.push(RECORDING_DIR);
        fs::create_dir_all(image_dir)?;
        fs::create_dir_all(thumb_dir)?;
        fs::create_dir_all(cfg_dir)?;
        fs::create_dir_all(rec_dir)?;
        Ok(())
    }

    fn path(self, root: String, sub: String, name: String) -> String {
        let mut p = PathBuf::new();
        p.push(root);
        p.push(sub);
        p.push(name);
        p.to_str().unwrap().to_string()
    }

    pub fn recorded_path(self, index: u64) -> String {
        let mut rec_dir = PathBuf::from(SAVE_DIR);
        rec_dir.push(RECORDING_DIR);
        let name = format!("{:?}", self.f.fractal);
        rec_dir.push(&name);
        fs::create_dir_all(&rec_dir).expect(&format!(
            "Failed to mkdir `{}`!",
            &rec_dir.as_path().display()
        ));
        self.path(
            format!("{}/{}", SAVE_DIR.to_string(), RECORDING_DIR.to_string()),
            name,
            format!("{}{}", index, IMAGE_SUFFIX),
        )
    }

    pub fn image_path(self) -> String {
        self.path(
            SAVE_DIR.to_string(),
            IMAGE_DIR.to_string(),
            format!("{}{}", self.name(), IMAGE_SUFFIX),
        )
    }

    pub fn cfg_path(self) -> String {
        self.path(
            SAVE_DIR.to_string(),
            CFG_DIR.to_string(),
            format!("{}{}", self.name(), CFG_SUFFIX),
        )
    }

    pub fn thumb_path(self) -> String {
        self.path(
            SAVE_DIR.to_string(),
            THUMB_DIR.to_string(),
            format!("{}{}", self.name(), THUMB_SUFFIX),
        )
    }

    fn cfg_name(arg: &String) -> String {
        let path = std::path::PathBuf::from(&arg);
        let basename = path.file_name().unwrap().to_str().unwrap();
        let mut root = path.parent().unwrap().parent().unwrap().to_path_buf();
        root.push(CFG_DIR);
        if arg.ends_with(CFG_SUFFIX) {
            arg.clone()
        } else if arg.ends_with(THUMB_SUFFIX) {
            let basenoext = (&basename[0..basename.len() - THUMB_SUFFIX.len()]).to_string();
            root.push(format!("{}{}", basenoext, CFG_SUFFIX));
            root.to_str().unwrap().to_string()
        } else if arg.ends_with(IMAGE_SUFFIX) {
            let basenoext = (&basename[0..basename.len() - IMAGE_SUFFIX.len()]).to_string();
            root.push(format!("{}{}", basenoext, CFG_SUFFIX));
            root.to_str().unwrap().to_string()
        } else {
            Self::default_cfg()
        }
    }

    fn default_cfg() -> String {
        format!("{}", DEFAULT_CFG)
    }

    pub fn from(arg: &String) -> AppConfig {
        let cfg_filename = Self::cfg_name(arg);
        let cfg_filename1 = cfg_filename.clone();
        let app_config_str = fs::read_to_string(cfg_filename)
            .expect(&format!("Something went wrong reading {}", cfg_filename1));
        let cfg: AppConfig = toml::from_str(&app_config_str).unwrap();
        return cfg;
    }

    pub fn default() -> AppConfig {
        Self::from(&Self::default_cfg())
    }
}
