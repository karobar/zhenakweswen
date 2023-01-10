/*
 * Copyright (C) 2023, Travis Pressler, All rights reserved.
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
mod shapes;
mod rules;

use std::path::PathBuf;
use serde::Deserialize;
use serde_yaml::Value;

use crate::shapes::Polytope;
use crate::rules::Rule;

/// Parse the YAML into Shapes.
/// TODO: Wrap the serde_yaml::Error
pub fn parse_polytopes(yaml_string: &str) -> Result<Vec<Polytope>, serde_yaml::Error> {
    let polytopes: Vec<Polytope> = serde_yaml::from_str(yaml_string).expect("Could not read values");
    println!("{:?}", polytopes);
    Ok(polytopes)
}

pub fn parse_rules(yaml_string: &str) -> Result<Vec<Rule>, serde_yaml::Error> {
    let rules: Vec<Rule> = serde_yaml::from_str(yaml_string).expect("Could not read values");
    println!("{:?}", rules);
    Ok(rules)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_resources_dir() {
        let mut resources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        resources_dir.push("resources");
        println!("resources dir = {}", resources_dir.display());
    }

    #[test]
    pub fn test_parse_shapes() {
        let shapes = parse_polytopes("\
        ---
        - vertices:
            - x: 0
              y: 0
            - x: 4
              y: 0
            - x: 0
              y: 4
            - x: 4
              y: 4
        ").unwrap();
        assert_ne!(shapes.len(), 0);
    }

    #[test]
    pub fn test_parse_rules() {
        let rules = parse_rules("\
        ---
        - matched_vertices:
          - x: 0
            y: 0
          - x: 4
            y: 0
          - x: 0
            y: 4
          - x: 4
            y: 4
          replaced_vertices:
          - x: 0
            y: 0
          - x: 4
            y: 0
          - x: 0
            y: 4
          - x: 4
            y: 4
            ").unwrap();
        assert_ne!(rules.len(), 0);
    }
}

