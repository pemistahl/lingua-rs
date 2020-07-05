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

use itertools::Itertools;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Eq, PartialEq, Hash)]
pub struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fraction({}, {})", self.numerator, self.denominator)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

struct FractionVisitor;

impl<'de> Visitor<'de> for FractionVisitor {
    type Value = Fraction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a rational number of the format 'numerator/denominator'")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let (numerator, denominator): (&str, &str) = v.split('/').collect_tuple().unwrap();
        let parsed_numerator = numerator.parse::<u32>().unwrap();
        let parsed_denominator = denominator.parse::<u32>().unwrap();
        Ok(Fraction::new(parsed_numerator, parsed_denominator))
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(FractionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_deserializer() {
        let fraction = serde_json::from_str::<Fraction>("\"3/5\"").unwrap();
        assert_eq!(fraction, Fraction::new(3, 5));
    }
}
