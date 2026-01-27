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

use indoc::formatdoc;
use itertools::Itertools;
use lingua::Language;
use polars::df;
use polars::frame::DataFrame;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter, Eq, PartialEq)]
pub(crate) enum Category {
    Average,
    SingleWords,
    WordPairs,
    Sentences,
}

impl Category {
    pub(crate) fn test_data_file_name(&self) -> Option<&'static str> {
        match self {
            Category::Average => None,
            Category::SingleWords => Some("single-words.txt"),
            Category::WordPairs => Some("word-pairs.txt"),
            Category::Sentences => Some("sentences.txt"),
        }
    }

    pub(crate) fn aggregated_report_file_name(&self) -> &'static str {
        match self {
            Category::Average => "average-accuracy-values.csv",
            Category::SingleWords => "single-words-accuracy-values.csv",
            Category::WordPairs => "word-pairs-accuracy-values.csv",
            Category::Sentences => "sentences-accuracy-values.csv",
        }
    }
}

pub(crate) struct DetectorStatistics {
    detector_name: String,
    is_single_language_detector: bool,
    pub(crate) language: Language,
    single_word_statistic: Statistic,
    word_pair_statistic: Statistic,
    sentence_statistic: Statistic,
    single_word_accuracy: f64,
    word_pair_accuracy: f64,
    sentence_accuracy: f64,
    average_accuracy: f64,
}

impl DetectorStatistics {
    pub(crate) fn new(
        detector_name: &str,
        is_single_language_detector: bool,
        language: Language,
    ) -> Self {
        Self {
            detector_name: detector_name.to_string(),
            is_single_language_detector,
            language,
            single_word_statistic: Statistic::new(),
            word_pair_statistic: Statistic::new(),
            sentence_statistic: Statistic::new(),
            single_word_accuracy: 0.0,
            word_pair_accuracy: 0.0,
            sentence_accuracy: 0.0,
            average_accuracy: 0.0,
        }
    }

    pub(crate) fn add_single_word_counts(&mut self, language: Option<Language>, single_word: &str) {
        self.single_word_statistic.add_language_count(language);
        self.single_word_statistic.add_entity_count();
        self.single_word_statistic
            .add_entity_length_count(single_word);
    }

    pub(crate) fn add_word_pair_counts(&mut self, language: Option<Language>, word_pair: &str) {
        self.word_pair_statistic.add_language_count(language);
        self.word_pair_statistic.add_entity_count();
        self.word_pair_statistic.add_entity_length_count(word_pair);
    }

    pub(crate) fn add_sentence_counts(&mut self, language: Option<Language>, sentence: &str) {
        self.sentence_statistic.add_language_count(language);
        self.sentence_statistic.add_entity_count();
        self.sentence_statistic.add_entity_length_count(sentence);
    }

    pub(crate) fn compute_accuracy_values(&mut self) {
        self.single_word_statistic.map_counts_to_accuracy_values();
        self.word_pair_statistic.map_counts_to_accuracy_values();
        self.sentence_statistic.map_counts_to_accuracy_values();
    }

    pub(crate) fn create_report_data(&mut self) -> Option<String> {
        let language_name = self.language.to_string().to_lowercase();
        let language =
            if self.is_single_language_detector && !self.detector_name.contains(&language_name) {
                None
            } else {
                Some(self.language)
            };

        let (single_word_accuracy, single_word_report) = self
            .single_word_statistic
            .create_report_data(&language, "single words");

        let (word_pair_accuracy, word_pair_report) = self
            .word_pair_statistic
            .create_report_data(&language, "word pairs");

        let (sentence_accuracy, sentence_report) = self
            .sentence_statistic
            .create_report_data(&language, "sentences");

        self.single_word_accuracy = single_word_accuracy;
        self.word_pair_accuracy = word_pair_accuracy;
        self.sentence_accuracy = sentence_accuracy;
        self.average_accuracy =
            (single_word_accuracy + word_pair_accuracy + sentence_accuracy) / 3.0;

        if self.average_accuracy == 0.0 {
            return None;
        }

        Some(formatdoc!(
            r#"
            ##### {:?} #####

            >>> Accuracy on average: {}

            {}
            {}
            {}
            "#,
            self.language,
            format_accuracy(self.average_accuracy),
            single_word_report,
            word_pair_report,
            sentence_report
        ))
    }

