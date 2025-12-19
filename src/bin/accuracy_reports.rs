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
use clap::Parser;
use cld2::{Format, Lang as CLD2Language, detect_language as cld2_detect_language};
use fraction::Zero;
use include_dir::Dir;
use indoc::formatdoc;
use itertools::Itertools;
use polars::prelude::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use titlecase::titlecase;
use whatlang::{Detector, Lang as WhatlangLanguage};
use whichlang::{Lang as WhichlangLanguage, detect_language as whichlang_detect_language};

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua_afrikaans_language_model::AFRIKAANS_TESTDATA_DIRECTORY;
use lingua_albanian_language_model::ALBANIAN_TESTDATA_DIRECTORY;
use lingua_arabic_language_model::ARABIC_TESTDATA_DIRECTORY;
use lingua_armenian_language_model::ARMENIAN_TESTDATA_DIRECTORY;
use lingua_azerbaijani_language_model::AZERBAIJANI_TESTDATA_DIRECTORY;
use lingua_basque_language_model::BASQUE_TESTDATA_DIRECTORY;
use lingua_belarusian_language_model::BELARUSIAN_TESTDATA_DIRECTORY;
use lingua_bengali_language_model::BENGALI_TESTDATA_DIRECTORY;
use lingua_bokmal_language_model::BOKMAL_TESTDATA_DIRECTORY;
use lingua_bosnian_language_model::BOSNIAN_TESTDATA_DIRECTORY;
use lingua_bulgarian_language_model::BULGARIAN_TESTDATA_DIRECTORY;
use lingua_catalan_language_model::CATALAN_TESTDATA_DIRECTORY;
use lingua_chinese_language_model::CHINESE_TESTDATA_DIRECTORY;
use lingua_croatian_language_model::CROATIAN_TESTDATA_DIRECTORY;
use lingua_czech_language_model::CZECH_TESTDATA_DIRECTORY;
use lingua_danish_language_model::DANISH_TESTDATA_DIRECTORY;
use lingua_dutch_language_model::DUTCH_TESTDATA_DIRECTORY;
use lingua_english_language_model::ENGLISH_TESTDATA_DIRECTORY;
use lingua_esperanto_language_model::ESPERANTO_TESTDATA_DIRECTORY;
use lingua_estonian_language_model::ESTONIAN_TESTDATA_DIRECTORY;
use lingua_finnish_language_model::FINNISH_TESTDATA_DIRECTORY;
use lingua_french_language_model::FRENCH_TESTDATA_DIRECTORY;
use lingua_ganda_language_model::GANDA_TESTDATA_DIRECTORY;
use lingua_georgian_language_model::GEORGIAN_TESTDATA_DIRECTORY;
use lingua_german_language_model::GERMAN_TESTDATA_DIRECTORY;
use lingua_greek_language_model::GREEK_TESTDATA_DIRECTORY;
use lingua_gujarati_language_model::GUJARATI_TESTDATA_DIRECTORY;
use lingua_hebrew_language_model::HEBREW_TESTDATA_DIRECTORY;
use lingua_hindi_language_model::HINDI_TESTDATA_DIRECTORY;
use lingua_hungarian_language_model::HUNGARIAN_TESTDATA_DIRECTORY;
use lingua_icelandic_language_model::ICELANDIC_TESTDATA_DIRECTORY;
use lingua_indonesian_language_model::INDONESIAN_TESTDATA_DIRECTORY;
use lingua_irish_language_model::IRISH_TESTDATA_DIRECTORY;
use lingua_italian_language_model::ITALIAN_TESTDATA_DIRECTORY;
use lingua_japanese_language_model::JAPANESE_TESTDATA_DIRECTORY;
use lingua_kazakh_language_model::KAZAKH_TESTDATA_DIRECTORY;
use lingua_korean_language_model::KOREAN_TESTDATA_DIRECTORY;
use lingua_latin_language_model::LATIN_TESTDATA_DIRECTORY;
use lingua_latvian_language_model::LATVIAN_TESTDATA_DIRECTORY;
use lingua_lithuanian_language_model::LITHUANIAN_TESTDATA_DIRECTORY;
use lingua_macedonian_language_model::MACEDONIAN_TESTDATA_DIRECTORY;
use lingua_malay_language_model::MALAY_TESTDATA_DIRECTORY;
use lingua_maori_language_model::MAORI_TESTDATA_DIRECTORY;
use lingua_marathi_language_model::MARATHI_TESTDATA_DIRECTORY;
use lingua_mongolian_language_model::MONGOLIAN_TESTDATA_DIRECTORY;
use lingua_nynorsk_language_model::NYNORSK_TESTDATA_DIRECTORY;
use lingua_persian_language_model::PERSIAN_TESTDATA_DIRECTORY;
use lingua_polish_language_model::POLISH_TESTDATA_DIRECTORY;
use lingua_portuguese_language_model::PORTUGUESE_TESTDATA_DIRECTORY;
use lingua_punjabi_language_model::PUNJABI_TESTDATA_DIRECTORY;
use lingua_romanian_language_model::ROMANIAN_TESTDATA_DIRECTORY;
use lingua_russian_language_model::RUSSIAN_TESTDATA_DIRECTORY;
use lingua_serbian_language_model::SERBIAN_TESTDATA_DIRECTORY;
use lingua_shona_language_model::SHONA_TESTDATA_DIRECTORY;
use lingua_slovak_language_model::SLOVAK_TESTDATA_DIRECTORY;
use lingua_slovene_language_model::SLOVENE_TESTDATA_DIRECTORY;
use lingua_somali_language_model::SOMALI_TESTDATA_DIRECTORY;
use lingua_sotho_language_model::SOTHO_TESTDATA_DIRECTORY;
use lingua_spanish_language_model::SPANISH_TESTDATA_DIRECTORY;
use lingua_swahili_language_model::SWAHILI_TESTDATA_DIRECTORY;
use lingua_swedish_language_model::SWEDISH_TESTDATA_DIRECTORY;
use lingua_tagalog_language_model::TAGALOG_TESTDATA_DIRECTORY;
use lingua_tamil_language_model::TAMIL_TESTDATA_DIRECTORY;
use lingua_telugu_language_model::TELUGU_TESTDATA_DIRECTORY;
use lingua_thai_language_model::THAI_TESTDATA_DIRECTORY;
use lingua_tsonga_language_model::TSONGA_TESTDATA_DIRECTORY;
use lingua_tswana_language_model::TSWANA_TESTDATA_DIRECTORY;
use lingua_turkish_language_model::TURKISH_TESTDATA_DIRECTORY;
use lingua_ukrainian_language_model::UKRAINIAN_TESTDATA_DIRECTORY;
use lingua_urdu_language_model::URDU_TESTDATA_DIRECTORY;
use lingua_vietnamese_language_model::VIETNAMESE_TESTDATA_DIRECTORY;
use lingua_welsh_language_model::WELSH_TESTDATA_DIRECTORY;
use lingua_xhosa_language_model::XHOSA_TESTDATA_DIRECTORY;
use lingua_yoruba_language_model::YORUBA_TESTDATA_DIRECTORY;
use lingua_zulu_language_model::ZULU_TESTDATA_DIRECTORY;

