// Mouse state and camera with pan/zoom inertia
// Ported from boomer's navigation.nim

use crate::config::Config;
use crate::math::Vec2;

const VELOCITY_THRESHOLD: f32 = 15.0;

// Mouse
#[derive(Debug, Clone, Default)]
pub struct Mouse {
    pub curr: Vec2,
    pub prev: Vec2,
    pub drag: bool,
}
