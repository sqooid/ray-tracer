use serde::{Deserialize, Serialize};

use super::{plane::Plane, sphere::Sphere};

#[derive(Debug, Deserialize, Serialize)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}