#[derive(Clone, EnumIter, Eq, PartialEq)]
enum Category {
    Average,
    SingleWords,
    WordPairs,
    Sentences,
}

impl Category {
    fn test_data_file_name(&self) -> Option<&'static str> {
        match self {
            Category::Average => None,
            Category::SingleWords => Some("single-words.txt"),
            Category::WordPairs => Some("word-pairs.txt"),
            Category::Sentences => Some("sentences.txt"),
        }
    }

    fn aggregated_report_file_name(&self) -> &'static str {
        match self {
            Category::Average => "average-accuracy-values.csv",
            Category::SingleWords => "single-words-accuracy-values.csv",
            Category::WordPairs => "word-pairs-accuracy-values.csv",
            Category::Sentences => "sentences-accuracy-values.csv",
        }
    }
}

struct DetectorStatistics {
    detector_name: String,
    is_single_language_detector: bool,
    language: Language,
    single_word_statistic: Statistic,
    word_pair_statistic: Statistic,
    sentence_statistic: Statistic,
    single_word_accuracy: f64,
    word_pair_accuracy: f64,
    sentence_accuracy: f64,
    average_accuracy: f64,
}

impl DetectorStatistics {
    fn new(detector_name: &str, is_single_language_detector: bool, language: Language) -> Self {
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

    fn create_report_data(&mut self) -> Option<String> {
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

        if self.average_accuracy.is_zero() {
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

    fn to_dataframe(&self, category: Category) -> DataFrame {
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
        self.entity_length_count += entity.chars().count() as u32;
    }

    fn map_counts_to_accuracy_values(&mut self) {
        let sum_of_counts: u32 = self.language_counts.values().sum();
        self.language_accuracies = self
            .language_counts
            .iter()
            .map(|(language, count)| (*language, *count as f64 / sum_of_counts as f64))
            .collect();
    }

    fn create_report_data(&self, language: &Option<Language>, description: &str) -> (f64, String) {
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

    fn format_language_accuracies(&self, language: &Option<Language>) -> String {
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

trait LanguageDetection {
    fn detector_name(&self) -> String;

    fn languages(&self) -> &Vec<Language>;

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>>;

    fn is_single_language_detector(&self) -> bool {
        false
    }

    fn reports_directory(&self) -> PathBuf {
        Path::new("accuracy-reports").join(self.detector_name())
    }

    fn single_words(&self) -> HashMap<Language, Vec<&str>> {
        self.get_file_content(Category::SingleWords.test_data_file_name().unwrap())
    }

    fn word_pairs(&self) -> HashMap<Language, Vec<&str>> {
        self.get_file_content(Category::WordPairs.test_data_file_name().unwrap())
    }

    fn sentences(&self) -> HashMap<Language, Vec<&str>> {
        self.get_file_content(Category::Sentences.test_data_file_name().unwrap())
    }

    fn get_file_content(&self, file_name: &str) -> HashMap<Language, Vec<&str>> {
        self.languages()
            .iter()
            .map(|&language| {
                let file_content = get_test_data_directory(&language)
                    .get_file(file_name)
                    .unwrap()
                    .contents_utf8()
                    .unwrap()
                    .split('\n')
                    .filter(|&line| !line.trim().is_empty())
                    .collect_vec();

                (language, file_content)
            })
            .collect()
    }

    fn collect_statistics(&self) -> Vec<DetectorStatistics> {
        let reports_directory = self.reports_directory();

        if !reports_directory.is_dir() {
            fs::create_dir_all(reports_directory).expect("Reports directory could not be created");
        }

        let all_single_words = self.single_words();
        let all_word_pairs = self.word_pairs();
        let all_sentences = self.sentences();

        let total_language_count = self.languages().len();
        let mut all_statistics = vec![];

        for (idx, language) in self.languages().iter().enumerate() {
            let step = format!("({i}/{total_language_count})", i = idx + 1);
            println!(
                "Collecting {} statistics for {}... {step}",
                self.detector_name(),
                language
            );

            let mut statistics = DetectorStatistics::new(
                &self.detector_name(),
                self.is_single_language_detector(),
                *language,
            );

            let single_words = all_single_words.get(language).unwrap();
            let detected_languages = self.detect(single_words);
            zip(single_words, detected_languages).for_each(|(single_word, detected_language)| {
                statistics.add_single_word_counts(detected_language, single_word);
            });

            let word_pairs = all_word_pairs.get(language).unwrap();
            let detected_languages = self.detect(word_pairs);
            zip(word_pairs, detected_languages).for_each(|(word_pair, detected_language)| {
                statistics.add_word_pair_counts(detected_language, word_pair);
            });

            let sentences = all_sentences.get(language).unwrap();
            let detected_languages = self.detect(sentences);
            zip(sentences, detected_languages).for_each(|(sentence, detected_language)| {
                statistics.add_sentence_counts(detected_language, sentence);
            });

            statistics.compute_accuracy_values();

            all_statistics.push(statistics);
        }

        all_statistics
    }

    fn write_reports(&self, statistics: &mut [DetectorStatistics]) {
        for stat in statistics {
            if let Some(report) = stat.create_report_data() {
                let report_file_name = titlecase(&format!("{:?}.txt", stat.language));
                let report_file_path = self.reports_directory().join(&report_file_name);
                fs::write(report_file_path, report).expect("Reports file could not be written");
            }
        }
    }
}

struct CLD2Detector {
    languages: Vec<Language>,
}

impl CLD2Detector {
    fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
        }
    }

    fn map_language_to_lingua(&self, language: Option<CLD2Language>) -> Option<Language> {
        match language {
            Some(cld2_language) => Language::iter().find(|lingua_language| {
                cld2_language.0 == lingua_language.iso_code_639_1().to_string()
            }),
            None => None,
        }
    }
}

impl LanguageDetection for CLD2Detector {
    fn detector_name(&self) -> String {
        "cld2".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        texts
            .iter()
            .map(|text| self.map_language_to_lingua(cld2_detect_language(text, Format::Text).0))
            .collect()
    }
}

struct LinguaLowAccuracyDetector {
    languages: Vec<Language>,
    detector: LanguageDetector,
}

impl LinguaLowAccuracyDetector {
    fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
            detector: LanguageDetectorBuilder::from_all_languages()
                .with_low_accuracy_mode()
                .with_preloaded_language_models()
                .build(),
        }
    }
}

impl LanguageDetection for LinguaLowAccuracyDetector {
    fn detector_name(&self) -> String {
        "lingua-low-accuracy".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        self.detector.detect_languages_in_parallel_of(texts)
    }
}

struct LinguaHighAccuracyDetector {
    languages: Vec<Language>,
    detector: LanguageDetector,
}

impl LinguaHighAccuracyDetector {
    fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
            detector: LanguageDetectorBuilder::from_all_languages()
                .with_preloaded_language_models()
                .build(),
        }
    }
}

