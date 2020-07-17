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

use crate::alphabet::Alphabet;
use crate::constant::{
    CHARS_TO_LANGUAGES_MAPPING, JAPANESE_CHARACTER_SET, LANGUAGE_MODELS_DIRECTORY,
    MULTIPLE_WHITESPACE, NO_LETTER, NUMBERS, PUNCTUATION,
};
use crate::language::Language;
use crate::language::Language::*;
use crate::model::{TestDataLanguageModel, TrainingDataLanguageModel};
use crate::ngram::Ngram;
use include_dir::Dir;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::io::{Cursor, Read};
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub struct LanguageDetector {
    languages: HashSet<Language>,
    minimum_relative_distance: f64,
    languages_with_unique_characters: HashSet<Language>,
    one_language_alphabets: HashMap<Alphabet, Language>,
}

impl LanguageDetector {
    pub(crate) fn from(languages: HashSet<Language>, minimum_relative_distance: f64) -> Self {
        let languages_with_unique_characters = languages
            .iter()
            .filter(|it| !it.unique_characters().is_empty())
            .cloned()
            .collect();
        let one_language_alphabets = Alphabet::all_supporting_single_language()
            .into_iter()
            .filter(|(_, language)| languages.contains(language))
            .collect();
        Self {
            languages,
            minimum_relative_distance,
            languages_with_unique_characters,
            one_language_alphabets,
        }
    }

    pub fn detect_language_of<T: Into<String>>(&self, text: T) -> Option<Language> {
        let confidence_values = self.compute_language_confidence_values(text);

        if confidence_values.is_empty() {
            return None;
        }

        let (most_likely_language, most_likely_language_probability) =
            &confidence_values.first().unwrap();

        if confidence_values.len() == 1 {
            return Some(most_likely_language.clone());
        }

        let (_, second_most_likely_language_probability) = &confidence_values.get(1).unwrap();

        if most_likely_language_probability == second_most_likely_language_probability {
            return None;
        }

        if (most_likely_language_probability - second_most_likely_language_probability)
            < self.minimum_relative_distance
        {
            return None;
        }

        Some(most_likely_language.clone())
    }

    pub fn compute_language_confidence_values<T: Into<String>>(
        &self,
        text: T,
    ) -> Vec<(Language, f64)> {
        let mut values = vec![];
        let cleaned_up_text = self.clean_up_input_text(text.into());

        if cleaned_up_text.is_empty() || NO_LETTER.is_match(&cleaned_up_text) {
            return values;
        }

        let words = self.split_text_into_words(&cleaned_up_text);
        let language_detected_by_rules = self.detect_language_with_rules(&words);

        if let Some(language) = language_detected_by_rules {
            values.push((language, 1.0));
            return values;
        }

        let mut all_probabilities = Vec::<HashMap<Language, f64>>::new();
        let mut unigram_counts = HashMap::<Language, u32>::new();
        let mut filtered_languages = self.filter_languages_by_rules(words);

        for i in 1..6 {
            if cleaned_up_text.len() < i {
                continue;
            }
            let test_data_model = TestDataLanguageModel::from(&cleaned_up_text, i);
            let probabilities =
                self.compute_language_probabilities(&test_data_model, &filtered_languages);
            let languages = probabilities.keys().collect_vec();

            if !languages.is_empty() {
                filtered_languages = filtered_languages
                    .into_iter()
                    .filter(|it| languages.contains(&it))
                    .collect();
            }

            if i == 1 {
                self.count_unigrams(&mut unigram_counts, &test_data_model, &filtered_languages);
            }

            all_probabilities.push(probabilities);
        }

        let summed_up_probabilities =
            self.sum_up_probabilities(all_probabilities, unigram_counts, filtered_languages);

        if summed_up_probabilities.is_empty() {
            return values;
        }

        let highest_probability = summed_up_probabilities
            .iter()
            .map(|(_, &probability)| probability)
            .sorted_by(|&first, &second| second.partial_cmp(&first).unwrap())
            .next()
            .unwrap();

        let confidence_values = summed_up_probabilities
            .iter()
            .map(|(language, &probability)| (language.clone(), highest_probability / probability))
            .sorted_by(|(_, first_probability), (_, second_probability)| {
                second_probability.partial_cmp(first_probability).unwrap()
            })
            .collect_vec();

        confidence_values
    }

