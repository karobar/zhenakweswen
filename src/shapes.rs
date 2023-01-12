/*
 * Copyright (C) 2023, Travis Pressler
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::rules::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polytope {
    vertices: Vec<Point>,
}

impl Polytope {
    pub fn apply_rule(&self, rule: Rule) -> Result<Polytope, Polytope> {
        for vertex in &(self.vertices) {
            if !rule.matched_vertices().contains(vertex) {
                return Err(self.clone());
            }
        }
        return Ok(Polytope {
            vertices: rule.replaced_vertices().to_vec(),
        });
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