impl LanguageDetection for LinguaHighAccuracyDetector {
    fn detector_name(&self) -> String {
        "lingua-high-accuracy".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        self.detector.detect_languages_in_parallel_of(texts)
    }
}

struct LinguaSingleLanguageDetector {
    language: Language,
    languages: Vec<Language>,
    detector: LanguageDetector,
}

impl LinguaSingleLanguageDetector {
    fn new(language: Language, languages: &[Language]) -> Self {
        Self {
            language,
            languages: languages.to_vec(),
            detector: LanguageDetectorBuilder::from_languages(&[language]).build(),
        }
    }
}

impl LanguageDetection for LinguaSingleLanguageDetector {
    fn detector_name(&self) -> String {
        format!(
            "lingua-{}-detector",
            self.language.to_string().to_lowercase()
        )
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        self.detector.detect_languages_in_parallel_of(texts)
    }

    fn is_single_language_detector(&self) -> bool {
        true
    }
}

struct WhatlangDetector {
    languages: Vec<Language>,
    detector: Detector,
}

impl WhatlangDetector {
    fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
            detector: Detector::new(),
        }
    }

    fn map_language_to_lingua(&self, language: Option<WhatlangLanguage>) -> Option<Language> {
        match language {
            Some(WhatlangLanguage::Afr) => Some(Language::Afrikaans),
            Some(WhatlangLanguage::Ara) => Some(Language::Arabic),
            Some(WhatlangLanguage::Aze) => Some(Language::Azerbaijani),
            Some(WhatlangLanguage::Bel) => Some(Language::Belarusian),
            Some(WhatlangLanguage::Ben) => Some(Language::Bengali),
            Some(WhatlangLanguage::Bul) => Some(Language::Bulgarian),
            Some(WhatlangLanguage::Cat) => Some(Language::Catalan),
            Some(WhatlangLanguage::Ces) => Some(Language::Czech),
            Some(WhatlangLanguage::Cmn) => Some(Language::Chinese),
            Some(WhatlangLanguage::Dan) => Some(Language::Danish),
            Some(WhatlangLanguage::Deu) => Some(Language::German),
            Some(WhatlangLanguage::Ell) => Some(Language::Greek),
            Some(WhatlangLanguage::Eng) => Some(Language::English),
            Some(WhatlangLanguage::Epo) => Some(Language::Esperanto),
            Some(WhatlangLanguage::Est) => Some(Language::Estonian),
            Some(WhatlangLanguage::Fin) => Some(Language::Finnish),
            Some(WhatlangLanguage::Fra) => Some(Language::French),
            Some(WhatlangLanguage::Guj) => Some(Language::Gujarati),
            Some(WhatlangLanguage::Heb) => Some(Language::Hebrew),
            Some(WhatlangLanguage::Hin) => Some(Language::Hindi),
            Some(WhatlangLanguage::Hrv) => Some(Language::Croatian),
            Some(WhatlangLanguage::Hun) => Some(Language::Hungarian),
            Some(WhatlangLanguage::Ind) => Some(Language::Indonesian),
            Some(WhatlangLanguage::Ita) => Some(Language::Italian),
            Some(WhatlangLanguage::Jpn) => Some(Language::Japanese),
            Some(WhatlangLanguage::Kat) => Some(Language::Georgian),
            Some(WhatlangLanguage::Kor) => Some(Language::Korean),
            Some(WhatlangLanguage::Lat) => Some(Language::Latin),
            Some(WhatlangLanguage::Lav) => Some(Language::Latvian),
            Some(WhatlangLanguage::Lit) => Some(Language::Lithuanian),
            Some(WhatlangLanguage::Mar) => Some(Language::Marathi),
            Some(WhatlangLanguage::Mkd) => Some(Language::Macedonian),
            Some(WhatlangLanguage::Nld) => Some(Language::Dutch),
            Some(WhatlangLanguage::Nob) => Some(Language::Bokmal),
            Some(WhatlangLanguage::Pan) => Some(Language::Punjabi),
            Some(WhatlangLanguage::Pes) => Some(Language::Persian),
            Some(WhatlangLanguage::Pol) => Some(Language::Polish),
            Some(WhatlangLanguage::Por) => Some(Language::Portuguese),
            Some(WhatlangLanguage::Ron) => Some(Language::Romanian),
            Some(WhatlangLanguage::Rus) => Some(Language::Russian),
            Some(WhatlangLanguage::Slk) => Some(Language::Slovak),
            Some(WhatlangLanguage::Slv) => Some(Language::Slovene),
            Some(WhatlangLanguage::Sna) => Some(Language::Shona),
            Some(WhatlangLanguage::Spa) => Some(Language::Spanish),
            Some(WhatlangLanguage::Srp) => Some(Language::Serbian),
            Some(WhatlangLanguage::Swe) => Some(Language::Swedish),
            Some(WhatlangLanguage::Tam) => Some(Language::Tamil),
            Some(WhatlangLanguage::Tel) => Some(Language::Telugu),
            Some(WhatlangLanguage::Tha) => Some(Language::Thai),
            Some(WhatlangLanguage::Tur) => Some(Language::Turkish),
            Some(WhatlangLanguage::Ukr) => Some(Language::Ukrainian),
            Some(WhatlangLanguage::Urd) => Some(Language::Urdu),
            Some(WhatlangLanguage::Vie) => Some(Language::Vietnamese),
            Some(WhatlangLanguage::Zul) => Some(Language::Zulu),
            _ => None,
        }
    }
}

