use serde::{Deserialize, Serialize};

use crate::math::vector::Vector3;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
    pub fn at_time(&self, t: f32) -> Vector3 {
        &self.origin + &(t * &self.direction)
    }
}