    pub(crate) fn to_dataframe(&self, category: Category) -> DataFrame {
        let accuracy = if category == Category::Average && self.average_accuracy > 0.0 {
            self.average_accuracy
        } else if category == Category::SingleWords && self.single_word_accuracy > 0.0 {
            self.single_word_accuracy
        } else if category == Category::WordPairs && self.word_pair_accuracy > 0.0 {
            self.word_pair_accuracy
        } else if category == Category::Sentences && self.sentence_accuracy > 0.0 {
            self.sentence_accuracy
        } else {
            f64::NAN
        };

        df!(
            "language" => [self.language.to_string()],
            self.detector_name.clone() => [accuracy * 100.0]
        )
        .unwrap()
    }
}

pub(crate) struct Statistic {
    language_counts: HashMap<Option<Language>, u32>,
    language_accuracies: HashMap<Option<Language>, f64>,
    entity_count: u32,
    entity_length_count: u32,
}

impl Statistic {
    pub(crate) fn new() -> Self {
        Self {
            language_counts: HashMap::new(),
            language_accuracies: HashMap::new(),
            entity_count: 0,
            entity_length_count: 0,
        }
    }

    pub(crate) fn add_language_count(&mut self, language: Option<Language>) {
        let count = self.language_counts.entry(language).or_insert(0);
        *count += 1;
    }

    pub(crate) fn add_entity_count(&mut self) {
        self.entity_count += 1;
    }

    pub(crate) fn add_entity_length_count(&mut self, entity: &str) {
        self.entity_length_count += entity.chars().count() as u32;
    }

    pub(crate) fn map_counts_to_accuracy_values(&mut self) {
        let sum_of_counts: u32 = self.language_counts.values().sum();
        self.language_accuracies = self
            .language_counts
            .iter()
            .map(|(language, count)| (*language, *count as f64 / sum_of_counts as f64))
            .collect();
    }

    pub(crate) fn create_report_data(
        &self,
        language: &Option<Language>,
        description: &str,
    ) -> (f64, String) {
        let accuracy = *self.language_accuracies.get(language).unwrap_or(&0.0);

        let average_length =
            ((self.entity_length_count as f64) / (self.entity_count as f64)).round();

        (
            accuracy,
            formatdoc!(
                r#"
                >> Detection of {} {} (average length: {} chars)
                Accuracy: {}
                Erroneously classified as {}
                "#,
                self.entity_count,
                description,
                average_length,
                format_accuracy(accuracy),
                self.format_language_accuracies(language)
            ),
        )
    }

    pub(crate) fn format_language_accuracies(&self, language: &Option<Language>) -> String {
        self.language_accuracies
            .iter()
            .filter(|(lang, _)| lang.as_ref() != language.as_ref())
            .sorted_by(
                |&(first_lang, &first_accuracy), &(second_lang, &second_accuracy)| {
                    let sorted_by_accuracy = second_accuracy.partial_cmp(&first_accuracy).unwrap();
                    let sorted_by_language = first_lang.partial_cmp(second_lang).unwrap();
                    sorted_by_accuracy.then(sorted_by_language)
                },
            )
            .map(|(lang, &accuracy)| {
                let formatted_lang = if lang.is_some() {
                    format!("{:?}", lang.as_ref().unwrap())
                } else {
                    "Unknown".to_string()
                };
                format!("{}: {}", formatted_lang, format_accuracy(accuracy))
            })
            .join(", ")
    }
}

fn format_accuracy(accuracy: f64) -> String {
    format!("{:.2}%", accuracy * 100_f64)
}