    fn clean_up_input_text(&self, text: String) -> String {
        let trimmed = text.trim().to_lowercase();
        let without_punctuation = PUNCTUATION.replace_all(&trimmed, "");
        let without_numbers = NUMBERS.replace_all(&without_punctuation, "");
        let normalized_whitespace = MULTIPLE_WHITESPACE.replace_all(&without_numbers, " ");
        normalized_whitespace.to_string()
    }

    fn split_text_into_words<'a>(&self, text: &'a String) -> Vec<&'a str> {
        if text.contains(' ') {
            text.split(' ').collect_vec()
        } else {
            vec![text]
        }
    }

    fn detect_language_with_rules(&self, words: &Vec<&str>) -> Option<Language> {
        let mut total_language_counts = HashMap::<Option<&Language>, u32>::new();
        let half_word_count = (words.len() as f64) * 0.5;

        for word in words {
            let mut word_language_counts = HashMap::<&Language, u32>::new();

            for character in word.chars() {
                let mut is_match = false;
                let mut buffer = [0; 4];
                let char_str = character.encode_utf8(&mut buffer);

                for (alphabet, language) in self.one_language_alphabets.iter() {
                    if alphabet.matches(char_str) {
                        self.increment_counter(&mut word_language_counts, language);
                        is_match = true;
                    }
                }

                if !is_match {
                    if Alphabet::Han.matches(char_str) {
                        self.increment_counter(&mut word_language_counts, &Chinese);
                    } else if JAPANESE_CHARACTER_SET.is_match(char_str) {
                        self.increment_counter(&mut word_language_counts, &Japanese);
                    } else if Alphabet::Latin.matches(char_str)
                        || Alphabet::Cyrillic.matches(char_str)
                        || Alphabet::Devanagari.matches(char_str)
                    {
                        self.languages_with_unique_characters
                            .iter()
                            .filter(|it| it.unique_characters().contains(character))
                            .for_each(|it| self.increment_counter(&mut word_language_counts, it));
                    }
                }
            }

            if word_language_counts.is_empty() {
                self.increment_counter(&mut total_language_counts, None);
            } else if word_language_counts.len() == 1 {
                let counted_languages = word_language_counts.keys().collect_vec();
                let language = counted_languages.first().unwrap();
                if self.languages.contains(language) {
                    self.increment_counter(&mut total_language_counts, Some(language));
                } else {
                    self.increment_counter(&mut total_language_counts, None);
                }
            } else if word_language_counts.contains_key(&Chinese)
                && word_language_counts.contains_key(&Japanese)
            {
                self.increment_counter(&mut total_language_counts, Some(&Japanese));
            } else {
                let sorted_word_language_counts = word_language_counts
                    .into_iter()
                    .sorted_by(|(_, first_count), (_, second_count)| second_count.cmp(first_count))
                    .collect_vec();
                let (most_frequent_language, first_count) = sorted_word_language_counts[0];
                let (_, second_count) = sorted_word_language_counts[1];

                if first_count > second_count && self.languages.contains(most_frequent_language) {
                    self.increment_counter(
                        &mut total_language_counts,
                        Some(most_frequent_language),
                    );
                } else {
                    self.increment_counter(&mut total_language_counts, None);
                }
            }
        }

        let unknown_language_count = *total_language_counts.get(&None).or(Some(&0)).unwrap() as f64;

        if unknown_language_count < half_word_count {
            total_language_counts.remove(&None);
        }

        if total_language_counts.is_empty() {
            return None;
        }

        if total_language_counts.len() == 1 {
            return total_language_counts.iter().next().unwrap().0.cloned();
        }

        let sorted_total_language_counts = total_language_counts
            .into_iter()
            .sorted_by(|(_, first_count), (_, second_count)| second_count.cmp(first_count))
            .collect_vec();
        let (most_frequent_language, first_count) = sorted_total_language_counts[0];
        let (_, second_count) = sorted_total_language_counts[1];

        if first_count == second_count {
            return None;
        }

        most_frequent_language.cloned()
    }

    fn filter_languages_by_rules(&self, words: Vec<&str>) -> HashSet<Language> {
        let alphabets = vec![
            Alphabet::Arabic,
            Alphabet::Cyrillic,
            Alphabet::Devanagari,
            Alphabet::Han,
            Alphabet::Latin,
        ];
        let mut detected_alphabets = HashMap::<&Alphabet, u32>::new();
        let half_word_count = (words.len() as f64) * 0.5;

        for word in words.iter() {
            for alphabet in alphabets.iter() {
                if alphabet.matches(word) {
                    self.increment_counter(&mut detected_alphabets, alphabet);
                    break;
                }
            }
        }

        if detected_alphabets.is_empty() {
            return self.languages.clone();
        }

        let most_frequent_alphabet = detected_alphabets
            .into_iter()
            .sorted_by(|(_, first_count), (_, second_count)| second_count.cmp(first_count))
            .next()
            .unwrap()
            .0;

        let filtered_languages = self
            .languages
            .iter()
            .cloned()
            .filter(|it| it.alphabets().contains(most_frequent_alphabet))
            .collect::<HashSet<_>>();

        let mut language_counts = HashMap::<&Language, u32>::new();

        for word in words.iter() {
            for (characters, languages) in CHARS_TO_LANGUAGES_MAPPING.iter() {
                for character in characters.chars() {
                    if word.contains(character) {
                        for language in languages.iter() {
                            self.increment_counter(&mut language_counts, language);
                        }
                        break;
                    }
                }
            }
        }

        let languages_subset = language_counts
            .into_iter()
            .filter(|(_, count)| (*count as f64) >= half_word_count)
            .map(|(language, _)| language)
            .collect::<HashSet<_>>();

        if !languages_subset.is_empty() {
            filtered_languages
                .into_iter()
                .filter(|it| languages_subset.contains(&it))
                .collect::<HashSet<_>>()
        } else {
            filtered_languages
        }
    }

    fn compute_language_probabilities(
        &self,
        model: &TestDataLanguageModel,
        languages: &HashSet<Language>,
    ) -> HashMap<Language, f64> {
        let mut probabilities = hashmap!();
        for language in languages.iter() {
            let sum = self.compute_sum_of_ngram_probabilities(language, &model.ngrams);
            if sum < 0.0 {
                probabilities.insert(language.clone(), sum);
            }
        }
        probabilities
    }

    fn compute_sum_of_ngram_probabilities(
        &self,
        language: &Language,
        ngrams: &HashSet<Ngram>,
    ) -> f64 {
        let mut probabilities = vec![];
        for ngram in ngrams.iter() {
            for elem in ngram.range_of_lower_order_ngrams() {
                let probability = self.look_up_ngram_probability(language, &elem);
                if probability > 0.0 {
                    probabilities.push(probability);
                    break;
                }
            }
        }
        probabilities.into_iter().map(|it| it.ln()).sum()
    }

    fn look_up_ngram_probability(&self, language: &Language, ngram: &Ngram) -> f64 {
        let language_models = match ngram.value.len() {
            5 => fivegram_models(),
            4 => quadrigram_models(),
            3 => trigram_models(),
            2 => bigram_models(),
            1 => unigram_models(),
            0 => panic!("zerogram detected"),
            _ => panic!("unsupported ngram length detected: {}", ngram.value.len()),
        };

        language_models
            .get(language)
            .unwrap()
            .get()
            .unwrap()
            .get_relative_frequency(ngram)
    }

    fn count_unigrams(
        &self,
        unigram_counts: &mut HashMap<Language, u32>,
        unigram_model: &TestDataLanguageModel,
        filtered_languages: &HashSet<Language>,
    ) {
        for language in filtered_languages.iter() {
            for unigram in unigram_model.ngrams.iter() {
                if self.look_up_ngram_probability(language, unigram) > 0.0 {
                    self.increment_counter(unigram_counts, language.clone());
                }
            }
        }
    }

    fn sum_up_probabilities(
        &self,
        probabilities: Vec<HashMap<Language, f64>>,
        unigram_counts: HashMap<Language, u32>,
        filtered_languages: HashSet<Language>,
    ) -> HashMap<Language, f64> {
        let mut summed_up_probabilities = hashmap!();
        for language in filtered_languages.iter() {
            let mut sum = probabilities
                .iter()
                .map(|it| match it.get(language) {
                    Some(probability) => *probability,
                    None => 0.0,
                })
                .sum();

            if unigram_counts.contains_key(language) {
                sum /= *unigram_counts.get(language).unwrap() as f64;
            }

            if sum != 0.0 {
                summed_up_probabilities.insert(language.clone(), sum);
            }
        }

        summed_up_probabilities
    }

    fn increment_counter<T: Eq + Hash>(&self, counts: &mut HashMap<T, u32>, key: T) {
        let counter = counts.entry(key).or_insert(0);
        *counter += 1;
    }
}

