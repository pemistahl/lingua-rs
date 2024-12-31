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

use std::collections::{BTreeMap, HashMap};

use ahash::AHashMap;
use compact_str::CompactString;
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::fraction::Fraction;
use crate::language::Language;
use crate::ngram::{Ngram, NgramRef};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct JsonLanguageModel {
    language: Language,
    ngrams: BTreeMap<Fraction, String>,
}

pub(crate) struct TrainingDataLanguageModel {
    language: Language,
    pub(crate) absolute_frequencies: Option<HashMap<Ngram, u32>>,
    relative_frequencies: Option<HashMap<Ngram, Fraction>>,
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
            ngram_length,
            &absolute_frequencies,
            lower_ngram_absolute_frequencies,
        );

        TrainingDataLanguageModel {
            language: *language,
            absolute_frequencies: Some(absolute_frequencies),
            relative_frequencies: Some(relative_frequencies),
        }
    }

    pub(crate) fn from_json(json: &str) -> AHashMap<CompactString, f64> {
        let json_language_model = serde_json::from_str::<JsonLanguageModel>(json).unwrap();
        let mut json_relative_frequencies = AHashMap::new();

        for (fraction, ngrams) in json_language_model.ngrams {
            let floating_point_value = fraction.to_f64();
            for ngram in ngrams.split(' ') {
                json_relative_frequencies.insert(CompactString::new(ngram), floating_point_value);
            }
        }

        json_relative_frequencies
    }

    pub(crate) fn to_json(&self) -> String {
        let mut fractions_to_ngrams = hashmap!();
        for (ngram, fraction) in self.relative_frequencies.as_ref().unwrap() {
            let ngrams = fractions_to_ngrams.entry(fraction).or_insert_with(Vec::new);
            ngrams.push(ngram);
        }

        let mut fractions_to_joined_ngrams = btreemap!();
        for (fraction, ngrams) in fractions_to_ngrams {
            fractions_to_joined_ngrams.insert(
                *fraction,
                ngrams.iter().map(|&it| &it.value).sorted().join(" "),
            );
        }

        let model = JsonLanguageModel {
            language: self.language,
            ngrams: fractions_to_joined_ngrams,
        };

        serde_json::to_string(&model).unwrap()
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
        ngram_length: usize,
        absolute_frequencies: &HashMap<Ngram, u32>,
        lower_ngram_absolute_frequencies: &HashMap<Ngram, u32>,
    ) -> HashMap<Ngram, Fraction> {
        let mut ngram_probabilities = hashmap!();
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
            ngram_probabilities.insert(ngram.clone(), Fraction::new(*frequency, denominator));
        }

        ngram_probabilities
    }
}

pub(crate) struct TestDataLanguageModel<'a> {
    pub(crate) ngrams: Vec<Vec<NgramRef<'a>>>,
}

impl<'a> TestDataLanguageModel<'a> {
    pub(crate) fn from(words: &'a [String], ngram_length: usize) -> Self {
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

        let mut lower_order_ngrams = Vec::with_capacity(ngrams.len());

        for ngram in ngrams {
            lower_order_ngrams.push(ngram.range_of_lower_order_ngrams().collect_vec());
        }

        Self {
            ngrams: lower_order_ngrams,
        }
    }
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

    mod json_data {
        use super::*;

        #[test]
        fn test_json_model_serializer_and_deserializer() {
            let model = JsonLanguageModel {
                language: Language::English,
                ngrams: btreemap!(Fraction::new(3, 5) => "a b c d e".to_string()),
            };

            let serialized = serde_json::to_string(&model).unwrap();
            assert_eq!(
                serialized,
                r#"{"language":"ENGLISH","ngrams":{"3/5":"a b c d e"}}"#
            );

            let deserialized = serde_json::from_str::<JsonLanguageModel>(&serialized).unwrap();
            assert_eq!(deserialized, model);
        }
    }

    mod training_data {
        use super::*;

        fn map_keys_to_ngrams(map: HashMap<&str, u32>) -> HashMap<Ngram, u32> {
            map.into_iter()
                .map(|(key, value)| (Ngram::new(key), value))
                .collect()
        }

