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

use include_dir::{include_dir, Dir, File};
use indoc::formatdoc;
use itertools::Itertools;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use titlecase::titlecase;
use whatlang::{Detector, Lang};

const TEST_DATA_DIRECTORY: Dir = include_dir!("assets/test/language-testdata");

struct DetectorStatistics {
    single_word_statistic: Statistic,
    word_pair_statistic: Statistic,
    sentence_statistic: Statistic,
    average_accuracies: HashMap<Language, f64>,
}

impl DetectorStatistics {
    fn new() -> Self {
        Self {
            single_word_statistic: Statistic::new(),
            word_pair_statistic: Statistic::new(),
            sentence_statistic: Statistic::new(),
            average_accuracies: HashMap::new(),
        }
    }

    fn add_single_word_counts(&mut self, language: Option<Language>, single_word: &str) {
        self.single_word_statistic.add_language_count(language);
        self.single_word_statistic.add_entity_count();
        self.single_word_statistic
            .add_entity_length_count(single_word);
    }

    fn add_word_pair_counts(&mut self, language: Option<Language>, word_pair: &str) {
        self.word_pair_statistic.add_language_count(language);
        self.word_pair_statistic.add_entity_count();
        self.word_pair_statistic.add_entity_length_count(word_pair);
    }

    fn add_sentence_counts(&mut self, language: Option<Language>, sentence: &str) {
        self.sentence_statistic.add_language_count(language);
        self.sentence_statistic.add_entity_count();
        self.sentence_statistic.add_entity_length_count(sentence);
    }

    fn compute_accuracy_values(&mut self) {
        self.single_word_statistic.map_counts_to_accuracy_values();
        self.word_pair_statistic.map_counts_to_accuracy_values();
        self.sentence_statistic.map_counts_to_accuracy_values();
    }

    fn create_report_data(&mut self, language: &Language) -> Option<String> {
        let (single_word_accuracy, single_word_report) = self
            .single_word_statistic
            .create_report_data(language, "single words");

        let (word_pair_accuracy, word_pair_report) = self
            .word_pair_statistic
            .create_report_data(language, "word pairs");

        let (sentence_accuracy, sentence_report) = self
            .sentence_statistic
            .create_report_data(language, "sentences");

        let average_accuracy =
            (single_word_accuracy + word_pair_accuracy + sentence_accuracy) / 3.0;

        self.average_accuracies
            .insert(language.clone(), average_accuracy);

        if average_accuracy == 0.0 {
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
            language,
            format_accuracy(average_accuracy),
            single_word_report,
            word_pair_report,
            sentence_report
        ))
    }

    fn create_aggregated_report_row(&self, language: &Language) -> String {
        let average_accuracy_column = match self.average_accuracies.get(language) {
            Some(&accuracy) => {
                if accuracy > 0.0 {
                    accuracy.to_string()
                } else {
                    "NaN".to_string()
                }
            }
            None => "NaN".to_string(),
        };
        let single_words_accuracy_column = match self
            .single_word_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };
        let word_pairs_accuracy_column = match self
            .word_pair_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };
        let sentences_accuracy_column = match self
            .sentence_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };

        format!(
            "{},{},{},{}",
            average_accuracy_column,
            single_words_accuracy_column,
            word_pairs_accuracy_column,
            sentences_accuracy_column
        )
    }
}

struct Statistic {
    language_counts: HashMap<Option<Language>, u32>,
    language_accuracies: HashMap<Option<Language>, f64>,
    entity_count: u32,
    entity_length_count: u32,
}

impl Statistic {
    fn new() -> Self {
        Self {
            language_counts: HashMap::new(),
            language_accuracies: HashMap::new(),
            entity_count: 0,
            entity_length_count: 0,
        }
    }

    fn add_language_count(&mut self, language: Option<Language>) {
        let count = self.language_counts.entry(language).or_insert(0);
        *count += 1;
    }

    fn add_entity_count(&mut self) {
        self.entity_count += 1;
    }

    fn add_entity_length_count(&mut self, entity: &str) {
        self.entity_length_count += (entity.len() as u32);
    }

    fn map_counts_to_accuracy_values(&mut self) {
        let sum_of_counts: u32 = self.language_counts.values().sum();
        self.language_accuracies = self
            .language_counts
            .iter()
            .map(|(language, count)| {
                (
                    language.clone(),
                    (*count as f64) / (sum_of_counts as f64) * 100.0,
                )
            })
            .collect();
    }

