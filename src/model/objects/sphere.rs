use serde::{Deserialize, Serialize};

use crate::{math::vector::Vector3, model::materials::material::Surface, render::ray::Ray};

use super::traits::{Collidable, Textured};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sphere {
    pub centre: Vector3,
    pub radius: f32,
}

impl Textured for Sphere {
    fn get_material(&self) -> &dyn Surface {
        todo!()
    }
}

impl Collidable for Sphere {
    fn collision_distance(&self, ray: &Ray) -> Option<f32> {
        // Find collision time
        let a = &self.centre - &ray.origin;
        let b = &ray.direction;
        let time = a.dot(&b.normalized());

        if time < 0.0 {
            None
        } else if (&self.centre - &ray.at_time(time)).abs() > self.radius {
            None
        } else {
            Some(time)
        }
    }

    fn collision_normal(&self, collision_point: &Vector3) -> Vector3 {
        (collision_point - &self.centre).normalized()
    }

    fn is_inward(&self, ray: &Ray) -> bool {
        (&ray.origin - &self.centre).dot(&ray.direction) > 0.0
    }
}
