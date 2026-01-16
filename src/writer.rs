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

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::{fs, io};

use crate::Language;
use crate::constant::{DIGITS_AT_BEGINNING, MULTIPLE_WHITESPACE, NUMBERS, PUNCTUATION};
use crate::detector::split_text_into_words;
use crate::file::read_test_data_file;
use crate::model::{
    NGRAM_PROBABILITY_MODEL_FILE_NAME, NgramCountModel, NgramCountModelType,
    TrainingDataLanguageModel, create_fst_map, create_fst_set, get_utf8_slice,
    load_ngram_probability_model,
};
use crate::ngram::Ngram;
use counter::Counter;
use fst::Streamer;
use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator;

pub(crate) const LANGUAGES_MESSAGE: &str = "Set of languages must not be empty";

pub(crate) const MOST_COMMON_NGRAMS_MESSAGE: &str =
    "Amount of most common ngrams must be greater than zero";

/// This struct creates language model files and writes them to a directory.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(module = "lingua"))]
pub struct LanguageModelFilesWriter;

/// This struct creates test data files for accuracy report generation
/// and writes them to a directory.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(module = "lingua"))]
pub struct TestDataFilesWriter;

/// This struct determines ngrams being unique to any specific language
/// and writes them to a directory.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(module = "lingua"))]
pub struct UniqueNgramsWriter;

/// This struct determines the most common ngrams for each supported language
/// and writes them to a directory.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(module = "lingua"))]
pub struct MostCommonNgramsWriter;

impl LanguageModelFilesWriter {
    /// Creates language model files and writes them to a directory.
    ///
    /// `input_file_path`: The path to a txt file used for language model creation.
    /// The assumed encoding of the txt file is UTF-8.
    ///
    /// `output_directory_path`: The path to an existing directory where the language model files
    /// are to be written.
    ///
    /// `language`: The language for which to create language models.
    ///
    /// `char_class`: A regex character class such as `\\p{L}` to restrict the set of characters
    /// that the language models are built from.
    ///
    /// ⚠ Panics if:
    /// - the input file path is not absolute or does not point to an existing txt file
    /// - the input file's encoding is not UTF-8
    /// - the output directory path is not absolute or does not point to an existing directory
    /// - the character class cannot be compiled to a valid regular expression
    pub fn create_and_write_language_model_files(
        input_file_path: &Path,
        output_directory_path: &Path,
        language: Language,
        char_class: &str,
    ) -> io::Result<()> {
        check_input_file_path(input_file_path);
        check_output_directory_path(output_directory_path);

        let unigram_model =
            Self::create_language_model(input_file_path, &language, 1, char_class, &hashmap!())?;

        let bigram_model = Self::create_language_model(
            input_file_path,
            &language,
            2,
            char_class,
            &unigram_model.absolute_frequencies,
        )?;

        let trigram_model = Self::create_language_model(
            input_file_path,
            &language,
            3,
            char_class,
            &bigram_model.absolute_frequencies,
        )?;

        let quadrigram_model = Self::create_language_model(
            input_file_path,
            &language,
            4,
            char_class,
            &trigram_model.absolute_frequencies,
        )?;

        let fivegram_model = Self::create_language_model(
            input_file_path,
            &language,
            5,
            char_class,
            &quadrigram_model.absolute_frequencies,
        )?;

        Self::write_language_models(
            &[
                &unigram_model,
                &bigram_model,
                &trigram_model,
                &quadrigram_model,
                &fivegram_model,
            ],
            output_directory_path,
            NGRAM_PROBABILITY_MODEL_FILE_NAME,
        )?;

        Ok(())
    }

    fn create_language_model(
        input_file_path: &Path,
        language: &Language,
        ngram_length: usize,
        char_class: &str,
        lower_ngram_absolute_frequencies: &HashMap<Ngram, u32>,
    ) -> io::Result<TrainingDataLanguageModel> {
        let file = fs::File::open(input_file_path)?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.trim().is_empty())
            .collect_vec();
        let lines_as_str = lines.iter().map(|line| line.as_str()).collect_vec();

        Ok(TrainingDataLanguageModel::from_text(
            &lines_as_str,
            language,
            ngram_length,
            char_class,
            lower_ngram_absolute_frequencies,
        ))
    }

    fn write_language_models(
        models: &[&TrainingDataLanguageModel],
        output_directory_path: &Path,
        file_name: &str,
    ) -> io::Result<()> {
        let mut kvs = vec![];

        for model in models {
            let mut stream = model.ngram_probability_model.ngrams.stream();
            while let Some((key, value)) = stream.next() {
                kvs.push((key.to_vec(), value));
            }
        }

        let fst_map = create_fst_map(kvs);
        let file_path = output_directory_path.join(file_name);

        fs::write(file_path, fst_map.as_fst().as_bytes())?;

        Ok(())
    }
}

