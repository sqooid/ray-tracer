use serde::{Deserialize, Serialize};

use crate::model::model::Rgb;

use super::material::Surface;

#[derive(Debug, Deserialize, Serialize)]
pub struct Diffuse {
    color: Rgb,
    gloss: f32,
}

impl Surface for Diffuse {
    fn get_radius(&self) -> f32 {
        1.0 - self.gloss
    }

    fn get_color(&self) -> &Rgb {
        &self.color
    }
}
