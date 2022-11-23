use serde::{Deserialize, Serialize};

use crate::model::tracer::ray::Ray;

pub fn dot(v1: &Coordinate, v2: &Coordinate) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn distance_point_ray(point: &Coordinate, ray: &Ray) -> Option<f32> {
    todo!()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate(f32, f32, f32);
impl Coordinate {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
}
