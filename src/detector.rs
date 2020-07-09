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
use crate::model::TrainingDataLanguageModel;
use crate::ngram::Ngram;
use include_dir::Dir;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::io::{Cursor, Read};
use strum::IntoEnumIterator;
use zip::ZipArchive;

pub struct LanguageDetector {
    languages: HashSet<Language>,
    languages_with_unique_characters: HashSet<Language>,
    one_language_alphabets: HashMap<Alphabet, Language>,
    minimum_relative_distance: f64,
}

impl LanguageDetector {
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

        // TODO: ngram lookup section

        values
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

        let unknown_language_count = total_language_counts
            .get(&None)
            .or(Some(&0))
            .unwrap()
            .clone() as f64;

        if unknown_language_count < half_word_count {
            total_language_counts.remove(&None);
        }

        if total_language_counts.is_empty() {
            return None;
        }

        if total_language_counts.len() == 1 {
            return total_language_counts
                .iter()
                .collect_vec()
                .first()
                .unwrap()
                .0
                .cloned();
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
            .collect_vec()
            .first()
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

    fn increment_counter<T: Eq + Hash>(&self, counts: &mut HashMap<T, u32>, key: T) {
        let counter = counts.entry(key).or_insert(0);
        *counter += 1;
    }
}

fn load_json(directory: &Dir, language: Language, ngram_length: u32) -> std::io::Result<String> {
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
        let result = load_json(&LANGUAGE_MODELS_TEST_DIRECTORY, Language::English, 1);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"language":"ENGLISH","ngrams":{"2/93616591":"ﬀ ċ ė ĩ ȼ ɔ ţ ũ ʔ ơ ả ộ ù"}}"#
        );
    }
}
