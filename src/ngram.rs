/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
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

use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Ngram {
    pub(crate) value: String,
}

impl Ngram {
    pub(crate) fn new(value: &str) -> Self {
        let char_count = value.chars().count();
        if !(0..6).contains(&char_count) {
            panic!("length {char_count} of ngram '{value}' is not in range 0..6");
        }
        Self {
            value: value.to_string(),
        }
    }
}

impl Display for Ngram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct NgramRef<'a> {
    pub(crate) value: &'a str,
    pub(crate) char_count: usize,
}

impl<'a> NgramRef<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        let char_count = value.chars().count();
        if !(0..6).contains(&char_count) {
            panic!("length {char_count} of ngram '{value}' is not in range 0..6");
        }
        Self { value, char_count }
    }

    pub(crate) fn range_of_lower_order_ngrams(&self) -> NgramRefRange<'a> {
        NgramRefRange { start: *self }
    }
}

pub(crate) struct NgramRefRange<'a> {
    start: NgramRef<'a>,
}

impl<'a> Iterator for NgramRefRange<'a> {
    type Item = NgramRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let last_ch = self.start.value.chars().next_back()?;
        let result = self.start;
        self.start.value = &self.start.value[..self.start.value.len() - last_ch.len_utf8()];
        self.start.char_count -= 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram_iterator() {
        let ngram = NgramRef::new("äbcde");
        let mut range = ngram.range_of_lower_order_ngrams();
        assert_eq!(range.next(), Some(NgramRef::new("äbcde")));
        assert_eq!(range.next(), Some(NgramRef::new("äbcd")));
        assert_eq!(range.next(), Some(NgramRef::new("äbc")));
        assert_eq!(range.next(), Some(NgramRef::new("äb")));
        assert_eq!(range.next(), Some(NgramRef::new("ä")));
        assert_eq!(range.next(), None);
    }
}
