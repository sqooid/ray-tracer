use serde::{Deserialize, Serialize};

use crate::model::model::Coordinate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ray {
    origin: Coordinate,
    direction: Coordinate,
}