impl TestDataFilesWriter {
    /// Creates test data files for accuracy report generation and writes them to a directory.
    ///
    /// `input_file_path`: The path to a txt file used for test data creation.
    /// The assumed encoding of the txt file is UTF-8.
    ///
    /// `output_directory_path`: The path to an existing directory where the test data files
    /// are to be written.
    ///
    /// `char_class`: A regex character class such as `\\p{L}` to restrict the set of characters
    /// that the test data are built from.
    ///
    /// `maximum_lines`: The maximum number of lines each test data file should have.
    ///
    /// ⚠ Panics if:
    /// - the input file path is not absolute or does not point to an existing txt file
    /// - the input file's encoding is not UTF-8
    /// - the output directory path is not absolute or does not point to an existing directory
    /// - the character class cannot be compiled to a valid regular expression
    pub fn create_and_write_test_data_files(
        input_file_path: &Path,
        output_directory_path: &Path,
        char_class: &str,
        maximum_lines: u32,
    ) -> io::Result<()> {
        check_input_file_path(input_file_path);
        check_output_directory_path(output_directory_path);

        Self::create_and_write_sentences_file(
            input_file_path,
            output_directory_path,
            maximum_lines,
        )?;

        let single_words = Self::create_and_write_single_words_file(
            input_file_path,
            output_directory_path,
            char_class,
            maximum_lines,
        )?;

        Self::create_and_write_word_pairs_file(single_words, output_directory_path, maximum_lines)?;

        Ok(())
    }

