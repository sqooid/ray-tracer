use serde::{Deserialize, Serialize};

use crate::model::model::Rgb;

use super::diffuse::Diffuse;

#[derive(Debug, Serialize, Deserialize)]
pub enum Material {
    Diffuse(Diffuse),
}

pub trait Surface {
    fn get_radius(&self) -> f32;
    fn get_color(&self) -> &Rgb;
}
