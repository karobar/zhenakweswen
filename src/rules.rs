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
