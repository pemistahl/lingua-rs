/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
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

use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref JAPANESE_CHARACTER_SET: Regex =
        Regex::new("^[\\p{Hiragana}\\p{Katakana}\\p{Han}]+$").unwrap();
    pub static ref MULTIPLE_WHITESPACE: Regex = Regex::new("\\s+").unwrap();
    pub static ref NO_LETTER: Regex = Regex::new("^[^\\p{L}]+$").unwrap();
    pub static ref NUMBERS: Regex = Regex::new("\\p{N}").unwrap();
    pub static ref PUNCTUATION: Regex = Regex::new("\\p{P}").unwrap();
}
