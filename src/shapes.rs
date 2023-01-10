use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Polytope {
    vertices: Vec<Point>
}