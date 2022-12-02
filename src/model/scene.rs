use serde::{Deserialize, Serialize};

use super::camera::Camera;

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    pub camera: Camera,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub width: u32,
    pub height: u32,
}
