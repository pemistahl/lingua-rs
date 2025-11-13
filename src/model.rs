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

use crate::json::load_json;
use crate::language::Language;
use crate::ngram::{Ngram, NgramRef};
use ahash::AHashMap;
use compact_str::CompactString;
use fraction::{Fraction, ToPrimitive};
use hashbrown::{DefaultHashBuilder, HashTable};
use itertools::Itertools;
use memmap2::Mmap;
use regex::Regex;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::{exists, File};
use std::hash::{BuildHasher, Hash, Hasher};
use std::io::{self, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NgramProbabilityModel {
    language: Language,
    #[serde(
        serialize_with = "serialize_ngram_probabilities",
        deserialize_with = "deserialize_ngram_probabilities"
    )]
    pub(crate) ngrams: AHashMap<CompactString, Fraction>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NgramCountModel {
    language: Language,
    pub(crate) ngrams: HashSet<String>,
}

#[derive(Debug)]
pub(crate) enum NgramModelType {
    Unique,
    MostCommon,
}

impl Display for NgramModelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

pub(crate) fn load_ngram_probability_model(
    language: Language,
    ngram_length: usize,
) -> Option<NgramProbabilityModel> {
    let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
    let file_name = format!("{ngram_name}s.json.br");
    match load_json(language, &file_name) {
        Ok(json) => Some(serde_json::from_str::<NgramProbabilityModel>(&json).unwrap()),
        Err(_) => None,
    }
}

pub(crate) fn load_ngram_count_model(
    language: Language,
    ngram_length: usize,
    model_type: NgramModelType,
) -> Option<NgramCountModel> {
    let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
    let file_name = format!("{model_type}_{ngram_name}s.json.br");
    match load_json(language, &file_name) {
        Ok(json) => Some(serde_json::from_str::<NgramCountModel>(&json).unwrap()),
        Err(_) => None,
    }
}

fn serialize_ngram_probabilities<S: Serializer>(
    source: &AHashMap<CompactString, Fraction>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut fractions_to_ngrams = btreemap!();
    for (ngram, fraction) in source {
        let serialized_fraction = format!(
            "{}/{}",
            fraction.numer().unwrap(),
            fraction.denom().unwrap()
        );
        let ngrams = fractions_to_ngrams
            .entry(serialized_fraction)
            .or_insert_with(Vec::new);
        ngrams.push(ngram);
    }
    let mut target = serializer.serialize_map(None)?;
    for (fraction, ngrams) in fractions_to_ngrams {
        let joined_ngrams = ngrams.iter().sorted().join(" ");
        target.serialize_entry(&fraction, &joined_ngrams)?;
    }
    target.end()
}

fn deserialize_ngram_probabilities<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<AHashMap<CompactString, Fraction>, D::Error> {
    let source = HashMap::<String, String>::deserialize(deserializer)?;
    let mut target = AHashMap::<CompactString, Fraction>::new();
    for (key, value) in source {
        let (numerator, denominator) = key.split('/').collect_tuple().unwrap();
        let parsed_numerator = numerator.parse::<u32>().unwrap();
        let parsed_denominator = denominator.parse::<u32>().unwrap();
        for ngram in value.split(' ') {
            target.insert(
                CompactString::new(ngram),
                Fraction::new(parsed_numerator, parsed_denominator),
            );
        }
    }
    Ok(target)
}

pub(crate) struct TrainingDataLanguageModel {
    pub(crate) absolute_frequencies: HashMap<Ngram, u32>,
    pub(crate) ngram_probability_model: NgramProbabilityModel,
}

impl TrainingDataLanguageModel {
    pub(crate) fn from_text(
        text: &[&str],
        language: &Language,
        ngram_length: usize,
        char_class: &str,
        lower_ngram_absolute_frequencies: &HashMap<Ngram, u32>,
    ) -> Self {
        let absolute_frequencies =
            Self::compute_absolute_frequencies(text, ngram_length, char_class);

        let relative_frequencies = Self::compute_relative_frequencies(
            *language,
            ngram_length,
            &absolute_frequencies,
            lower_ngram_absolute_frequencies,
        );

        TrainingDataLanguageModel {
            absolute_frequencies,
            ngram_probability_model: relative_frequencies,
        }
    }

    pub(crate) fn to_json(&self) -> String {
        serde_json::to_string(&self.ngram_probability_model).unwrap()
    }

    fn compute_absolute_frequencies(
        text: &[&str],
        ngram_length: usize,
        char_class: &str,
    ) -> HashMap<Ngram, u32> {
        let mut absolute_frequencies = hashmap!();
        let regex = Regex::new(&format!("^[{char_class}]+$")).unwrap_or_else(|_| {
            panic!(
                "The character class '{char_class}' cannot be compiled to a valid regular expression"
            )
        });

        for line in text.iter() {
            let chars = line.to_lowercase().chars().collect_vec();

            for i in 0..=chars.len() - ngram_length {
                let slice = &chars[i..i + ngram_length].iter().collect::<String>();

                if regex.is_match(slice) {
                    let counter = absolute_frequencies.entry(Ngram::new(slice)).or_insert(0);
                    *counter += 1;
                }
            }
        }

        absolute_frequencies
    }

