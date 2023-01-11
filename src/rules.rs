use crate::shapes::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    matched_vertices: Vec<Point>,
    replaced_vertices: Vec<Point>,
}

impl Rule {
    pub fn matched_vertices(&self) -> &Vec<Point> {
        &self.matched_vertices
    }
    pub fn replaced_vertices(&self) -> &Vec<Point> {
        &self.replaced_vertices
    }
}
