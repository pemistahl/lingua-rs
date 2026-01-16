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

use crate::detector::{CountModelFst, LanguageModelFst};
use crate::file::{read_count_model_data_file, read_probability_model_data_file};
use crate::language::Language;
use crate::ngram::{Ngram, NgramRef};
use itertools::Itertools;
use regex::Regex;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct NgramProbabilityModel {
    language: Language,
    pub(crate) ngrams: LanguageModelFst,
}

impl PartialEq for NgramProbabilityModel {
    fn eq(&self, other: &Self) -> bool {
        let ngrams = self.ngrams.as_fst().as_bytes();
        let other_ngrams = other.ngrams.as_fst().as_bytes();
        self.language == other.language && ngrams == other_ngrams
    }
}

#[derive(Debug)]
pub(crate) struct NgramCountModel {
    pub(crate) language: Language,
    pub(crate) ngrams: CountModelFst,
}

impl PartialEq for NgramCountModel {
    fn eq(&self, other: &Self) -> bool {
        let ngrams = self.ngrams.as_fst().as_bytes();
        let other_ngrams = other.ngrams.as_fst().as_bytes();
        self.language == other.language && ngrams == other_ngrams
    }
}

#[derive(Copy, Clone, Debug)]
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
    is_low_accuracy_mode_enabled: bool,
) -> Option<NgramProbabilityModel> {
    let file_name = if is_low_accuracy_mode_enabled {
        "low-accuracy-model.fst"
    } else {
        "high-accuracy-model.fst"
    };
    match read_probability_model_data_file(language, file_name) {
        Ok(ngrams) => Some(NgramProbabilityModel { language, ngrams }),
        Err(_) => None,
    }
}

pub(crate) fn load_ngram_count_model(
    language: Language,
    model_type: NgramModelType,
) -> Option<NgramCountModel> {
    let file_name = format!("{model_type}-ngrams.fst");
    match read_count_model_data_file(language, &file_name) {
        Ok(ngrams) => Some(NgramCountModel { language, ngrams }),
        Err(_) => None,
    }
}

pub(crate) fn create_fst_map(mut data: Vec<(Vec<u8>, u64)>) -> LanguageModelFst {
    data.sort_unstable_by(|(first, _), (second, _)| first.cmp(second));

    let mut fst_builder = fst::MapBuilder::memory();
    fst_builder.extend_iter(data).unwrap();

    let bytes = fst_builder.into_inner().unwrap();
    fst::Map::new(Cow::Owned(bytes)).unwrap()
}

pub(crate) fn create_fst_set(mut data: Vec<Vec<u8>>) -> CountModelFst {
    data.sort_unstable();

    let mut fst_builder = fst::SetBuilder::memory();
    fst_builder.extend_iter(data).unwrap();

    let bytes = fst_builder.into_inner().unwrap();
    fst::Set::new(Cow::Owned(bytes)).unwrap()
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
        let mut fst_data = vec![];
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
            let key = ngram.value.as_bytes().to_vec();
            let value = (*frequency as f64 / denominator as f64).ln().to_bits();
            fst_data.push((key, value));
        }

        NgramProbabilityModel {
            language,
            ngrams: create_fst_map(fst_data),
        }
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

pub(crate) fn get_utf8_slice(string: &str, start: usize, end: usize) -> &str {
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

    #[test]
    fn test_load_ngram_probability_model() {
        let optional_ngram_model = load_ngram_probability_model(Language::English, false);
        assert!(optional_ngram_model.is_some());

        let ngram_model = optional_ngram_model.unwrap();
        assert_eq!(ngram_model.language, Language::English);
        assert!(ngram_model.ngrams.contains_key(b"a".to_vec()));

        let expected_value = (7915445f64 / 93616591f64).ln().to_bits();
        let actual_value = ngram_model.ngrams.get(b"a".to_vec()).unwrap();
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_load_unique_ngram_model() {
        let optional_unique_ngram_model =
            load_ngram_count_model(Language::English, NgramModelType::Unique);
        assert!(optional_unique_ngram_model.is_some());

        let unique_ngram_model = optional_unique_ngram_model.unwrap();
        assert_eq!(unique_ngram_model.language, Language::English);
        assert!(unique_ngram_model.ngrams.contains("ɦ".as_bytes().to_vec()));
        assert!(unique_ngram_model.ngrams.contains("ƅ".as_bytes().to_vec()));
        assert!(unique_ngram_model.ngrams.contains("ﬀ".as_bytes().to_vec()));
        assert!(unique_ngram_model.ngrams.contains("ƴ".as_bytes().to_vec()));
        assert!(unique_ngram_model.ngrams.contains("ｍ".as_bytes().to_vec()));
        assert!(unique_ngram_model.ngrams.contains("ȼ".as_bytes().to_vec()));
    }

    #[test]
    fn test_load_most_common_ngram_model() {
        let optional_most_common_ngram_model =
            load_ngram_count_model(Language::English, NgramModelType::MostCommon);
        assert!(optional_most_common_ngram_model.is_some());

        let most_common_ngram_model = optional_most_common_ngram_model.unwrap();
        assert_eq!(most_common_ngram_model.language, Language::English);
        assert!(most_common_ngram_model.ngrams.contains(b"e".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"t".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"a".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"o".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"i".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"n".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"r".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"s".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"l".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"h".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"d".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"c".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"u".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"m".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"p".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"f".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"g".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"y".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"w".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"b".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"v".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"k".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"x".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"j".to_vec()));
        assert!(most_common_ngram_model.ngrams.contains(b"q".to_vec()));
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
            let mut fst_data = vec![];

            for (ngram, fraction) in map {
                let (numerator, denominator) = fraction
                    .split('/')
                    .map(|it| it.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let key = ngram.as_bytes().to_vec();
                let value = (numerator as f64 / denominator as f64).ln().to_bits();
                fst_data.push((key, value));
            }

            NgramProbabilityModel {
                language,
                ngrams: create_fst_map(fst_data),
            }
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
