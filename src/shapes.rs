use serde::{Deserialize, Serialize};
use crate::rules::Rule;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polytope {
    vertices: Vec<Point>
}

impl Polytope {
    pub fn apply_rule(&self, rule: Rule) -> Result<Polytope, Polytope> {
        for vertex in &(self.vertices) {
            if !rule.matched_vertices().contains(vertex) {
                return Err(self.clone());
            }
        }
        return Ok(Polytope { vertices: rule.replaced_vertices().to_vec() })
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::shapes::Polytope;
//
//     #[test]
//     pub fn test_parse_shapes() {
//         Polytope {}
//     }
// }