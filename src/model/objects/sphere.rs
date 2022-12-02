use serde::{Deserialize, Serialize};

use crate::render::ray::Ray;

use super::traits::{Collidable, Textured};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sphere {}

impl Textured for Sphere {
    fn get_material(&self) -> &dyn crate::model::materials::material::Surface {
        todo!()
    }
}

impl Collidable for Sphere {
    fn collision_distance(&self, ray: &Ray) -> Option<f32> {
        todo!()
    }

    fn bounce(&self, ray: &Ray) -> Ray {
        todo!()
    }
}