    fn create_and_write_sentences_file(
        input_file_path: &Path,
        output_directory_path: &Path,
        maximum_lines: u32,
    ) -> io::Result<()> {
        let sentences_file_path = output_directory_path.join("sentences.txt");

        if sentences_file_path.is_file() {
            fs::remove_file(&sentences_file_path)?;
        }

        let input_lines_count = BufReader::new(fs::File::open(input_file_path)?)
            .lines()
            .count();
        let input_lines = BufReader::new(fs::File::open(input_file_path)?)
            .lines()
            .map(|line| line.unwrap());

        let sentences_file = fs::File::create(sentences_file_path)?;
        let mut sentences_writer = LineWriter::new(sentences_file);

        let mut line_counter = 0;
        let mut random_line_numbers = HashSet::new();

        loop {
            let n = fastrand::usize(0..input_lines_count);
            random_line_numbers.insert(n);
            if random_line_numbers.len() as u32 == maximum_lines {
                break;
            }
        }

        for (i, line) in input_lines.enumerate() {
            if !random_line_numbers.contains(&i) {
                continue;
            }

            let normalized_whitespace = MULTIPLE_WHITESPACE.replace_all(&line, " ");
            let removed_sentence_numbers = DIGITS_AT_BEGINNING.replace(&normalized_whitespace, "");
            let removed_quotes = removed_sentence_numbers.replace('\"', "");

            if line_counter < maximum_lines {
                sentences_writer.write_all(removed_quotes.as_bytes())?;
                sentences_writer.write_all(b"\n")?;
                line_counter += 1;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn create_and_write_single_words_file(
        input_file_path: &Path,
        output_directory_path: &Path,
        char_class: &str,
        maximum_lines: u32,
    ) -> io::Result<Vec<String>> {
        let single_words_file_path = output_directory_path.join("single-words.txt");
        let word_regex = Regex::new(&format!("[{char_class}]{{5,}}")).unwrap();
        let mut words = vec![];

        if single_words_file_path.is_file() {
            fs::remove_file(&single_words_file_path)?;
        }

        let input_file = fs::File::open(input_file_path)?;
        let input_lines = BufReader::new(input_file).lines().map(|line| line.unwrap());

        let single_words_file = fs::File::create(single_words_file_path)?;
        let mut single_words_writer = LineWriter::new(single_words_file);

        let mut line_counter = 0;

        for line in input_lines {
            let removed_punctuation = PUNCTUATION.replace_all(&line, "");
            let removed_numbers = NUMBERS.replace_all(&removed_punctuation, "");
            let normalized_whitespace = MULTIPLE_WHITESPACE.replace_all(&removed_numbers, " ");
            let removed_quotes = normalized_whitespace.replace('\"', "");
            let mut single_words = removed_quotes
                .split(' ')
                .map(|word| word.trim().to_lowercase())
                .filter(|word| word_regex.is_match(word))
                .collect_vec();

            words.append(&mut single_words);
        }

        for word in words.iter() {
            if line_counter < maximum_lines {
                single_words_writer.write_all(word.as_bytes())?;
                single_words_writer.write_all(b"\n")?;
                line_counter += 1;
            } else {
                break;
            }
        }

        Ok(words)
    }

    fn create_and_write_word_pairs_file(
        words: Vec<String>,
        output_directory_path: &Path,
        maximum_lines: u32,
    ) -> io::Result<()> {
        let word_pairs_file_path = output_directory_path.join("word-pairs.txt");
        let mut word_pairs = Vec::<String>::new();

        if word_pairs_file_path.is_file() {
            fs::remove_file(&word_pairs_file_path)?;
        }

        for i in (0..=(words.len() - 2)).step_by(2) {
            let slice = &words[i..i + 2];
            word_pairs.push(slice.join(" "));
        }

        let word_pairs_file = fs::File::create(word_pairs_file_path)?;
        let mut word_pairs_writer = LineWriter::new(word_pairs_file);
        let mut line_counter = 0;

        for word_pair in word_pairs {
            if line_counter < maximum_lines {
                word_pairs_writer.write_all(word_pair.as_bytes())?;
                word_pairs_writer.write_all(b"\n")?;
                line_counter += 1;
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl UniqueNgramsWriter {
    /// Creates unique ngram files from the current language models and writes them to a directory.
    ///
    /// `output_directory_path`: The path to an existing directory where the unique ngram files
    /// are to be written.
    ///
    /// ⚠ Panics if the output directory path is not absolute or does not point to an existing directory.
    pub fn create_and_write_unique_ngram_files(output_directory_path: &Path) -> io::Result<()> {
        check_output_directory_path(output_directory_path);
        let ngrams = Self::load_ngrams();
        let unique_ngrams = Self::identify_unique_ngrams(ngrams);
        Self::store_unique_ngrams(unique_ngrams, output_directory_path)?;
        Ok(())
    }

    fn load_ngrams() -> HashMap<Language, HashSet<Vec<u8>>> {
        let mut result = HashMap::new();
        for language in Language::iter() {
            if let Some(model) = load_ngram_probability_model(language) {
                let mut stream = model.ngrams.keys();
                let mut ngrams = HashSet::new();
                while let Some(key) = stream.next() {
                    ngrams.insert(key.to_vec());
                }
                result.insert(language, ngrams);
            }
        }
        result
    }

    fn identify_unique_ngrams(ngrams: HashMap<Language, HashSet<Vec<u8>>>) -> Vec<NgramCountModel> {
        let mut unique_ngrams = HashSet::new();
        for ngrams_i in ngrams.values() {
            let mut current = ngrams_i.clone();
            for ngrams_j in ngrams.values() {
                if ngrams_j != ngrams_i {
                    current = &current - ngrams_j;
                }
            }
            unique_ngrams = unique_ngrams.union(&current).cloned().collect();
        }
        let mut result = HashMap::new();
        for unique_ngram in unique_ngrams {
            for (language, ngrams_set) in ngrams.iter() {
                if ngrams_set.contains(&unique_ngram) {
                    if !result.contains_key(language) {
                        result.insert(*language, HashSet::new());
                    }
                    result.get_mut(language).unwrap().insert(unique_ngram);
                    break;
                }
            }
        }
        result
            .into_iter()
            .map(|(language, ngrams)| NgramCountModel {
                language,
                ngrams: create_fst_set(ngrams.into_iter().collect_vec()),
            })
            .sorted_by_key(|model| model.language)
            .collect()
    }

    fn store_unique_ngrams(
        unique_ngrams: Vec<NgramCountModel>,
        output_directory_path: &Path,
    ) -> io::Result<()> {
        store_ngram_count_models(
            unique_ngrams,
            output_directory_path,
            NgramCountModelType::Unique,
        )
    }
}

impl MostCommonNgramsWriter {
    /// Creates most common ngram files from the current language models and writes them to a directory.
    ///
    /// `output_directory_path`: The path to an existing directory where the most common ngram files
    /// are to be written.
    ///
    /// `languages`: The languages to determine the most common ngrams for.
    ///
    /// `most_common`: The amount of most common ngrams to be identified.
    ///
    /// ⚠ Panics if:
    /// - the output directory path is not absolute or does not point to an existing directory
    /// - `languages` is empty
    /// - `most_common` is zero
    pub fn create_and_write_most_common_ngram_files(
        output_directory_path: &Path,
        languages: &HashSet<Language>,
        most_common: u32,
    ) -> io::Result<()> {
        check_output_directory_path(output_directory_path);
        if languages.is_empty() {
            panic!("{LANGUAGES_MESSAGE}");
        }
        if most_common == 0 {
            panic!("{MOST_COMMON_NGRAMS_MESSAGE}");
        }
        let mut most_common_ngrams = vec![];
        for language in languages.iter() {
            most_common_ngrams.push(Self::identify_most_common_ngrams(
                *language,
                most_common as usize,
            ));
        }
        Self::store_most_common_ngrams(most_common_ngrams, output_directory_path)?;
        Ok(())
    }

    fn identify_most_common_ngrams(language: Language, most_common: usize) -> NgramCountModel {
        let sentences = read_test_data_file(language, "sentences.txt").unwrap();
        let words = split_text_into_words(sentences);
        let mut most_common_ngrams = vec![];

        for ngram_length in 1..6 {
            let mut counter = Counter::<&str>::new();

            for word in words.iter() {
                let chars_count = word.chars().count();
                if chars_count >= ngram_length {
                    for i in 0..=chars_count - ngram_length {
                        let slice = get_utf8_slice(word, i, i + ngram_length);
                        for alphabet in language.alphabets() {
                            if alphabet.matches(slice) {
                                counter[&slice] += 1;
                            }
                        }
                    }
                }
            }

            counter
                .k_most_common_ordered(most_common)
                .iter()
                .map(|(ngram, _)| ngram.as_bytes().to_vec())
                .for_each(|ngram| most_common_ngrams.push(ngram));
        }

        NgramCountModel {
            language,
            ngrams: create_fst_set(most_common_ngrams),
        }
    }

    fn store_most_common_ngrams(
        most_common_ngrams: Vec<NgramCountModel>,
        output_directory_path: &Path,
    ) -> io::Result<()> {
        store_ngram_count_models(
            most_common_ngrams,
            output_directory_path,
            NgramCountModelType::MostCommon,
        )
    }
}

fn store_ngram_count_models(
    ngram_count_models: Vec<NgramCountModel>,
    output_directory_path: &Path,
    model_type: NgramCountModelType,
) -> io::Result<()> {
    for model in ngram_count_models {
        let language_dir_path =
            output_directory_path.join(model.language.iso_code_639_1().to_string());
        if !language_dir_path.exists() {
            fs::create_dir(language_dir_path.as_path())?;
        }
        let file_path = language_dir_path.join(model_type.file_name());
        let mut file = fs::File::create(file_path)?;
        file.write_all(model.ngrams.into_fst().as_bytes())?;
    }
    Ok(())
}

fn check_input_file_path(input_file_path: &Path) {
    if !input_file_path.is_absolute() {
        panic!(
            "Input file path '{}' is not absolute",
            input_file_path.display()
        );
    }
    if !input_file_path.exists() {
        panic!("Input file '{}' does not exist", input_file_path.display());
    }
    if !input_file_path.is_file() {
        panic!(
            "Input file path '{}' does not represent a regular file",
            input_file_path.display()
        );
    }
}

fn check_output_directory_path(output_directory_path: &Path) {
    if !output_directory_path.is_absolute() {
        panic!(
            "Output directory path '{}' is not absolute",
            output_directory_path.display()
        );
    }
    if !output_directory_path.exists() {
        panic!(
            "Output directory path '{}' does not exist",
            output_directory_path.display()
        );
    }
    if !output_directory_path.is_dir() {
        panic!(
            "Output directory path '{}' does not represent a directory",
            output_directory_path.display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detector::CountModelFst;
    use fst::IntoStreamer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::{NamedTempFile, tempdir};

    fn create_temp_input_file(content: &str) -> NamedTempFile {
        let mut input_file = NamedTempFile::new().unwrap();
        input_file
            .write_all(content.as_bytes())
            .expect("Text could not be written to temporary input file");
        input_file
    }

    fn read_directory_content(directory: &Path) -> Vec<PathBuf> {
        let mut files = fs::read_dir(directory)
            .unwrap()
            .map(|it| it.unwrap().path())
            .collect_vec();

        files.sort();
        files
    }

    fn count_model_fst(data: HashSet<&'static str>) -> CountModelFst {
        let fst_data = data
            .iter()
            .map(|&value| value.as_bytes().to_vec())
            .collect_vec();

        create_fst_set(fst_data)
    }

    fn bytes_set(data: HashSet<&'static str>) -> HashSet<Vec<u8>> {
        data.into_iter()
            .map(|value| value.as_bytes().to_vec())
            .collect()
    }

    fn check_fst_map_file(
        file_path: &Path,
        expected_file_name: &str,
        expected_file_content: HashMap<String, f64>,
    ) {
        assert!(file_path.is_file());

        let file_name = file_path.file_name().unwrap();
        assert_eq!(file_name, expected_file_name);

        let bytes = fs::read(file_path).unwrap();
        let fst_map = fst::Map::new(bytes).unwrap();

        let mut actual_file_content = hashmap!();
        let mut stream = fst_map.into_stream();
        while let Some((key, value)) = stream.next() {
            actual_file_content.insert(
                String::from_utf8(key.to_vec()).unwrap(),
                f64::from_bits(value),
            );
        }
        assert_eq!(actual_file_content, expected_file_content);
    }

    fn check_fst_set_file(
        file_path: &Path,
        expected_file_name: &str,
        expected_file_content: HashSet<String>,
    ) {
        assert!(file_path.is_file());

        let file_name = file_path.file_name().unwrap();
        assert_eq!(file_name, expected_file_name);

        let bytes = fs::read(file_path).unwrap();
        let fst_set = fst::Set::new(bytes).unwrap();

        let mut actual_file_content = hashset!();
        let mut stream = fst_set.into_stream();
        while let Some(key) = stream.next() {
            actual_file_content.insert(String::from_utf8(key.to_vec()).unwrap());
        }
        assert_eq!(actual_file_content, expected_file_content);
    }

    fn to_string(ngrams: HashSet<&str>) -> HashSet<String> {
        ngrams.into_iter().map(|ngram| ngram.to_string()).collect()
    }

    mod language_model_files {
        use super::*;
        use rstest::*;

        #[fixture]
        fn text() -> &'static str {
            "
            These sentences are intended for testing purposes.
            Do not use them in production!
            By the way, they consist of 23 words in total.
        "
        }

        #[fixture]
        fn low_accuracy_model() -> HashMap<String, f64> {
            hashmap!(
                "n" => 0.1_f64,
                "o" => 0.1,
                "s" => 0.1,
                "b" => 0.01,
                "g" => 0.01,
                "l" => 0.01,
                "m" => 0.01,
                "d" => 0.05,
                "r" => 0.05,
                "h" => 0.04,
                "f" => 0.02,
                "w" => 0.02,
                "t" => 0.13,
                "a" => 0.03,
                "c" => 0.03,
                "p" => 0.03,
                "u" => 0.03,
                "y" => 0.03,
                "i" => 0.06,
                "e" => 0.14,
                "by" => 1.0,
                "he" => 1.0,
                "nc" => 0.1,
                "nd" => 0.1,
                "ng" => 0.1,
                "no" => 0.1,
                "ns" => 0.1,
                "od" => 0.1,
                "of" => 0.1,
                "os" => 0.1,
                "si" => 0.1,
                "ta" => 1.0 / 13.0,
                "to" => 1.0 / 13.0,
                "ed" => 1.0 / 14.0,
                "em" => 1.0 / 14.0,
                "ey" => 1.0 / 14.0,
                "fo" => 0.5,
                "wa" => 0.5,
                "wo" => 0.5,
                "al" => 1.0 / 3.0,
                "ar" => 1.0 / 3.0,
                "ay" => 1.0 / 3.0,
                "ce" => 1.0 / 3.0,
                "co" => 1.0 / 3.0,
                "ct" => 1.0 / 3.0,
                "po" => 1.0 / 3.0,
                "pr" => 1.0 / 3.0,
                "pu" => 1.0 / 3.0,
                "uc" => 1.0 / 3.0,
                "ur" => 1.0 / 3.0,
                "us" => 1.0 / 3.0,
                "de" => 0.2,
                "do" => 0.2,
                "ds" => 0.2,
                "du" => 0.2,
                "nt" => 0.2,
                "on" => 0.2,
                "or" => 0.2,
                "ot" => 0.2,
                "rd" => 0.2,
                "re" => 0.2,
                "ro" => 0.2,
                "rp" => 0.2,
                "st" => 0.2,
                "io" => 1.0 / 6.0,
                "is" => 1.0 / 6.0,
                "ti" => 2.0 / 13.0,
                "in" => 2.0 / 3.0,
                "se" => 0.4,
                "es" => 2.0 / 7.0,
                "te" => 3.0 / 13.0,
                "en" => 3.0 / 14.0,
                "th" => 4.0 / 13.0,
                "are" => 1.0,
                "ces" => 1.0,
                "con" => 1.0,
                "cti" => 1.0,
                "ded" => 1.0,
                "duc" => 1.0,
                "for" => 1.0,
                "ion" => 1.0,
                "ist" => 1.0,
                "nce" => 1.0,
                "nde" => 1.0,
                "not" => 1.0,
                "nsi" => 1.0,
                "nte" => 1.0,
                "odu" => 1.0,
                "ose" => 1.0,
                "pos" => 1.0,
                "pro" => 1.0,
                "pur" => 1.0,
                "rds" => 1.0,
                "rod" => 1.0,
                "rpo" => 1.0,
                "sis" => 1.0,
                "tal" => 1.0,
                "the" => 1.0,
                "tot" => 1.0,
                "uct" => 1.0,
                "urp" => 1.0,
                "use" => 1.0,
                "way" => 1.0,
                "wor" => 1.0,
                "ons" => 0.5,
                "ord" => 0.5,
                "ota" => 0.5,
                "sti" => 0.5,
                "tin" => 0.5,
                "tio" => 0.5,
                "enc" => 1.0 / 3.0,
                "end" => 1.0 / 3.0,
                "ent" => 1.0 / 3.0,
                "tes" => 1.0 / 3.0,
                "ese" => 0.25,
                "est" => 0.25,
                "hem" => 0.25,
                "hes" => 0.25,
                "hey" => 0.25,
                "ing" => 0.25,
                "int" => 0.25,
                "sen" => 0.25,
                "ses" => 0.25,
                "ten" => 2.0 / 3.0,
            )
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.ln()))
            .collect()
        }

        #[fixture]
        fn high_accuracy_model(low_accuracy_model: HashMap<String, f64>) -> HashMap<String, f64> {
            let mut quadrigrams_and_fivegrams: HashMap<String, f64> = hashmap!(
                "cons" => 1.0_f64,
                "ctio" => 1.0,
                "duct" => 1.0,
                "ence" => 1.0,
                "ende" => 1.0,
                "ente" => 1.0,
                "esti" => 1.0,
                "hese" => 1.0,
                "inte" => 1.0,
                "nces" => 1.0,
                "nded" => 1.0,
                "nsis" => 1.0,
                "nten" => 1.0,
                "oduc" => 1.0,
                "onsi" => 1.0,
                "ords" => 1.0,
                "oses" => 1.0,
                "otal" => 1.0,
                "pose" => 1.0,
                "prod" => 1.0,
                "purp" => 1.0,
                "rodu" => 1.0,
                "rpos" => 1.0,
                "sent" => 1.0,
                "sist" => 1.0,
                "stin" => 1.0,
                "test" => 1.0,
                "ting" => 1.0,
                "tion" => 1.0,
                "tota" => 1.0,
                "ucti" => 1.0,
                "urpo" => 1.0,
                "word" => 1.0,
                "tenc" => 0.5,
                "tend" => 0.5,
                "them" => 0.25,
                "thes" => 0.25,
                "they" => 0.25,
                "consi" => 1.0,
                "ction" => 1.0,
                "ducti" => 1.0,
                "ences" => 1.0,
                "ended" => 1.0,
                "enten" => 1.0,
                "estin" => 1.0,
                "inten" => 1.0,
                "nsist" => 1.0,
                "oduct" => 1.0,
                "onsis" => 1.0,
                "poses" => 1.0,
                "produ" => 1.0,
                "purpo" => 1.0,
                "roduc" => 1.0,
                "rpose" => 1.0,
                "sente" => 1.0,
                "sting" => 1.0,
                "tence" => 1.0,
                "tende" => 1.0,
                "testi" => 1.0,
                "these" => 1.0,
                "total" => 1.0,
                "uctio" => 1.0,
                "urpos" => 1.0,
                "words" => 1.0,
                "ntenc" => 0.5,
                "ntend" => 0.5,
            )
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.ln()))
            .collect();
            quadrigrams_and_fivegrams.extend(low_accuracy_model.into_iter());
            quadrigrams_and_fivegrams
        }

        #[rstest]
        fn test_language_model_files_writer(
            text: &'static str,
            high_accuracy_model: HashMap<String, f64>,
        ) {
            let input_file = create_temp_input_file(text);
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result = LanguageModelFilesWriter::create_and_write_language_model_files(
                input_file.path(),
                output_directory.path(),
                Language::English,
                "\\p{L}",
            );
            assert!(result.is_ok());

            let files = read_directory_content(output_directory.path());
            assert_eq!(files.len(), 1);

            check_fst_map_file(
                &files[0],
                NGRAM_PROBABILITY_MODEL_FILE_NAME,
                high_accuracy_model,
            );
        }
    }

    mod test_data_files {
        use super::*;
        use indoc::indoc;

        const TEXT: &str = indoc! {r#"
            There are many attributes associated with good software.
            Some of these can be mutually contradictory, and different customers and participants may have different priorities.
            Weinberg provides an example of how different goals can have a dramatic effect on both effort required and efficiency.
            Furthermore, he notes that programmers will generally aim to achieve any explicit goals which may be set, probably at the expense of any other quality attributes.
            Sommerville has identified four generalised attributes which are not concerned with what a program does, but how well the program does it:
            Maintainability, Dependability, Efficiency, Usability
        "#};

        const EXPECTED_SINGLE_WORDS_FILE_CONTENT: &str = indoc! {r#"
            there
            attributes
            associated
            software
        "#};

        const EXPECTED_WORD_PAIRS_FILE_CONTENT: &str = indoc! {r#"
            there attributes
            associated software
            these mutually
            contradictory different
        "#};

        #[test]
        fn test_test_data_files_writer() {
            let input_file = create_temp_input_file(TEXT);
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result = TestDataFilesWriter::create_and_write_test_data_files(
                input_file.path(),
                output_directory.path(),
                "\\p{L}",
                4,
            );
            assert!(result.is_ok());

            let test_data_files = read_directory_content(output_directory.path());
            assert_eq!(test_data_files.len(), 3);

            check_test_data_sentences_file(&test_data_files[0], "sentences.txt");
            check_test_data_file(
                &test_data_files[1],
                "single-words.txt",
                EXPECTED_SINGLE_WORDS_FILE_CONTENT,
            );
            check_test_data_file(
                &test_data_files[2],
                "word-pairs.txt",
                EXPECTED_WORD_PAIRS_FILE_CONTENT,
            );
        }

        fn check_test_data_sentences_file(file_path: &Path, expected_file_name: &str) {
            check_file_name(file_path, expected_file_name);
            let sentences_file_content = fs::read_to_string(file_path).unwrap();
            assert_eq!(sentences_file_content.lines().count(), 4);
            // Sentences are chosen randomly, so we just check
            // if the chosen sentences are part of the original text
            let text_lines = TEXT.lines().collect_vec();
            for line in sentences_file_content.lines() {
                assert!(text_lines.contains(&line));
            }
        }

        fn check_test_data_file(
            file_path: &Path,
            expected_file_name: &str,
            expected_file_content: &str,
        ) {
            check_file_name(file_path, expected_file_name);
            let test_data_file_content = fs::read_to_string(file_path).unwrap();
            assert_eq!(test_data_file_content, expected_file_content);
        }

        fn check_file_name(file_path: &Path, expected_file_name: &str) {
            assert!(file_path.is_file());
            let file_name = file_path.file_name().unwrap();
            assert_eq!(file_name, expected_file_name);
        }
    }

    mod unique_ngrams_writer {
        use super::*;
        use crate::Language::{English, German, Spanish};
        use rstest::{fixture, rstest};

        #[fixture]
        fn expected_unique_ngrams() -> Vec<NgramCountModel> {
            vec![
                NgramCountModel {
                    language: English,
                    ngrams: count_model_fst(hashset!("th")),
                },
                NgramCountModel {
                    language: German,
                    ngrams: count_model_fst(hashset!("rz", "äu")),
                },
            ]
        }

        #[test]
        #[cfg_attr(target_os = "windows", ignore)]
        #[should_panic(expected = "Output directory path 'some/relative/path' is not absolute")]
        fn test_unique_ngrams_writer_with_relative_output_directory_path() {
            let relative_output_directory_path = PathBuf::from("some/relative/path");
            let _ = UniqueNgramsWriter::create_and_write_unique_ngram_files(
                relative_output_directory_path.as_path(),
            );
        }

        #[test]
        #[cfg_attr(target_os = "windows", ignore)]
        #[should_panic(expected = "Output directory path '/some/absolute/path' does not exist")]
        fn test_unique_ngrams_writer_with_non_existing_output_directory_path() {
            let relative_output_directory_path = PathBuf::from("/some/absolute/path");
            let _ = UniqueNgramsWriter::create_and_write_unique_ngram_files(
                relative_output_directory_path.as_path(),
            );
        }

        #[rstest]
        fn test_store_unique_ngrams(expected_unique_ngrams: Vec<NgramCountModel>) {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result = UniqueNgramsWriter::store_unique_ngrams(
                expected_unique_ngrams,
                output_directory.path(),
            );
            assert!(result.is_ok());

            let english_dir_path = output_directory
                .path()
                .join(English.iso_code_639_1().to_string());
            assert!(english_dir_path.exists());

            let english_unique_ngram_files = read_directory_content(&english_dir_path);
            assert_eq!(english_unique_ngram_files.len(), 1);
            check_fst_set_file(
                &english_unique_ngram_files[0],
                "unique-ngrams.fst",
                hashset!("th".to_string()),
            );

            let german_dir_path = output_directory
                .path()
                .join(German.iso_code_639_1().to_string());
            assert!(german_dir_path.exists());

            let german_unique_ngram_files = read_directory_content(&german_dir_path);
            assert_eq!(german_unique_ngram_files.len(), 1);
            check_fst_set_file(
                &german_unique_ngram_files[0],
                "unique-ngrams.fst",
                hashset!("rz".to_string(), "äu".to_string()),
            );
        }

        #[rstest]
        fn test_identify_unique_ngrams(expected_unique_ngrams: Vec<NgramCountModel>) {
            let ngrams = hashmap!(
                English => bytes_set(hashset!("th", "en", "es")),
                German => bytes_set(hashset!("äu", "en", "rz")),
                Spanish => bytes_set(hashset!("es", "en"))
            );
            let actual_unique_ngrams = UniqueNgramsWriter::identify_unique_ngrams(ngrams);
            assert_eq!(actual_unique_ngrams, expected_unique_ngrams);
        }
    }

    mod most_common_ngrams_writer {
        use super::*;
        use crate::Language::{English, German};
        use rstest::{fixture, rstest};

        #[fixture]
        fn most_common_german_ngrams() -> NgramCountModel {
            NgramCountModel {
                language: German,
                ngrams: count_model_fst(hashset!(
                    "ch", "chen", "de", "der", "die", "diese", "e", "ei", "ein", "eine", "en",
                    "er", "i", "ich", "icht", "ische", "lich", "n", "nicht", "r", "s", "sch",
                    "sche", "schen", "ungen"
                )),
            }
        }

        #[test]
        fn test_most_common_ngrams_writer() {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result = MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                output_directory.path(),
                &hashset!(English, German),
                5,
            );
            assert!(result.is_ok());

            let subdirectories = read_directory_content(output_directory.path());
            assert_eq!(subdirectories.len(), 2);

            let german_dir_name = German.iso_code_639_1().to_string();
            let german_dir_path = output_directory.path().join(german_dir_name);

            let english_dir_name = English.iso_code_639_1().to_string();
            let english_dir_path = output_directory.path().join(english_dir_name);

            assert_eq!(subdirectories[0].as_path(), german_dir_path);
            assert_eq!(subdirectories[1].as_path(), english_dir_path);

            assert!(german_dir_path.is_dir());
            assert!(english_dir_path.is_dir());

            let german_most_common_ngram_files = read_directory_content(&german_dir_path);
            assert_eq!(german_most_common_ngram_files.len(), 1);

            check_fst_set_file(
                &german_most_common_ngram_files[0],
                "mostcommon-ngrams.fst",
                to_string(hashset!(
                    "e", "i", "n", "r", "s", "ch", "de", "ei", "en", "er", "der", "die", "ein",
                    "ich", "sch", "chen", "eine", "icht", "lich", "sche", "diese", "ische",
                    "nicht", "schen", "ungen"
                )),
            );

            let english_most_common_ngram_files = read_directory_content(&english_dir_path);
            assert_eq!(english_most_common_ngram_files.len(), 1);

            check_fst_set_file(
                &english_most_common_ngram_files[0],
                "mostcommon-ngrams.fst",
                to_string(hashset!(
                    "a", "e", "i", "o", "t", "an", "he", "in", "re", "th", "and", "ing", "ion",
                    "the", "tio", "atio", "ment", "that", "tion", "with", "ation", "canad",
                    "ction", "ement", "tions"
                )),
            );
        }

        #[test]
        #[cfg_attr(target_os = "windows", ignore)]
        #[should_panic(expected = "Output directory path 'some/relative/path' is not absolute")]
        fn test_most_common_ngrams_writer_with_relative_output_directory_path() {
            let relative_output_directory_path = PathBuf::from("some/relative/path");
            let _ = MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                relative_output_directory_path.as_path(),
                &hashset!(English, German),
                5,
            );
        }

        #[test]
        #[cfg_attr(target_os = "windows", ignore)]
        #[should_panic(expected = "Output directory path '/some/absolute/path' does not exist")]
        fn test_most_common_ngrams_writer_with_non_existing_output_directory_path() {
            let relative_output_directory_path = PathBuf::from("/some/absolute/path");
            let _ = MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                relative_output_directory_path.as_path(),
                &hashset!(English, German),
                5,
            );
        }

        #[test]
        #[should_panic(expected = "Set of languages must not be empty")]
        fn test_most_common_ngrams_writer_without_languages() {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let _ = MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                output_directory.path(),
                &hashset!(),
                5,
            );
        }

        #[test]
        #[should_panic(expected = "Amount of most common ngrams must be greater than zero")]
        fn test_most_common_ngrams_writer_without_most_common() {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let _ = MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                output_directory.path(),
                &hashset!(English, German),
                0,
            );
        }

        #[rstest]
        fn test_identify_most_common_ngrams(most_common_german_ngrams: NgramCountModel) {
            let actual_most_common_ngrams =
                MostCommonNgramsWriter::identify_most_common_ngrams(German, 5);
            assert_eq!(actual_most_common_ngrams, most_common_german_ngrams);
        }
    }
}