    fn compute_relative_frequencies(
        language: Language,
        ngram_length: usize,
        absolute_frequencies: &HashMap<Ngram, u32>,
        lower_ngram_absolute_frequencies: &HashMap<Ngram, u32>,
    ) -> NgramProbabilityModel {
        let mut ngrams = AHashMap::<CompactString, Fraction>::new();
        let total_ngram_frequency = absolute_frequencies.values().sum::<u32>();

        for (ngram, frequency) in absolute_frequencies {
            let denominator = if ngram_length == 1 || lower_ngram_absolute_frequencies.is_empty() {
                total_ngram_frequency
            } else {
                let chars = ngram.value.chars().collect_vec();
                let slice = &chars[0..ngram_length - 1].iter().collect::<String>();
                *lower_ngram_absolute_frequencies
                    .get(&Ngram::new(slice))
                    .unwrap()
            };
            ngrams.insert(
                CompactString::new(&ngram.value),
                Fraction::new(*frequency, denominator),
            );
        }

        NgramProbabilityModel { language, ngrams }
    }
}

pub(crate) fn create_ngrams(words: &[String], ngram_length: usize) -> HashSet<NgramRef<'_>> {
    if !(1..6).contains(&ngram_length) {
        panic!("ngram length {ngram_length} is not in range 1..6");
    }
    let mut ngrams = hashset!();
    for word in words.iter() {
        let chars_count = word.chars().count();
        if chars_count >= ngram_length {
            for i in 0..=chars_count - ngram_length {
                let slice = get_utf8_slice(word, i, i + ngram_length);
                ngrams.insert(NgramRef::new(slice));
            }
        }
    }
    ngrams
}

pub(crate) fn create_lower_order_ngrams(
    words: &[String],
    ngram_length: usize,
) -> Vec<Vec<NgramRef<'_>>> {
    let ngrams = create_ngrams(words, ngram_length);
    let mut lower_order_ngrams = Vec::with_capacity(ngrams.len());
    for ngram in ngrams {
        lower_order_ngrams.push(ngram.range_of_lower_order_ngrams().collect_vec());
    }
    lower_order_ngrams
}

fn get_utf8_slice(string: &str, start: usize, end: usize) -> &str {
    string
        .char_indices()
        .nth(start)
        .map(|(start_pos, _)| {
            string[start_pos..]
                .char_indices()
                .nth(end - start)
                .map_or_else(
                    || &string[start_pos..],
                    |(end_pos, _)| &string[start_pos..start_pos + end_pos],
                )
        })
        .unwrap()
}

pub(crate) struct UnifiedNgramModel {
    #[allow(dead_code)]
    file: File,
    mmap: Mmap,
    hash_builder: DefaultHashBuilder,
    hash_table: HashTable<usize>,
}

