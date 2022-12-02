use std::ops;

use image::ImageBuffer;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Rgb<T>(T, T, T);

impl<T> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self(r, g, b)
    }

    pub fn r(&self) -> &T {
        &self.0
    }
    pub fn g(&self) -> &T {
        &self.1
    }
    pub fn b(&self) -> &T {
        &self.2
    }
}

impl<T: std::ops::Mul<Output = T>> ops::Mul for Rgb<T> {
    type Output = Rgb<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

pub type RgbImage = ImageBuffer<Rgb<u8>, Vec<u8>>;

impl From<Rgb<f32>> for image::Rgb<u8> {
    fn from(old: Rgb<f32>) -> Self {
        Self([
            normalized_to_u8(old.0),
            normalized_to_u8(old.1),
            normalized_to_u8(old.2),
        ])
    }
}

impl From<image::Rgb<u8>> for Rgb<f32> {
    fn from(old: image::Rgb<u8>) -> Self {
        Self(
            old[0] as f32 / 255.0,
            old[1] as f32 / 255.0,
            old[2] as f32 / 255.0,
        )
    }
}

fn normalized_to_u8(num: f32) -> u8 {
    let clamped_num = if num > 1.0 { 1.0 } else { num };
    let scaled = (clamped_num * 255.0) as u8;
    scaled
}