        fn map_keys_to_ngrams_and_values_to_fractions(
            map: HashMap<&str, &str>,
        ) -> HashMap<Ngram, Fraction> {
            map.into_iter()
                .map(|(key, value)| {
                    let (numerator, denominator) = value
                        .split('/')
                        .map(|it| it.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (Ngram::new(key), Fraction::new(numerator, denominator))
                })
                .collect()
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
        fn expected_unigram_relative_frequencies() -> HashMap<Ngram, Fraction> {
            map_keys_to_ngrams_and_values_to_fractions(hashmap!(
                "a" => "3/100", "b" => "1/100", "c" => "3/100", "d" => "1/20",
                "e" => "7/50", "f" => "1/50", "g" => "1/100", "h" => "1/25",
                "i" => "3/50", "l" => "1/100", "m" => "1/100", "n" => "1/10",
                "o" => "1/10", "p" => "3/100", "r" => "1/20", "s" => "1/10",
                "t" => "13/100", "u" => "3/100", "w" => "1/50", "y" => "3/100"
            ))
        }

        fn expected_unigram_json_relative_frequencies() -> AHashMap<CompactString, f64> {
            expected_unigram_relative_frequencies()
                .iter()
                .map(|(ngram, fraction)| {
                    (CompactString::new(ngram.value.clone()), fraction.to_f64())
                })
                .collect()
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
        fn expected_bigram_relative_frequencies() -> HashMap<Ngram, Fraction> {
            map_keys_to_ngrams_and_values_to_fractions(hashmap!(
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
            ))
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
        fn expected_trigram_relative_frequencies() -> HashMap<Ngram, Fraction> {
            map_keys_to_ngrams_and_values_to_fractions(hashmap!(
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
            ))
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
        fn expected_quadrigram_relative_frequencies() -> HashMap<Ngram, Fraction> {
            map_keys_to_ngrams_and_values_to_fractions(hashmap!(
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
            ))
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
        fn expected_fivegram_relative_frequencies() -> HashMap<Ngram, Fraction> {
            map_keys_to_ngrams_and_values_to_fractions(hashmap!(
                "testi" => "1/1", "sente" => "1/1", "ences" => "1/1", "tende" => "1/1",
                "ducti" => "1/1", "ntenc" => "1/2", "these" => "1/1", "onsis" => "1/1",
                "ntend" => "1/2", "total" => "1/1", "uctio" => "1/1", "enten" => "1/1",
                "poses" => "1/1", "ction" => "1/1", "produ" => "1/1", "inten" => "1/1",
                "nsist" => "1/1", "words" => "1/1", "sting" => "1/1", "purpo" => "1/1",
                "tence" => "1/1", "estin" => "1/1", "roduc" => "1/1", "urpos" => "1/1",
                "rpose" => "1/1", "ended" => "1/1", "oduct" => "1/1", "consi" => "1/1"
            ))
        }

        #[rstest(
            ngram_length,
            expected_absolute_frequencies,
            expected_relative_frequencies,
            lower_ngram_absolute_frequencies,
            case::unigram_model(
                1,
                expected_unigram_absolute_frequencies(),
                expected_unigram_relative_frequencies(),
                hashmap!()
            ),
            case::bigram_model(
                2,
                expected_bigram_absolute_frequencies(),
                expected_bigram_relative_frequencies(),
                expected_unigram_absolute_frequencies()
            ),
            case::trigram_model(
                3,
                expected_trigram_absolute_frequencies(),
                expected_trigram_relative_frequencies(),
                expected_bigram_absolute_frequencies()
            ),
            case::quadrigram_model(
                4,
                expected_quadrigram_absolute_frequencies(),
                expected_quadrigram_relative_frequencies(),
                expected_trigram_absolute_frequencies()
            ),
            case::fivegram_model(
                5,
                expected_fivegram_absolute_frequencies(),
                expected_fivegram_relative_frequencies(),
                expected_quadrigram_absolute_frequencies()
            ),
        )]
        fn test_ngram_model_creation(
            ngram_length: usize,
            expected_absolute_frequencies: HashMap<Ngram, u32>,
            expected_relative_frequencies: HashMap<Ngram, Fraction>,
            lower_ngram_absolute_frequencies: HashMap<Ngram, u32>,
        ) {
            let model = TrainingDataLanguageModel::from_text(
                &TEXT.trim().to_lowercase().lines().collect::<Vec<_>>(),
                &Language::English,
                ngram_length,
                "\\p{L}&&\\p{Latin}",
                &lower_ngram_absolute_frequencies,
            );

            assert_eq!(model.language, Language::English);
            assert_eq!(
                model.absolute_frequencies,
                Some(expected_absolute_frequencies)
            );
            assert_eq!(
                model.relative_frequencies,
                Some(expected_relative_frequencies)
            );
        }

        #[test]
        fn test_model_serializer_and_deserializer() {
            let model = TrainingDataLanguageModel {
                language: Language::English,
                absolute_frequencies: None,
                relative_frequencies: Some(expected_unigram_relative_frequencies()),
            };
            let deserialized = TrainingDataLanguageModel::from_json(&model.to_json());
            assert_eq!(deserialized, expected_unigram_json_relative_frequencies());
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
            let mut model = TestDataLanguageModel::from(&words, ngram_length);
            model
                .ngrams
                .sort_by(|first, second| first[0].value.cmp(&second[0].value));
            assert_eq!(model.ngrams, expected_ngrams);
        }
    }
}