impl<'a> UnifiedNgramModel {
    pub(crate) const PROBABILITY: u8 = 0;
    pub(crate) const UNIQUE: u8 = 1;
    pub(crate) const MOST_COMMON: u8 = 2;

    pub(crate) fn load(language: Language) -> io::Result<Self> {
        let file_name = format!("{language:?}.fst");

        if !exists(&file_name)? {
            eprintln!("Writing unified model {file_name}...");

            let mut buffer = Vec::new();

            for ngram_length in [1, 2, 3, 4, 5] {
                if let Some(ngram_probability_model) =
                    load_ngram_probability_model(language, ngram_length)
                {
                    for (ngram, probability) in &ngram_probability_model.ngrams {
                        let key = UnifiedNgramKey::new(ngram, Self::PROBABILITY);
                        buffer.extend(key.as_ref());

                        let probability = probability.to_f64().unwrap().ln().to_le_bytes();
                        buffer.extend(&probability);
                    }
                }

                if let Some(unique) =
                    load_ngram_count_model(language, ngram_length, NgramModelType::Unique)
                {
                    for ngram in &unique.ngrams {
                        let key = UnifiedNgramKey::new(ngram, Self::UNIQUE);
                        buffer.extend(key.as_ref());
                    }
                }

                if let Some(most_common) =
                    load_ngram_count_model(language, ngram_length, NgramModelType::MostCommon)
                {
                    for ngram in &most_common.ngrams {
                        let key = UnifiedNgramKey::new(ngram, Self::MOST_COMMON);
                        buffer.extend(key.as_ref());
                    }
                }
            }

            let mut file = File::create(&file_name)?;
            file.write_all(&buffer)?;
        }

        let file = File::open(file_name)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let hash_builder = DefaultHashBuilder::default();
        let mut hash_table = HashTable::new();

        let mut offset = 0;

        while offset < mmap.len() {
            let key = UnifiedNgramKey::read(&mmap[offset..]);

            let hash = hash_builder.hash_one(&key);

            let hasher = |&offset: &usize| {
                let key = UnifiedNgramKey::read(&mmap[offset..]);

                hash_builder.hash_one(&key)
            };

            hash_table.insert_unique(hash, offset, hasher);

            offset += key.as_ref().len();
            if key.kind() == Self::PROBABILITY {
                offset += 8;
            }
        }

        debug_assert_eq!(offset, mmap.len());

        Ok(Self {
            file,
            mmap,
            hash_builder,
            hash_table,
        })
    }

    pub(crate) fn get_probability(&self, ngram: &str) -> Option<f64> {
        self.get(ngram, Self::PROBABILITY)
            .map(|offset| f64::from_le_bytes(self.mmap[offset..offset + 8].try_into().unwrap()))
    }

    pub(crate) fn is_unique(&self, ngram: &str) -> bool {
        self.get(ngram, Self::UNIQUE).is_some()
    }

    pub(crate) fn is_most_common(&self, ngram: &str) -> bool {
        self.get(ngram, Self::MOST_COMMON).is_some()
    }

    fn get(&self, ngram: &str, kind: u8) -> Option<usize> {
        let key = UnifiedNgramKey::new(ngram, kind);

        let hash = self.hash_builder.hash_one(key);

        let eq = |&offset: &usize| UnifiedNgramKey::read(&self.mmap[offset..]) == key;

        self.hash_table
            .find(hash, eq)
            .map(|&offset| offset + key.as_ref().len())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct UnifiedNgramKey {
    key: [u8; Self::MAX_LEN],
}

impl UnifiedNgramKey {
    // Length byte, kind byte and maximum UTF-8-encoded length of fivegrams.
    const MAX_LEN: usize = 1 + 1 + 5 * 4;

    fn new(ngram: &str, kind: u8) -> Self {
        let len = ngram.len();

        let mut key = [0; Self::MAX_LEN];
        key[0] = len as u8;
        key[1] = kind;
        key[2..len + 2].copy_from_slice(ngram.as_bytes());

        Self { key }
    }

    fn read(bytes: &[u8]) -> Self {
        let len = bytes[0] as usize;

        let mut key = [0; Self::MAX_LEN];
        key[..len + 2].copy_from_slice(&bytes[..len + 2]);

        Self { key }
    }

    fn kind(&self) -> u8 {
        self.key[1]
    }
}

impl PartialEq for UnifiedNgramKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for UnifiedNgramKey {}

impl Hash for UnifiedNgramKey {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write(&self.key);
    }
}