impl LanguageDetection for WhatlangDetector {
    fn detector_name(&self) -> String {
        "whatlang".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        texts
            .iter()
            .map(|text| self.map_language_to_lingua(self.detector.detect_lang(text)))
            .collect()
    }
}

struct WhichlangDetector {
    languages: Vec<Language>,
}

impl WhichlangDetector {
    fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
        }
    }

    fn map_language_to_lingua(&self, language: WhichlangLanguage) -> Option<Language> {
        match language {
            WhichlangLanguage::Ara => Some(Language::Arabic),
            WhichlangLanguage::Cmn => Some(Language::Chinese),
            WhichlangLanguage::Deu => Some(Language::German),
            WhichlangLanguage::Eng => Some(Language::English),
            WhichlangLanguage::Fra => Some(Language::French),
            WhichlangLanguage::Hin => Some(Language::Hindi),
            WhichlangLanguage::Ita => Some(Language::Italian),
            WhichlangLanguage::Jpn => Some(Language::Japanese),
            WhichlangLanguage::Kor => Some(Language::Korean),
            WhichlangLanguage::Nld => Some(Language::Dutch),
            WhichlangLanguage::Por => Some(Language::Portuguese),
            WhichlangLanguage::Rus => Some(Language::Russian),
            WhichlangLanguage::Spa => Some(Language::Spanish),
            WhichlangLanguage::Swe => Some(Language::Swedish),
            WhichlangLanguage::Tur => Some(Language::Turkish),
            WhichlangLanguage::Vie => Some(Language::Vietnamese),
        }
    }
}

