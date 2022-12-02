use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Vector3(f32, f32, f32);
impl Vector3 {
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
        f32::sqrt(self.dot(self))
    }

    pub fn normalized(&self) -> Self {
        self / self.abs()
    }

    pub fn dot<T: AsRef<Self>>(&self, other: T) -> f32 {
        let other = other.as_ref();
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross<T: AsRef<Self>>(&self, other: T) -> Self {
        let other = other.as_ref();
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.0 * other.2 - self.2 * other.0,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl ops::Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// Reimplementing for non-borrowed types

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        &rhs * self
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

// Logistics
impl AsRef<Vector3> for Vector3 {
    fn as_ref(&self) -> &Vector3 {
        self
    }
}