impl AsRef<[u8]> for UnifiedNgramKey {
    fn as_ref(&self) -> &[u8] {
        &self.key[..self.key[0] as usize + 2]
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::*;

    use super::*;

    const TEXT: &str = "
        These sentences are intended for testing purposes.
        ⚠ Do not use them in production
        By the way, they consist of 23 words in total.
    ";

    fn map_strs_to_strings(strs: HashSet<&str>) -> HashSet<String> {
        strs.iter().map(|it| it.to_string()).collect()
    }

    #[test]
    fn test_ngram_probability_model_serializer_and_deserializer() {
        let mut ngrams = AHashMap::new();
        ngrams.insert(CompactString::new("a"), Fraction::new(1u32, 2u32));
        ngrams.insert(CompactString::new("b"), Fraction::new(1u32, 2u32));
        ngrams.insert(CompactString::new("c"), Fraction::new(7u32, 10u32));

        let model = NgramProbabilityModel {
            language: Language::English,
            ngrams,
        };

        let serialized_result = serde_json::to_string(&model);
        assert!(serialized_result.is_ok());

        let serialized = serialized_result.unwrap();
        assert_eq!(
            serialized,
            r#"{"language":"ENGLISH","ngrams":{"1/2":"a b","7/10":"c"}}"#
        );

        let deserialized_result = serde_json::from_str::<NgramProbabilityModel>(&serialized);
        assert!(deserialized_result.is_ok());

        let deserialized = deserialized_result.unwrap();
        assert_eq!(deserialized, model);
    }

    #[test]
    fn test_load_ngram_probability_model() {
        let optional_ngram_model = load_ngram_probability_model(Language::English, 1);
        assert!(optional_ngram_model.is_some());

        let ngram_model = optional_ngram_model.unwrap();
        assert_eq!(ngram_model.language, Language::English);
        assert!(ngram_model.ngrams.contains_key("a"));

        let expected_fraction = Fraction::new(7915445u32, 93616591u32);
        let actual_fraction = *ngram_model.ngrams.get("a").unwrap();
        assert_eq!(actual_fraction, expected_fraction);
    }

    #[test]
    fn test_load_unique_ngram_model() {
        let optional_unique_ngram_model =
            load_ngram_count_model(Language::English, 1, NgramModelType::Unique);
        assert!(optional_unique_ngram_model.is_some());

        let unique_ngram_model = optional_unique_ngram_model.unwrap();
        assert_eq!(unique_ngram_model.language, Language::English);
        assert_eq!(
            unique_ngram_model.ngrams,
            map_strs_to_strings(hashset!("ɦ", "ƅ", "ﬀ", "ƴ", "ｍ", "ȼ"))
        );
    }

    #[test]
    fn test_load_most_common_ngram_model() {
        let optional_most_common_ngram_model =
            load_ngram_count_model(Language::English, 1, NgramModelType::MostCommon);
        assert!(optional_most_common_ngram_model.is_some());

        let most_common_ngram_model = optional_most_common_ngram_model.unwrap();
        assert_eq!(most_common_ngram_model.language, Language::English);
        assert_eq!(
            most_common_ngram_model.ngrams,
            map_strs_to_strings(hashset!(
                "e", "t", "a", "o", "i", "n", "r", "s", "l", "h", "d", "c", "u", "m", "p", "f",
                "g", "y", "w", "b", "v", "k", "x", "j", "q"
            ))
        )
    }

    mod training_data {
        use super::*;

        fn map_keys_to_ngrams(map: HashMap<&str, u32>) -> HashMap<Ngram, u32> {
            map.into_iter()
                .map(|(key, value)| (Ngram::new(key), value))
                .collect()
        }

        fn map_relative_frequencies_to_ngram_probability_model(
            language: Language,
            map: HashMap<&str, &str>,
        ) -> NgramProbabilityModel {
            let mut ngrams = AHashMap::<CompactString, Fraction>::new();
            for (key, value) in map {
                let (numerator, denominator) = value
                    .split('/')
                    .map(|it| it.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ngrams.insert(
                    CompactString::new(key),
                    Fraction::new(numerator, denominator),
                );
            }
            NgramProbabilityModel { language, ngrams }
        }

        #[fixture]
        fn expected_unigram_absolute_frequencies() -> HashMap<Ngram, u32> {
            map_keys_to_ngrams(hashmap!(
                "a" => 3, "b" => 1, "c" => 3, "d" => 5, "e" => 14,
                "f" => 2, "g" => 1, "h" => 4, "i" => 6, "l" => 1,
                "m" => 1, "n" => 10, "o" => 10, "p" => 3, "r" => 5,
                "s" => 10, "t" => 13, "u" => 3, "w" => 2, "y" => 3
            ))
        }

        #[fixture]
        fn expected_unigram_probability_model() -> NgramProbabilityModel {
            map_relative_frequencies_to_ngram_probability_model(
                Language::English,
                hashmap!(
                    "a" => "3/100", "b" => "1/100", "c" => "3/100", "d" => "1/20",
                    "e" => "7/50", "f" => "1/50", "g" => "1/100", "h" => "1/25",
                    "i" => "3/50", "l" => "1/100", "m" => "1/100", "n" => "1/10",
                    "o" => "1/10", "p" => "3/100", "r" => "1/20", "s" => "1/10",
                    "t" => "13/100", "u" => "3/100", "w" => "1/50", "y" => "3/100"
                ),
            )
        }

        #[fixture]
        fn expected_bigram_absolute_frequencies() -> HashMap<Ngram, u32> {
            map_keys_to_ngrams(hashmap!(
                "de" => 1, "pr" => 1, "pu" => 1, "do" => 1, "uc" => 1, "ds" => 1,
                "du" => 1, "ur" => 1, "us" => 1, "ed" => 1, "in" => 4, "io" => 1,
                "em" => 1, "en" => 3, "is" => 1, "al" => 1, "es" => 4, "ar" => 1,
                "rd" => 1, "re" => 1, "ey" => 1, "nc" => 1, "nd" => 1, "ay" => 1,
                "ng" => 1, "ro" => 1, "rp" => 1, "no" => 1, "ns" => 1, "nt" => 2,
                "fo" => 1, "wa" => 1, "se" => 4, "od" => 1, "si" => 1, "of" => 1,
                "by" => 1, "wo" => 1, "on" => 2, "st" => 2, "ce" => 1, "or" => 2,
                "os" => 1, "ot" => 2, "co" => 1, "ta" => 1, "ct" => 1, "te" => 3,
                "th" => 4, "ti" => 2, "to" => 1, "he" => 4, "po" => 1
            ))
        }

        #[fixture]
        fn expected_bigram_probability_model() -> NgramProbabilityModel {
            map_relative_frequencies_to_ngram_probability_model(
                Language::English,
                hashmap!(
                    "de" => "1/5", "pr" => "1/3", "pu" => "1/3", "do" => "1/5",
                    "uc" => "1/3", "ds" => "1/5", "du" => "1/5", "ur" => "1/3",
                    "us" => "1/3", "ed" => "1/14", "in" => "2/3", "io" => "1/6",
                    "em" => "1/14", "en" => "3/14", "is" => "1/6", "al" => "1/3",
                    "es" => "2/7", "ar" => "1/3", "rd" => "1/5", "re" => "1/5",
                    "ey" => "1/14", "nc" => "1/10", "nd" => "1/10", "ay" => "1/3",
                    "ng" => "1/10", "ro" => "1/5", "rp" => "1/5", "no" => "1/10",
                    "ns" => "1/10", "nt" => "1/5", "fo" => "1/2", "wa" => "1/2",
                    "se" => "2/5", "od" => "1/10", "si" => "1/10", "of" => "1/10",
                    "by" => "1/1", "wo" => "1/2", "on" => "1/5", "st" => "1/5",
                    "ce" => "1/3", "or" => "1/5", "os" => "1/10", "ot" => "1/5",
                    "co" => "1/3", "ta" => "1/13", "ct" => "1/3", "te" => "3/13",
                    "th" => "4/13", "ti" => "2/13", "to" => "1/13", "he" => "1/1",
                    "po" => "1/3"
                ),
            )
        }

        #[fixture]
        fn expected_trigram_absolute_frequencies() -> HashMap<Ngram, u32> {
            map_keys_to_ngrams(hashmap!(
                "rds" => 1, "ose" => 1, "ded" => 1, "con" => 1, "use" => 1,
                "est" => 1, "ion" => 1, "ist" => 1, "pur" => 1, "hem" => 1,
                "hes" => 1, "tin" => 1, "cti" => 1, "wor" => 1, "tio" => 1,
                "ten" => 2, "ota" => 1, "hey" => 1, "tal" => 1, "tes" => 1,
                "uct" => 1, "sti" => 1, "pro" => 1, "odu" => 1, "nsi" => 1,
                "rod" => 1, "for" => 1, "ces" => 1, "nce" => 1, "not" => 1,
                "pos" => 1, "are" => 1, "tot" => 1, "end" => 1, "enc" => 1,
                "sis" => 1, "sen" => 1, "nte" => 2, "ord" => 1, "ses" => 1,
                "ing" => 1, "ent" => 1, "way" => 1, "nde" => 1, "int" => 1,
                "rpo" => 1, "the" => 4, "urp" => 1, "duc" => 1, "ons" => 1,
                "ese" => 1
            ))
        }

        #[fixture]
        fn expected_trigram_probability_model() -> NgramProbabilityModel {
            map_relative_frequencies_to_ngram_probability_model(
                Language::English,
                hashmap!(
                    "rds" => "1/1", "ose" => "1/1", "ded" => "1/1", "con" => "1/1",
                    "use" => "1/1", "est" => "1/4", "ion" => "1/1", "ist" => "1/1",
                    "pur" => "1/1", "hem" => "1/4", "hes" => "1/4", "tin" => "1/2",
                    "cti" => "1/1", "wor" => "1/1", "tio" => "1/2", "ten" => "2/3",
                    "ota" => "1/2", "hey" => "1/4", "tal" => "1/1", "tes" => "1/3",
                    "uct" => "1/1", "sti" => "1/2", "pro" => "1/1", "odu" => "1/1",
                    "nsi" => "1/1", "rod" => "1/1", "for" => "1/1", "ces" => "1/1",
                    "nce" => "1/1", "not" => "1/1", "pos" => "1/1", "are" => "1/1",
                    "tot" => "1/1", "end" => "1/3", "enc" => "1/3", "sis" => "1/1",
                    "sen" => "1/4", "nte" => "1/1", "ord" => "1/2", "ses" => "1/4",
                    "ing" => "1/4", "ent" => "1/3", "way" => "1/1", "nde" => "1/1",
                    "int" => "1/4", "rpo" => "1/1", "the" => "1/1", "urp" => "1/1",
                    "duc" => "1/1", "ons" => "1/2", "ese" => "1/4"
                ),
            )
        }

        #[fixture]
        fn expected_quadrigram_absolute_frequencies() -> HashMap<Ngram, u32> {
            map_keys_to_ngrams(hashmap!(
                "onsi" => 1, "sist" => 1, "ende" => 1, "ords" => 1, "esti" => 1,
                "oduc" => 1, "nces" => 1, "tenc" => 1, "tend" => 1, "thes" => 1,
                "rpos" => 1, "ting" => 1, "nsis" => 1, "nten" => 2, "tota" => 1,
                "they" => 1, "cons" => 1, "tion" => 1, "prod" => 1, "otal" => 1,
                "test" => 1, "ence" => 1, "pose" => 1, "oses" => 1, "nded" => 1,
                "inte" => 1, "them" => 1, "urpo" => 1, "duct" => 1, "sent" => 1,
                "stin" => 1, "ucti" => 1, "ente" => 1, "purp" => 1, "ctio" => 1,
                "rodu" => 1, "word" => 1, "hese" => 1
            ))
        }

        #[fixture]
        fn expected_quadrigram_probability_model() -> NgramProbabilityModel {
            map_relative_frequencies_to_ngram_probability_model(
                Language::English,
                hashmap!(
                    "onsi" => "1/1", "sist" => "1/1", "ende" => "1/1", "ords" => "1/1",
                    "esti" => "1/1", "oduc" => "1/1", "nces" => "1/1", "tenc" => "1/2",
                    "tend" => "1/2", "thes" => "1/4", "rpos" => "1/1", "ting" => "1/1",
                    "nsis" => "1/1", "nten" => "1/1", "tota" => "1/1", "they" => "1/4",
                    "cons" => "1/1", "tion" => "1/1", "prod" => "1/1", "otal" => "1/1",
                    "test" => "1/1", "ence" => "1/1", "pose" => "1/1", "oses" => "1/1",
                    "nded" => "1/1", "inte" => "1/1", "them" => "1/4", "urpo" => "1/1",
                    "duct" => "1/1", "sent" => "1/1", "stin" => "1/1", "ucti" => "1/1",
                    "ente" => "1/1", "purp" => "1/1", "ctio" => "1/1", "rodu" => "1/1",
                    "word" => "1/1", "hese" => "1/1"
                ),
            )
        }

        #[fixture]
        fn expected_fivegram_absolute_frequencies() -> HashMap<Ngram, u32> {
            map_keys_to_ngrams(hashmap!(
                "testi" => 1, "sente" => 1, "ences" => 1, "tende" => 1,
                "ducti" => 1, "ntenc" => 1, "these" => 1, "onsis" => 1,
                "ntend" => 1, "total" => 1, "uctio" => 1, "enten" => 1,
                "poses" => 1, "ction" => 1, "produ" => 1, "inten" => 1,
                "nsist" => 1, "words" => 1, "sting" => 1, "purpo" => 1,
                "tence" => 1, "estin" => 1, "roduc" => 1, "urpos" => 1,
                "rpose" => 1, "ended" => 1, "oduct" => 1, "consi" => 1
            ))
        }

        #[fixture]
        fn expected_fivegram_probability_model() -> NgramProbabilityModel {
            map_relative_frequencies_to_ngram_probability_model(
                Language::English,
                hashmap!(
                    "testi" => "1/1", "sente" => "1/1", "ences" => "1/1", "tende" => "1/1",
                    "ducti" => "1/1", "ntenc" => "1/2", "these" => "1/1", "onsis" => "1/1",
                    "ntend" => "1/2", "total" => "1/1", "uctio" => "1/1", "enten" => "1/1",
                    "poses" => "1/1", "ction" => "1/1", "produ" => "1/1", "inten" => "1/1",
                    "nsist" => "1/1", "words" => "1/1", "sting" => "1/1", "purpo" => "1/1",
                    "tence" => "1/1", "estin" => "1/1", "roduc" => "1/1", "urpos" => "1/1",
                    "rpose" => "1/1", "ended" => "1/1", "oduct" => "1/1", "consi" => "1/1"
                ),
            )
        }

        #[rstest(
            ngram_length,
            expected_absolute_frequencies,
            expected_probability_model,
            lower_ngram_absolute_frequencies,
            case::unigram_model(
                1,
                expected_unigram_absolute_frequencies(),
                expected_unigram_probability_model(),
                hashmap!()
            ),
            case::bigram_model(
                2,
                expected_bigram_absolute_frequencies(),
                expected_bigram_probability_model(),
                expected_unigram_absolute_frequencies()
            ),
            case::trigram_model(
                3,
                expected_trigram_absolute_frequencies(),
                expected_trigram_probability_model(),
                expected_bigram_absolute_frequencies()
            ),
            case::quadrigram_model(
                4,
                expected_quadrigram_absolute_frequencies(),
                expected_quadrigram_probability_model(),
                expected_trigram_absolute_frequencies()
            ),
            case::fivegram_model(
                5,
                expected_fivegram_absolute_frequencies(),
                expected_fivegram_probability_model(),
                expected_quadrigram_absolute_frequencies()
            ),
        )]
        fn test_ngram_model_creation(
            ngram_length: usize,
            expected_absolute_frequencies: HashMap<Ngram, u32>,
            expected_probability_model: NgramProbabilityModel,
            lower_ngram_absolute_frequencies: HashMap<Ngram, u32>,
        ) {
            let model = TrainingDataLanguageModel::from_text(
                &TEXT.trim().to_lowercase().lines().collect::<Vec<_>>(),
                &Language::English,
                ngram_length,
                "\\p{L}&&\\p{Latin}",
                &lower_ngram_absolute_frequencies,
            );

            assert_eq!(model.absolute_frequencies, expected_absolute_frequencies);
            assert_eq!(model.ngram_probability_model, expected_probability_model);
        }
    }

