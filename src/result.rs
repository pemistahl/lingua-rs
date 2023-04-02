/*
 * Copyright © 2020-today Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::language::Language;

/// This struct describes a contiguous single-language
/// text section within a possibly mixed-language text.
pub struct DetectionResult {
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
    pub(crate) word_count: usize,
    pub(crate) language: Language,
}

impl DetectionResult {
    /// Returns the start index of the identified single-language substring.
    pub fn start_index(&self) -> usize {
        self.start_index
    }
    /// Returns the end index of the identified single-language substring.
    pub fn end_index(&self) -> usize {
        self.end_index
    }
    /// Returns the detected language of the identified single-language substring.
    pub fn language(&self) -> Language {
        self.language.clone()
    }
}