fn unigram_models() -> &'static HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    static UNIGRAM_MODELS: OnceCell<HashMap<Language, OnceCell<TrainingDataLanguageModel>>> =
        OnceCell::new();
    UNIGRAM_MODELS.get_or_init(|| load_models(1))
}

fn bigram_models() -> &'static HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    static BIGRAM_MODELS: OnceCell<HashMap<Language, OnceCell<TrainingDataLanguageModel>>> =
        OnceCell::new();
    BIGRAM_MODELS.get_or_init(|| load_models(2))
}

fn trigram_models() -> &'static HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    static TRIGRAM_MODELS: OnceCell<HashMap<Language, OnceCell<TrainingDataLanguageModel>>> =
        OnceCell::new();
    TRIGRAM_MODELS.get_or_init(|| load_models(3))
}

fn quadrigram_models() -> &'static HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    static QUADRIGRAM_MODELS: OnceCell<HashMap<Language, OnceCell<TrainingDataLanguageModel>>> =
        OnceCell::new();
    QUADRIGRAM_MODELS.get_or_init(|| load_models(4))
}

fn fivegram_models() -> &'static HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    static FIVEGRAM_MODELS: OnceCell<HashMap<Language, OnceCell<TrainingDataLanguageModel>>> =
        OnceCell::new();
    FIVEGRAM_MODELS.get_or_init(|| load_models(5))
}