    fn create_report_data(&self, language: &Language, description: &str) -> (f64, String) {
        let accuracy = *self
            .language_accuracies
            .get(&Some(language.clone()))
            .unwrap_or(&0.0);

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

    fn format_language_accuracies(&self, language: &Language) -> String {
        self.language_accuracies
            .iter()
            .filter(|(lang, _)| lang.as_ref() != Some(language))
            .sorted_by(|(_, &first_accuracy), (_, &second_accuracy)| {
                second_accuracy.partial_cmp(&first_accuracy).unwrap()
            })
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

fn main() {
    let lingua_detector = LanguageDetectorBuilder::from_all_languages().build();
    let whatlang_detector = Detector::new();

    let accuracy_reports_directory = Path::new("accuracy-reports");
    let lingua_reports_directory = accuracy_reports_directory.join("lingua");
    let whatlang_reports_directory = accuracy_reports_directory.join("whatlang");

    if !lingua_reports_directory.is_dir() {
        fs::create_dir_all(&lingua_reports_directory)
            .expect("Lingua reports directory could not be created");
    }

    if !whatlang_reports_directory.is_dir() {
        fs::create_dir_all(&whatlang_reports_directory)
            .expect("Whatlang reports directory could not be created");
    }

    let aggregated_report_file_path =
        accuracy_reports_directory.join("aggregated-accuracy-values.csv");
    let mut aggregated_report_file =
        fs::File::create(aggregated_report_file_path).expect("CSV file could not be created");
    let aggregated_report_columns = vec![
        "language",
        "average-lingua",
        "single-words-lingua",
        "word-pairs-lingua",
        "sentences-lingua",
        "average-whatlang",
        "single-words-whatlang",
        "word-pairs-whatlang",
        "sentences-whatlang\n",
    ];

    aggregated_report_file
        .write_all(aggregated_report_columns.iter().join(",").as_bytes())
        .expect("CSV header row could not be written");

    let total_language_count = Language::iter().count();

    for (idx, language) in Language::iter().enumerate() {
        println!(
            "Writing reports for {:?}... ({}/{})",
            &language,
            (idx + 1),
            total_language_count
        );

        let single_words = get_file_content("single-words", &language);
        let word_pairs = get_file_content("word-pairs", &language);
        let sentences = get_file_content("sentences", &language);

        let mut lingua_statistics = DetectorStatistics::new();
        let mut whatlang_statistics = DetectorStatistics::new();

        for single_word in single_words {
            let lingua_language = lingua_detector.detect_language_of(single_word);
            lingua_statistics.add_single_word_counts(lingua_language, single_word);

            let whatlang_language =
                map_whatlang_to_lingua(whatlang_detector.detect_lang(single_word));
            whatlang_statistics.add_single_word_counts(whatlang_language, single_word);
        }

        for word_pair in word_pairs {
            let lingua_language = lingua_detector.detect_language_of(word_pair);
            lingua_statistics.add_word_pair_counts(lingua_language, word_pair);

            let whatlang_language =
                map_whatlang_to_lingua(whatlang_detector.detect_lang(word_pair));
            whatlang_statistics.add_word_pair_counts(whatlang_language, word_pair);
        }

        for sentence in sentences {
            let lingua_language = lingua_detector.detect_language_of(sentence);
            lingua_statistics.add_sentence_counts(lingua_language, sentence);

            let whatlang_language = map_whatlang_to_lingua(whatlang_detector.detect_lang(sentence));
            whatlang_statistics.add_sentence_counts(whatlang_language, sentence);
        }

        lingua_statistics.compute_accuracy_values();
        whatlang_statistics.compute_accuracy_values();

        let lingua_report = lingua_statistics.create_report_data(&language);
        let whatlang_report = whatlang_statistics.create_report_data(&language);

        let lingua_aggregated_report_row =
            lingua_statistics.create_aggregated_report_row(&language);
        let whatlang_aggregated_report_row =
            whatlang_statistics.create_aggregated_report_row(&language);
        let total_aggregated_report_row = format!(
            "{:?},{},{}\n",
            &language, lingua_aggregated_report_row, whatlang_aggregated_report_row
        );

        aggregated_report_file
            .write_all(total_aggregated_report_row.as_bytes())
            .expect("CSV data row could not be written");

        let report_file_name = titlecase(&format!("{:?}.txt", &language));
        let lingua_reports_file_path = lingua_reports_directory.join(&report_file_name);
        let whatlang_reports_file_path = whatlang_reports_directory.join(&report_file_name);

        if let Some(report) = lingua_report {
            fs::write(lingua_reports_file_path, report)
                .expect("Lingua reports file could not be written");
        }

        if let Some(report) = whatlang_report {
            fs::write(whatlang_reports_file_path, report)
                .expect("Whatlang reports file could not be written");
        }

        println!("Done\n");
    }

    println!("All accuracy reports written successfully");
}

fn get_file_content<'a>(subdirectory: &'a str, language: &'a Language) -> Vec<&'a str> {
    let iso_code = language.iso_code_639_1().to_string();
    let file_path = format!("{}/{}.txt", subdirectory, iso_code);
    TEST_DATA_DIRECTORY
        .get_file(file_path)
        .unwrap()
        .contents_utf8()
        .unwrap()
        .split("\n")
        .filter(|&line| !line.trim().is_empty())
        .collect_vec()
}

fn format_accuracy(accuracy: f64) -> String {
    format!("{:.2}%", accuracy)
}

fn map_whatlang_to_lingua(language: Option<Lang>) -> Option<Language> {
    match language {
        Some(Lang::Afr) => Some(Language::Afrikaans),
        Some(Lang::Arb) => Some(Language::Arabic),
        Some(Lang::Azj) => Some(Language::Azerbaijani),
        Some(Lang::Bel) => Some(Language::Belarusian),
        Some(Lang::Ben) => Some(Language::Bengali),
        Some(Lang::Bul) => Some(Language::Bulgarian),
        Some(Lang::Cat) => Some(Language::Catalan),
        Some(Lang::Ces) => Some(Language::Czech),
        Some(Lang::Cmn) => Some(Language::Chinese),
        Some(Lang::Dan) => Some(Language::Danish),
        Some(Lang::Deu) => Some(Language::German),
        Some(Lang::Ell) => Some(Language::Greek),
        Some(Lang::Eng) => Some(Language::English),
        Some(Lang::Epo) => Some(Language::Esperanto),
        Some(Lang::Est) => Some(Language::Estonian),
        Some(Lang::Fin) => Some(Language::Finnish),
        Some(Lang::Fra) => Some(Language::French),
        Some(Lang::Guj) => Some(Language::Gujarati),
        Some(Lang::Heb) => Some(Language::Hebrew),
        Some(Lang::Hin) => Some(Language::Hindi),
        Some(Lang::Hrv) => Some(Language::Croatian),
        Some(Lang::Hun) => Some(Language::Hungarian),
        Some(Lang::Ind) => Some(Language::Indonesian),
        Some(Lang::Ita) => Some(Language::Italian),
        Some(Lang::Jpn) => Some(Language::Japanese),
        Some(Lang::Kat) => Some(Language::Georgian),
        Some(Lang::Kor) => Some(Language::Korean),
        Some(Lang::Lat) => Some(Language::Latin),
        Some(Lang::Lav) => Some(Language::Latvian),
        Some(Lang::Lit) => Some(Language::Lithuanian),
        Some(Lang::Mar) => Some(Language::Marathi),
        Some(Lang::Mkd) => Some(Language::Macedonian),
        Some(Lang::Nld) => Some(Language::Dutch),
        Some(Lang::Nno) => Some(Language::Nynorsk),
        Some(Lang::Nob) => Some(Language::Bokmal),
        Some(Lang::Pan) => Some(Language::Punjabi),
        Some(Lang::Pes) => Some(Language::Persian),
        Some(Lang::Pol) => Some(Language::Polish),
        Some(Lang::Por) => Some(Language::Portuguese),
        Some(Lang::Ron) => Some(Language::Romanian),
        Some(Lang::Rus) => Some(Language::Russian),
        Some(Lang::Slk) => Some(Language::Slovak),
        Some(Lang::Slv) => Some(Language::Slovene),
        Some(Lang::Sna) => Some(Language::Shona),
        Some(Lang::Som) => Some(Language::Somali),
        Some(Lang::Spa) => Some(Language::Spanish),
        Some(Lang::Srp) => Some(Language::Serbian),
        Some(Lang::Swe) => Some(Language::Swedish),
        Some(Lang::Tam) => Some(Language::Tamil),
        Some(Lang::Tel) => Some(Language::Telugu),
        Some(Lang::Tgl) => Some(Language::Tagalog),
        Some(Lang::Tha) => Some(Language::Thai),
        Some(Lang::Tur) => Some(Language::Turkish),
        Some(Lang::Ukr) => Some(Language::Ukrainian),
        Some(Lang::Urd) => Some(Language::Urdu),
        Some(Lang::Vie) => Some(Language::Vietnamese),
        Some(Lang::Yor) => Some(Language::Yoruba),
        Some(Lang::Zul) => Some(Language::Zulu),
        _ => None,
    }
}
