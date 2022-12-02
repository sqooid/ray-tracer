use serde::{Deserialize, Serialize};
use std::ops;

use crate::model::tracer::ray::Ray;

pub fn dot(v1: &Coordinate, v2: &Coordinate) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn point_ray_perp_t(point: &Coordinate, ray: &Ray) -> f32 {
    let a = point - &ray.origin;
    let b = &ray.direction;
    dot(&a, &b.norm())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate(f32, f32, f32);
impl Coordinate {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn random_gaussian() -> Self {
        todo!()
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

    pub fn abs(&self) -> f32 {
        f32::sqrt(dot(self, self))
    }

    pub fn norm(&self) -> Coordinate {
        self / self.abs()
    }
}

impl ops::Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Mul<f32> for &Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: f32) -> Self::Output {
        Coordinate::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<&Coordinate> for f32 {
    type Output = Coordinate;

    fn mul(self, rhs: &Coordinate) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f32> for &Coordinate {
    type Output = Coordinate;

    fn div(self, rhs: f32) -> Self::Output {
        Coordinate::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}