impl LanguageDetection for WhichlangDetector {
    fn detector_name(&self) -> String {
        "whichlang".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        texts
            .iter()
            .map(|text| self.map_language_to_lingua(whichlang_detect_language(text)))
            .collect()
    }
}

fn create_detector_instance(
    detector_name: &str,
    languages: &[Language],
) -> Option<Box<dyn LanguageDetection>> {
    match detector_name {
        "cld2" => Some(Box::new(CLD2Detector::new(languages))),
        "lingua-high-accuracy" => Some(Box::new(LinguaHighAccuracyDetector::new(languages))),
        "lingua-low-accuracy" => Some(Box::new(LinguaLowAccuracyDetector::new(languages))),
        name if name.starts_with("lingua-") && name.ends_with("-detector") => {
            let name_parts = name.split('-').collect_vec();
            let language_name = name_parts.get(1).unwrap();
            let language = Language::from_str(language_name).unwrap();
            Some(Box::new(LinguaSingleLanguageDetector::new(
                language, languages,
            )))
        }
        "whatlang" => Some(Box::new(WhatlangDetector::new(languages))),
        "whichlang" => Some(Box::new(WhichlangDetector::new(languages))),
        _ => None,
    }
}

fn format_accuracy(accuracy: f64) -> String {
    format!("{:.2}%", accuracy * 100_f64)
}

fn get_dataframe_language_name(df: &DataFrame) -> String {
    df.column("language")
        .unwrap()
        .get(0)
        .unwrap()
        .get_str()
        .unwrap()
        .to_string()
}

fn get_dataframe_detector_name(df: &DataFrame) -> String {
    df.get_column_names().get(1).unwrap().to_string()
}

fn get_dataframe_probability(df: &DataFrame) -> f64 {
    df.get_columns()
        .get(1)
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap()
}

fn dataframe_contains_language(df: &DataFrame, language_name: &str) -> bool {
    if df.column("language").is_err() {
        return false;
    }
    let column_name = "contains_language";
    let contains_language_df = df
        .clone()
        .lazy()
        .select([col("language")
            .eq(lit(language_name))
            .any(true)
            .alias(column_name)])
        .collect()
        .unwrap();

    contains_language_df
        .column(column_name)
        .unwrap()
        .bool()
        .unwrap()
        .get(0)
        .unwrap()
}

fn dataframe_contains_detector(df: &DataFrame, detector_name: &str) -> bool {
    df.column(detector_name).is_ok()
}

fn update_dataframe_with_new_language(main_df: LazyFrame, df: DataFrame) -> LazyFrame {
    let union_args = UnionArgs {
        diagonal: true,
        ..Default::default()
    };
    concat([main_df, df.lazy()], union_args).unwrap()
}

fn update_dataframe_with_new_detector(main_df: LazyFrame, detector_name: &str) -> LazyFrame {
    main_df.with_column(lit("NaN").cast(DataType::Float64).alias(detector_name))
}

fn update_dataframe_with_new_probability(main_df: LazyFrame, df: DataFrame) -> LazyFrame {
    let language_name = get_dataframe_language_name(&df);
    let detector_name = get_dataframe_detector_name(&df);
    let probability = get_dataframe_probability(&df);

    main_df.with_column(
        when(col("language").eq(lit(language_name)))
            .then(lit(probability))
            .otherwise(col(&detector_name))
            .alias(detector_name),
    )
}

