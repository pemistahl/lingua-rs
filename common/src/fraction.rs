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

use fraction::GenericFraction;
use itertools::Itertools;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        let fraction = GenericFraction::<u32>::new(numerator, denominator);
        Self {
            numerator: *fraction.numer().unwrap(),
            denominator: *fraction.denom().unwrap(),
        }
    }

    pub fn to_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
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

impl Serialize for Fraction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}/{}", self.numerator, self.denominator))
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
    fn test_fraction_reduction() {
        assert_eq!(Fraction::new(12, 144), Fraction::new(1, 12));
    }

    #[test]
    fn test_fraction_serializer() {
        let fraction = Fraction::new(3, 5);
        let serialized = serde_json::to_string(&fraction).unwrap();
        assert_eq!(serialized, "\"3/5\"");
    }

    #[test]
    fn test_fraction_deserializer() {
        let fraction = serde_json::from_str::<Fraction>("\"3/5\"").unwrap();
        assert_eq!(fraction, Fraction::new(3, 5));
    }
}