    mod test_data {
        use crate::detector::split_text_into_words;

        use super::*;

        fn map_strs_to_ngrams(strs: Vec<Vec<&'static str>>) -> Vec<Vec<NgramRef<'static>>> {
            strs.iter()
                .map(|ngram_strs| ngram_strs.iter().map(|&it| NgramRef::new(it)).collect())
                .collect()
        }

        #[fixture]
        fn expected_unigrams() -> Vec<Vec<NgramRef<'static>>> {
            map_strs_to_ngrams(vec![
                vec!["a"],
                vec!["b"],
                vec!["c"],
                vec!["d"],
                vec!["e"],
                vec!["f"],
                vec!["g"],
                vec!["h"],
                vec!["i"],
                vec!["l"],
                vec!["m"],
                vec!["n"],
                vec!["o"],
                vec!["p"],
                vec!["r"],
                vec!["s"],
                vec!["t"],
                vec!["u"],
                vec!["w"],
                vec!["y"],
            ])
        }

        #[fixture]
        fn expected_bigrams() -> Vec<Vec<NgramRef<'static>>> {
            map_strs_to_ngrams(vec![
                vec!["al", "a"],
                vec!["ar", "a"],
                vec!["ay", "a"],
                vec!["by", "b"],
                vec!["ce", "c"],
                vec!["co", "c"],
                vec!["ct", "c"],
                vec!["de", "d"],
                vec!["do", "d"],
                vec!["ds", "d"],
                vec!["du", "d"],
                vec!["ed", "e"],
                vec!["em", "e"],
                vec!["en", "e"],
                vec!["es", "e"],
                vec!["ey", "e"],
                vec!["fo", "f"],
                vec!["he", "h"],
                vec!["in", "i"],
                vec!["io", "i"],
                vec!["is", "i"],
                vec!["nc", "n"],
                vec!["nd", "n"],
                vec!["ng", "n"],
                vec!["no", "n"],
                vec!["ns", "n"],
                vec!["nt", "n"],
                vec!["od", "o"],
                vec!["of", "o"],
                vec!["on", "o"],
                vec!["or", "o"],
                vec!["os", "o"],
                vec!["ot", "o"],
                vec!["po", "p"],
                vec!["pr", "p"],
                vec!["pu", "p"],
                vec!["rd", "r"],
                vec!["re", "r"],
                vec!["ro", "r"],
                vec!["rp", "r"],
                vec!["se", "s"],
                vec!["si", "s"],
                vec!["st", "s"],
                vec!["ta", "t"],
                vec!["te", "t"],
                vec!["th", "t"],
                vec!["ti", "t"],
                vec!["to", "t"],
                vec!["uc", "u"],
                vec!["ur", "u"],
                vec!["us", "u"],
                vec!["wa", "w"],
                vec!["wo", "w"],
            ])
        }

