use serde::{Deserialize, Serialize};

use super::{color::Rgb, objects::object::Object};

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub objects: Vec<Object>,
    pub sky_top_color: Rgb<u8>,
    pub sky_bottom_color: Rgb<u8>,
}
