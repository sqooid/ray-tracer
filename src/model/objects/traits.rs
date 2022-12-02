use crate::{math::vector::Vector3, model::materials::material::Surface, render::ray::Ray};

pub trait Collidable {
    /// Check if a ray collides with this object.
    /// Returns collision time as multiple of ray direction if it collides, None otherwise
    fn collision_distance(&self, ray: &Ray) -> Option<f32>;

    /// Find the normal vector at the point of the collision.
    /// Returns normalized normal vector
    fn collision_normal(&self, collision_point: &Vector3) -> Vector3;

    /// True if ray travels 'inwards' into the object.
    /// Ray origin must be on the surface of the object
    fn is_inward(&self, ray: &Ray) -> bool;
}

pub trait Textured {
    fn get_material(&self) -> &dyn Surface;
}
