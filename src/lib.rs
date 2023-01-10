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

use std::path::PathBuf;
use serde::Deserialize;
use serde_yaml::Value;
use crate::shapes::Polytope;

pub fn parse_string(yaml_string: &str) -> Result<(), serde_yaml::Error> {
    let polytopes: Vec<Polytope> = serde_yaml::from_str(yaml_string).expect("Could not read values");
    println!("{:?}", polytopes);
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_parse_string() {
        let mut resources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        resources_dir.push("resources");

        //println!("resources dir = {}", resources_dir.display());

        parse_string("\
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
    }
}
