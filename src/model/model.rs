use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Coordinate(i32, i32, i32);

#[derive(Debug, Deserialize, Serialize)]
struct Model {
    width: u32,
    height: u32,
    camera: Camera,
}

#[derive(Debug, Deserialize, Serialize)]
struct Camera {
    position: Coordinate,
    points_to: Coordinate,
}
