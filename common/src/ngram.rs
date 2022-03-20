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

use itertools::Itertools;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Hash, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive_attr(derive(Eq, PartialEq, Hash))]
pub struct Ngram {
    pub value: String,
}

impl Ngram {
    pub fn new(value: &str) -> Self {
        let char_count = value.chars().count();
        if !(0..6).contains(&char_count) {
            panic!(
                "length {} of ngram '{}' is not in range 0..6",
                char_count, value
            );
        }
        Self {
            value: value.to_string(),
        }
    }

    pub fn find_ngram_name_by_length(ngram_length: usize) -> &'static str {
        match ngram_length {
            1 => "unigram",
            2 => "bigram",
            3 => "trigram",
            4 => "quadrigram",
            5 => "fivegram",
            _ => panic!("ngram length {} is not in range 1..6", ngram_length),
        }
    }

    pub fn range_of_lower_order_ngrams(&self) -> NgramRange {
        NgramRange {
            start: self.clone(),
        }
    }
}

impl Display for Ngram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for Ngram {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.value)
    }
}

struct NgramVisitor;

impl<'de> Visitor<'de> for NgramVisitor {
    type Value = Ngram;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with length between 1 and 5")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Ngram::new(v))
    }
}

impl<'de> Deserialize<'de> for Ngram {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(NgramVisitor)
    }
}

pub struct NgramRange {
    start: Ngram,
}

impl Iterator for NgramRange {
    type Item = Ngram;

    fn next(&mut self) -> Option<Self::Item> {
        let value = &self.start.value;
        let length = value.chars().count();
        if length == 0 {
            None
        } else {
            let result = Some(self.start.clone());
            let chars = value.chars().collect_vec();
            let new_value = &chars[0..length - 1].iter().collect::<String>();
            self.start = Ngram::new(new_value);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram_serializer() {
        let ngram = Ngram::new("äbcde");
        let serialized = serde_json::to_string(&ngram).unwrap();
        assert_eq!(serialized, "\"äbcde\"");
    }

    #[test]
    fn test_ngram_deserializer() {
        let ngram = serde_json::from_str::<Ngram>("\"äbcde\"").unwrap();
        assert_eq!(ngram, Ngram::new("äbcde"));
    }

    #[test]
    fn test_ngram_iterator() {
        let ngram = Ngram::new("äbcde");
        let mut range = ngram.range_of_lower_order_ngrams();
        assert_eq!(range.next(), Some(Ngram::new("äbcde")));
        assert_eq!(range.next(), Some(Ngram::new("äbcd")));
        assert_eq!(range.next(), Some(Ngram::new("äbc")));
        assert_eq!(range.next(), Some(Ngram::new("äb")));
        assert_eq!(range.next(), Some(Ngram::new("ä")));
        assert_eq!(range.next(), None);
    }
}