        #[fixture]
        fn expected_trigrams() -> Vec<Vec<NgramRef<'static>>> {
            map_strs_to_ngrams(vec![
                vec!["are", "ar", "a"],
                vec!["ces", "ce", "c"],
                vec!["con", "co", "c"],
                vec!["cti", "ct", "c"],
                vec!["ded", "de", "d"],
                vec!["duc", "du", "d"],
                vec!["enc", "en", "e"],
                vec!["end", "en", "e"],
                vec!["ent", "en", "e"],
                vec!["ese", "es", "e"],
                vec!["est", "es", "e"],
                vec!["for", "fo", "f"],
                vec!["hem", "he", "h"],
                vec!["hes", "he", "h"],
                vec!["hey", "he", "h"],
                vec!["ing", "in", "i"],
                vec!["int", "in", "i"],
                vec!["ion", "io", "i"],
                vec!["ist", "is", "i"],
                vec!["nce", "nc", "n"],
                vec!["nde", "nd", "n"],
                vec!["not", "no", "n"],
                vec!["nsi", "ns", "n"],
                vec!["nte", "nt", "n"],
                vec!["odu", "od", "o"],
                vec!["ons", "on", "o"],
                vec!["ord", "or", "o"],
                vec!["ose", "os", "o"],
                vec!["ota", "ot", "o"],
                vec!["pos", "po", "p"],
                vec!["pro", "pr", "p"],
                vec!["pur", "pu", "p"],
                vec!["rds", "rd", "r"],
                vec!["rod", "ro", "r"],
                vec!["rpo", "rp", "r"],
                vec!["sen", "se", "s"],
                vec!["ses", "se", "s"],
                vec!["sis", "si", "s"],
                vec!["sti", "st", "s"],
                vec!["tal", "ta", "t"],
                vec!["ten", "te", "t"],
                vec!["tes", "te", "t"],
                vec!["the", "th", "t"],
                vec!["tin", "ti", "t"],
                vec!["tio", "ti", "t"],
                vec!["tot", "to", "t"],
                vec!["uct", "uc", "u"],
                vec!["urp", "ur", "u"],
                vec!["use", "us", "u"],
                vec!["way", "wa", "w"],
                vec!["wor", "wo", "w"],
            ])
        }

        #[fixture]
        fn expected_quadrigrams() -> Vec<Vec<NgramRef<'static>>> {
            map_strs_to_ngrams(vec![
                vec!["cons", "con", "co", "c"],
                vec!["ctio", "cti", "ct", "c"],
                vec!["duct", "duc", "du", "d"],
                vec!["ence", "enc", "en", "e"],
                vec!["ende", "end", "en", "e"],
                vec!["ente", "ent", "en", "e"],
                vec!["esti", "est", "es", "e"],
                vec!["hese", "hes", "he", "h"],
                vec!["inte", "int", "in", "i"],
                vec!["nces", "nce", "nc", "n"],
                vec!["nded", "nde", "nd", "n"],
                vec!["nsis", "nsi", "ns", "n"],
                vec!["nten", "nte", "nt", "n"],
                vec!["oduc", "odu", "od", "o"],
                vec!["onsi", "ons", "on", "o"],
                vec!["ords", "ord", "or", "o"],
                vec!["oses", "ose", "os", "o"],
                vec!["otal", "ota", "ot", "o"],
                vec!["pose", "pos", "po", "p"],
                vec!["prod", "pro", "pr", "p"],
                vec!["purp", "pur", "pu", "p"],
                vec!["rodu", "rod", "ro", "r"],
                vec!["rpos", "rpo", "rp", "r"],
                vec!["sent", "sen", "se", "s"],
                vec!["sist", "sis", "si", "s"],
                vec!["stin", "sti", "st", "s"],
                vec!["tenc", "ten", "te", "t"],
                vec!["tend", "ten", "te", "t"],
                vec!["test", "tes", "te", "t"],
                vec!["them", "the", "th", "t"],
                vec!["thes", "the", "th", "t"],
                vec!["they", "the", "th", "t"],
                vec!["ting", "tin", "ti", "t"],
                vec!["tion", "tio", "ti", "t"],
                vec!["tota", "tot", "to", "t"],
                vec!["ucti", "uct", "uc", "u"],
                vec!["urpo", "urp", "ur", "u"],
                vec!["word", "wor", "wo", "w"],
            ])
        }

        #[fixture]
        fn expected_fivegrams() -> Vec<Vec<NgramRef<'static>>> {
            map_strs_to_ngrams(vec![
                vec!["consi", "cons", "con", "co", "c"],
                vec!["ction", "ctio", "cti", "ct", "c"],
                vec!["ducti", "duct", "duc", "du", "d"],
                vec!["ences", "ence", "enc", "en", "e"],
                vec!["ended", "ende", "end", "en", "e"],
                vec!["enten", "ente", "ent", "en", "e"],
                vec!["estin", "esti", "est", "es", "e"],
                vec!["inten", "inte", "int", "in", "i"],
                vec!["nsist", "nsis", "nsi", "ns", "n"],
                vec!["ntenc", "nten", "nte", "nt", "n"],
                vec!["ntend", "nten", "nte", "nt", "n"],
                vec!["oduct", "oduc", "odu", "od", "o"],
                vec!["onsis", "onsi", "ons", "on", "o"],
                vec!["poses", "pose", "pos", "po", "p"],
                vec!["produ", "prod", "pro", "pr", "p"],
                vec!["purpo", "purp", "pur", "pu", "p"],
                vec!["roduc", "rodu", "rod", "ro", "r"],
                vec!["rpose", "rpos", "rpo", "rp", "r"],
                vec!["sente", "sent", "sen", "se", "s"],
                vec!["sting", "stin", "sti", "st", "s"],
                vec!["tence", "tenc", "ten", "te", "t"],
                vec!["tende", "tend", "ten", "te", "t"],
                vec!["testi", "test", "tes", "te", "t"],
                vec!["these", "thes", "the", "th", "t"],
                vec!["total", "tota", "tot", "to", "t"],
                vec!["uctio", "ucti", "uct", "uc", "u"],
                vec!["urpos", "urpo", "urp", "ur", "u"],
                vec!["words", "word", "wor", "wo", "w"],
            ])
        }

        #[rstest(
            ngram_length,
            expected_ngrams,
            case::unigram_model(1, expected_unigrams()),
            case::bigram_model(2, expected_bigrams()),
            case::trigram_model(3, expected_trigrams()),
            case::quadrigram_model(4, expected_quadrigrams()),
            case::fivegram_model(5, expected_fivegrams())
        )]
        fn test_ngram_model_creation(ngram_length: usize, expected_ngrams: Vec<Vec<NgramRef>>) {
            let words = split_text_into_words(TEXT);
            let mut model = create_lower_order_ngrams(&words, ngram_length);
            model.sort_by(|first, second| first[0].value.cmp(&second[0].value));
            assert_eq!(model, expected_ngrams);
        }
    }
}
