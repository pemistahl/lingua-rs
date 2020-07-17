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

#![allow(dead_code)]

#[macro_use]
extern crate maplit;

mod alphabet;
mod builder;
mod constant;
mod detector;
mod fraction;
mod isocode;
mod language;
mod model;
mod ngram;

pub use builder::LanguageDetectorBuilder;
pub use detector::LanguageDetector;
pub use isocode::{IsoCode639_1, IsoCode639_3};
pub use language::Language;
