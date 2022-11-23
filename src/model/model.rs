use serde::{Deserialize, Serialize};

use super::objects::object::Object;

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate(f32, f32, f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub objects: Vec<Object>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub position: Coordinate,
    pub points_to: Coordinate,
}