fn sort_dataframe(df: DataFrame) -> DataFrame {
    let sorted_columns = &mut df.get_column_names_str()[1..]
        .iter()
        .sorted()
        .map(|&it| col(it))
        .collect_vec();

    sorted_columns.insert(0, col("language"));

    df.lazy()
        .select(sorted_columns)
        .sort(["language"], Default::default())
        .collect()
        .unwrap()
}

fn default_detectors() -> Vec<String> {
    let mut detectors = vec![
        "cld2".to_string(),
        "lingua-high-accuracy".to_string(),
        "lingua-low-accuracy".to_string(),
        "whatlang".to_string(),
        "whichlang".to_string(),
    ];
    for language in Language::iter() {
        detectors.push(format!(
            "lingua-{}-detector",
            language.to_string().to_lowercase()
        ));
    }
    detectors.sort();
    detectors
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, num_args = 1.., default_values_t = default_detectors())]
    detectors: Vec<String>,

    #[arg(value_enum, short, long, num_args = 1.., default_values_t = Language::all())]
    languages: Vec<Language>,
}

fn main() {
    let total_start = Instant::now();
    let cli = Cli::parse();
    let mut detector_names = cli.detectors.iter().cloned().collect_vec();
    let language_names = cli
        .languages
        .iter()
        .map(|it| it.to_string().to_lowercase())
        .collect_vec();
    let mut all_statistics = HashMap::new();
    let all_single_language_detectors_name = "lingua-all-single-language-detectors".to_string();

    if detector_names.contains(&all_single_language_detectors_name) {
        let idx = detector_names
            .iter()
            .position(|detector_name| detector_name == &all_single_language_detectors_name)
            .unwrap();
        detector_names.remove(idx);
        for language_name in language_names.iter() {
            let detector_name = format!("lingua-{language_name}-detector");
            if !detector_names.contains(&detector_name) {
                detector_names.push(detector_name);
            }
        }
    }

    for detector_name in detector_names.iter() {
        if let Some(detector) = create_detector_instance(detector_name, &cli.languages) {
            let start = Instant::now();
            let mut statistics = detector.collect_statistics();
            detector.write_reports(&mut statistics);
            let stop = Instant::now();
            println!(
                "{detector_name} statistics written in {:.2} seconds\n",
                stop.duration_since(start).as_secs_f64()
            );
            all_statistics.insert(detector_name, statistics);
        }
    }

    println!("Updating aggregated reports...");
    let start = Instant::now();

    for category in Category::iter() {
        let report_file_path = format!(
            "accuracy-reports/{}",
            category.aggregated_report_file_name()
        );

        let mut dataframe = match CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(PathBuf::from(&report_file_path)))
        {
            Ok(csv_reader) => csv_reader.finish().unwrap(),
            Err(_) => df!("language" => &language_names).unwrap(),
        };

        for detector_name in detector_names.iter() {
            let statistics = all_statistics.get(detector_name).unwrap();
            let mut lazy_dataframe = dataframe.clone().lazy();

            if !dataframe_contains_detector(&dataframe, detector_name) {
                lazy_dataframe = update_dataframe_with_new_detector(lazy_dataframe, detector_name);
            }

            for stat in statistics {
                let df = stat.to_dataframe(category.clone());
                let language_name = get_dataframe_language_name(&df);

                lazy_dataframe = if dataframe_contains_language(&dataframe, &language_name) {
                    update_dataframe_with_new_probability(lazy_dataframe, df)
                } else {
                    update_dataframe_with_new_language(lazy_dataframe, df)
                };
            }
            dataframe = sort_dataframe(lazy_dataframe.collect().unwrap());
        }

        let mut aggregated_report_file =
            fs::File::create(&report_file_path).expect("CSV file could not be created");

        CsvWriter::new(&mut aggregated_report_file)
            .with_null_value("NaN".to_string())
            .finish(&mut dataframe)
            .expect("Data frame could not be written to CSV file");
    }

    let total_stop = Instant::now();
    let total_time = total_stop.duration_since(total_start);
    let total_minutes = total_time.as_secs() / 60;
    let total_seconds = total_time.as_secs() % 60;

    println!(
        "Aggregated reports updated in {:.2} seconds\n",
        total_stop.duration_since(start).as_secs_f64()
    );
    println!("All reports written in {total_minutes} minutes, {total_seconds} seconds");
}