fn load_models(ngram_length: u32) -> HashMap<Language, OnceCell<TrainingDataLanguageModel>> {
    let mut map = hashmap!();
    for language in Language::iter() {
        let model = OnceCell::new();
        model.get_or_init(|| {
            let json = load_json(&LANGUAGE_MODELS_DIRECTORY, &language, ngram_length).unwrap();
            TrainingDataLanguageModel::from_json(&json)
        });
        map.insert(language, model);
    }
    map
}

fn load_json(directory: &Dir, language: &Language, ngram_length: u32) -> std::io::Result<String> {
    let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
    let file_path = format!("{}/{}s.json.zip", language.iso_code_639_1(), ngram_name);
    let zip_file = directory.get_file(file_path).unwrap();
    let zip_file_reader = Cursor::new(zip_file.contents());
    let mut archive = ZipArchive::new(zip_file_reader).unwrap();
    let mut json_file = archive.by_index(0).unwrap();
    let mut json = String::new();
    json_file.read_to_string(&mut json)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::include_dir;

    const LANGUAGE_MODELS_TEST_DIRECTORY: Dir = include_dir!("assets/test/language-models");

    #[test]
    fn test_load_json() {
        let result = load_json(&LANGUAGE_MODELS_TEST_DIRECTORY, &Language::English, 1);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"language":"ENGLISH","ngrams":{"2/93616591":"ﬀ ċ ė ĩ ȼ ɔ ţ ũ ʔ ơ ả ộ ù"}}"#
        );
    }
}
