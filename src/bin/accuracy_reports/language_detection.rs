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

use crate::statistic::{Category, DetectorStatistics};
use crate::testdata::get_test_data_directory;
use itertools::Itertools;
use lingua::Language;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::{Path, PathBuf};
use titlecase::titlecase;

pub(crate) trait LanguageDetection {
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
