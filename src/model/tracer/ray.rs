use serde::{Deserialize, Serialize};

use crate::math::vector::Coordinate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ray {
    pub origin: Coordinate,
    pub direction: Coordinate,
}
