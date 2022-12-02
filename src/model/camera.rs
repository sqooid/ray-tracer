use crate::math::vector::Vector3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub position: Vector3,
    pub points_to: Vector3,
    pub horizontal_fov: f32,
    pub width: u32,
    pub height: u32,
    pub aperture: f32,
    pub focal_length: f32,
    #[serde(skip_deserializing)]
    canvas: Option<VectorCanvas>,
}

#[derive(Debug)]
struct VectorCanvas {
    pub corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    fn calculate_canvas(&mut self) {
        let direction = (&self.points_to - &self.position).normalized();
        let canvas_width = f32::atan(self.horizontal_fov) * 2.0;
        let canvas_height = self.height as f32 / self.width as f32 * canvas_width;
        let pixel_width = canvas_width / (self.width as f32 - 1.0);

        let width_vector_normalized = direction.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let height_vector_normalized = direction.cross(&width_vector_normalized).normalized();

        let canvas_corner = &self.position + &direction
            - (&width_vector_normalized / 2.0)
            - (&height_vector_normalized / 2.0);
    }
}
