use serde::{Deserialize, Serialize};

use super::{camera::Camera, objects::object::Object};

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub objects: Vec<Object>,
}
