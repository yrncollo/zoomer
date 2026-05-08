// Configuration loading and defaults

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub min_scale: f32,
    pub scroll_speed: f32,
    pub drag_friction: f32,
    pub scale_friction: f32,
}
