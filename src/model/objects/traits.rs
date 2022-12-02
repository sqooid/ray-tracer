use crate::{model::materials::material::Surface, render::ray::Ray};

pub trait Collidable: Textured {
    /// Check if a ray collides with this object
    fn collision_distance(&self, ray: &Ray) -> Option<f32>;

    /// Find the resultant ray after it bounces off this object
    fn bounce(&self, ray: &Ray) -> Ray;
}

pub trait Textured {
    fn get_material(&self) -> &dyn Surface;
}