fn get_test_data_directory(language: &Language) -> Dir<'static> {
    match *language {
        Language::Afrikaans => AFRIKAANS_TESTDATA_DIRECTORY,
        Language::Albanian => ALBANIAN_TESTDATA_DIRECTORY,
        Language::Arabic => ARABIC_TESTDATA_DIRECTORY,
        Language::Armenian => ARMENIAN_TESTDATA_DIRECTORY,
        Language::Azerbaijani => AZERBAIJANI_TESTDATA_DIRECTORY,
        Language::Basque => BASQUE_TESTDATA_DIRECTORY,
        Language::Belarusian => BELARUSIAN_TESTDATA_DIRECTORY,
        Language::Bengali => BENGALI_TESTDATA_DIRECTORY,
        Language::Bokmal => BOKMAL_TESTDATA_DIRECTORY,
        Language::Bosnian => BOSNIAN_TESTDATA_DIRECTORY,
        Language::Bulgarian => BULGARIAN_TESTDATA_DIRECTORY,
        Language::Catalan => CATALAN_TESTDATA_DIRECTORY,
        Language::Chinese => CHINESE_TESTDATA_DIRECTORY,
        Language::Croatian => CROATIAN_TESTDATA_DIRECTORY,
        Language::Czech => CZECH_TESTDATA_DIRECTORY,
        Language::Danish => DANISH_TESTDATA_DIRECTORY,
        Language::Dutch => DUTCH_TESTDATA_DIRECTORY,
        Language::English => ENGLISH_TESTDATA_DIRECTORY,
        Language::Esperanto => ESPERANTO_TESTDATA_DIRECTORY,
        Language::Estonian => ESTONIAN_TESTDATA_DIRECTORY,
        Language::Finnish => FINNISH_TESTDATA_DIRECTORY,
        Language::French => FRENCH_TESTDATA_DIRECTORY,
        Language::Ganda => GANDA_TESTDATA_DIRECTORY,
        Language::Georgian => GEORGIAN_TESTDATA_DIRECTORY,
        Language::German => GERMAN_TESTDATA_DIRECTORY,
        Language::Greek => GREEK_TESTDATA_DIRECTORY,
        Language::Gujarati => GUJARATI_TESTDATA_DIRECTORY,
        Language::Hebrew => HEBREW_TESTDATA_DIRECTORY,
        Language::Hindi => HINDI_TESTDATA_DIRECTORY,
        Language::Hungarian => HUNGARIAN_TESTDATA_DIRECTORY,
        Language::Icelandic => ICELANDIC_TESTDATA_DIRECTORY,
        Language::Indonesian => INDONESIAN_TESTDATA_DIRECTORY,
        Language::Irish => IRISH_TESTDATA_DIRECTORY,
        Language::Italian => ITALIAN_TESTDATA_DIRECTORY,
        Language::Japanese => JAPANESE_TESTDATA_DIRECTORY,
        Language::Kazakh => KAZAKH_TESTDATA_DIRECTORY,
        Language::Korean => KOREAN_TESTDATA_DIRECTORY,
        Language::Latin => LATIN_TESTDATA_DIRECTORY,
        Language::Latvian => LATVIAN_TESTDATA_DIRECTORY,
        Language::Lithuanian => LITHUANIAN_TESTDATA_DIRECTORY,
        Language::Macedonian => MACEDONIAN_TESTDATA_DIRECTORY,
        Language::Malay => MALAY_TESTDATA_DIRECTORY,
        Language::Maori => MAORI_TESTDATA_DIRECTORY,
        Language::Marathi => MARATHI_TESTDATA_DIRECTORY,
        Language::Mongolian => MONGOLIAN_TESTDATA_DIRECTORY,
        Language::Nynorsk => NYNORSK_TESTDATA_DIRECTORY,
        Language::Persian => PERSIAN_TESTDATA_DIRECTORY,
        Language::Polish => POLISH_TESTDATA_DIRECTORY,
        Language::Portuguese => PORTUGUESE_TESTDATA_DIRECTORY,
        Language::Punjabi => PUNJABI_TESTDATA_DIRECTORY,
        Language::Romanian => ROMANIAN_TESTDATA_DIRECTORY,
        Language::Russian => RUSSIAN_TESTDATA_DIRECTORY,
        Language::Serbian => SERBIAN_TESTDATA_DIRECTORY,
        Language::Shona => SHONA_TESTDATA_DIRECTORY,
        Language::Slovak => SLOVAK_TESTDATA_DIRECTORY,
        Language::Slovene => SLOVENE_TESTDATA_DIRECTORY,
        Language::Somali => SOMALI_TESTDATA_DIRECTORY,
        Language::Sotho => SOTHO_TESTDATA_DIRECTORY,
        Language::Spanish => SPANISH_TESTDATA_DIRECTORY,
        Language::Swahili => SWAHILI_TESTDATA_DIRECTORY,
        Language::Swedish => SWEDISH_TESTDATA_DIRECTORY,
        Language::Tagalog => TAGALOG_TESTDATA_DIRECTORY,
        Language::Tamil => TAMIL_TESTDATA_DIRECTORY,
        Language::Telugu => TELUGU_TESTDATA_DIRECTORY,
        Language::Thai => THAI_TESTDATA_DIRECTORY,
        Language::Tsonga => TSONGA_TESTDATA_DIRECTORY,
        Language::Tswana => TSWANA_TESTDATA_DIRECTORY,
        Language::Turkish => TURKISH_TESTDATA_DIRECTORY,
        Language::Ukrainian => UKRAINIAN_TESTDATA_DIRECTORY,
        Language::Urdu => URDU_TESTDATA_DIRECTORY,
        Language::Vietnamese => VIETNAMESE_TESTDATA_DIRECTORY,
        Language::Welsh => WELSH_TESTDATA_DIRECTORY,
        Language::Xhosa => XHOSA_TESTDATA_DIRECTORY,
        Language::Yoruba => YORUBA_TESTDATA_DIRECTORY,
        Language::Zulu => ZULU_TESTDATA_DIRECTORY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    // ##############################
    // FIXTURES
    // ##############################

    #[fixture]
    fn main_dataframe() -> DataFrame {
        df!(
            "language" => [
                Language::English.to_string(),
                Language::German.to_string(),
                Language::Yoruba.to_string()
            ],
            "lingua-high-accuracy" => [73.56271, 66.49882, 12.16833],
            "whatlang" => [43.18733, 27.94481, 35.62811]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unknown_language() -> DataFrame {
        df!(
            "language" => [Language::Danish.to_string()],
            "lingua-high-accuracy" => [32.77125]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unknown_detector() -> DataFrame {
        df!(
            "language" => [Language::German.to_string()],
            "cld2" => [17.39446]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_new_probability() -> DataFrame {
        df!(
            "language" => [Language::English.to_string()],
            "lingua-high-accuracy" => [12.34567]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unsorted_columns() -> DataFrame {
        df!(
            "language" => [
                Language::Yoruba.to_string(),
                Language::German.to_string(),
                Language::English.to_string(),
            ],
            "cld2" => [92.34567, 55.23456, 53.12345],
            "whatlang" => [35.62811, 27.94481, 43.18733],
            "lingua-high-accuracy" => [12.16833, 66.49882, 73.56271],
        )
        .unwrap()
    }

    // ##############################
    // TESTS
    // ##############################

    #[rstest]
    fn test_get_dataframe_language_name(dataframe_with_unknown_language: DataFrame) {
        assert_eq!(
            get_dataframe_language_name(&dataframe_with_unknown_language),
            "Danish"
        );
    }

    #[rstest]
    fn test_get_dataframe_detector_name(dataframe_with_unknown_detector: DataFrame) {
        assert_eq!(
            get_dataframe_detector_name(&dataframe_with_unknown_detector),
            "cld2"
        );
    }

    #[rstest]
    fn test_get_dataframe_probability(dataframe_with_new_probability: DataFrame) {
        assert_eq!(
            get_dataframe_probability(&dataframe_with_new_probability),
            12.34567
        );
    }

    #[rstest]
    fn test_dataframe_contains_language(main_dataframe: DataFrame) {
        assert!(dataframe_contains_language(&main_dataframe, "English"));
        assert!(dataframe_contains_language(&main_dataframe, "German"));
        assert!(dataframe_contains_language(&main_dataframe, "Yoruba"));
        assert!(!dataframe_contains_language(&main_dataframe, "Hindi"));
    }

    #[rstest]
    fn test_dataframe_contains_detector(main_dataframe: DataFrame) {
        assert!(dataframe_contains_detector(
            &main_dataframe,
            "lingua-high-accuracy"
        ));
        assert!(dataframe_contains_detector(&main_dataframe, "whatlang"));
        assert!(!dataframe_contains_detector(&main_dataframe, "cld2"));
    }

    #[rstest]
    fn test_update_dataframe_with_new_probability(
        main_dataframe: DataFrame,
        dataframe_with_new_probability: DataFrame,
    ) {
        let result = update_dataframe_with_new_probability(
            main_dataframe.lazy(),
            dataframe_with_new_probability,
        )
        .collect()
        .unwrap();

        assert_eq!(
            result,
            df!(
                "language" => [
                    Language::English.to_string(),
                    Language::German.to_string(),
                    Language::Yoruba.to_string()
                ],
                "lingua-high-accuracy" => [12.34567, 66.49882, 12.16833],
                "whatlang" => [43.18733, 27.94481, 35.62811]
            )
            .unwrap()
        );
    }

    #[rstest]
    fn test_sort_dataframe_columns(dataframe_with_unsorted_columns: DataFrame) {
        assert_eq!(
            sort_dataframe(dataframe_with_unsorted_columns),
            df!(
                "language" => [
                    Language::English.to_string(),
                    Language::German.to_string(),
                    Language::Yoruba.to_string()
                ],
                "cld2" => [53.12345, 55.23456, 92.34567],
                "lingua-high-accuracy" => [73.56271, 66.49882, 12.16833],
                "whatlang" => [43.18733, 27.94481, 35.62811],
            )
            .unwrap()
        )
    }
}
