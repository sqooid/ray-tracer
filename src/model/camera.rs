use crate::math::vector::Coordinate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub position: Coordinate,
    pub points_to: Coordinate,
    pub aperture: f32,
    pub focal_length: f32,
}
