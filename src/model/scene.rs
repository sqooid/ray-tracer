use serde::Deserialize;

use super::{camera::Camera, model::Model};

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub output: OutputConfig,
    pub mode: Model,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub width: u32,
    pub height: u32,
}
