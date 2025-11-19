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
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::{fs, io};

use crate::constant::{MULTIPLE_WHITESPACE, NUMBERS, PUNCTUATION};
use crate::model::{load_ngram_probability_model, TrainingDataLanguageModel};
use crate::ngram::Ngram;
use crate::Language;
use brotli::CompressorWriter;
use itertools::Itertools;
use regex::Regex;
use serde_json::json;
use strum::IntoEnumIterator;

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
        language: &Language,
        char_class: &str,
    ) -> io::Result<()> {
        check_input_file_path(input_file_path);
        check_output_directory_path(output_directory_path);

        let unigram_model =
            Self::create_language_model(input_file_path, language, 1, char_class, &hashmap!())?;

        let bigram_model = Self::create_language_model(
            input_file_path,
            language,
            2,
            char_class,
            &unigram_model.absolute_frequencies,
        )?;

        let trigram_model = Self::create_language_model(
            input_file_path,
            language,
            3,
            char_class,
            &bigram_model.absolute_frequencies,
        )?;

        let quadrigram_model = Self::create_language_model(
            input_file_path,
            language,
            4,
            char_class,
            &trigram_model.absolute_frequencies,
        )?;

        let fivegram_model = Self::create_language_model(
            input_file_path,
            language,
            5,
            char_class,
            &quadrigram_model.absolute_frequencies,
        )?;

        Self::write_compressed_language_model(
            &unigram_model,
            output_directory_path,
            "unigrams.json",
        )?;
        Self::write_compressed_language_model(
            &bigram_model,
            output_directory_path,
            "bigrams.json",
        )?;
        Self::write_compressed_language_model(
            &trigram_model,
            output_directory_path,
            "trigrams.json",
        )?;
        Self::write_compressed_language_model(
            &quadrigram_model,
            output_directory_path,
            "quadrigrams.json",
        )?;
        Self::write_compressed_language_model(
            &fivegram_model,
            output_directory_path,
            "fivegrams.json",
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
        let file = File::open(input_file_path)?;
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

    fn write_compressed_language_model(
        model: &TrainingDataLanguageModel,
        output_directory_path: &Path,
        file_name: &str,
    ) -> io::Result<()> {
        let file_name = format!("{file_name}.br");
        let file_path = output_directory_path.join(file_name);
        let file = File::create(file_path)?;
        let mut compressed_file = CompressorWriter::new(file, 4096, 11, 22);
        compressed_file.write_all(model.to_json().as_bytes())?;
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
            remove_file(&sentences_file_path)?;
        }

        let input_file = File::open(input_file_path)?;
        let input_lines = BufReader::new(input_file).lines().map(|line| line.unwrap());

        let sentences_file = File::create(sentences_file_path)?;
        let mut sentences_writer = LineWriter::new(sentences_file);

        let mut line_counter = 0;

        for line in input_lines {
            let normalized_whitespace = MULTIPLE_WHITESPACE.replace_all(&line, " ");
            let removed_quotes = normalized_whitespace.replace('\"', "");

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
            remove_file(&single_words_file_path)?;
        }

        let input_file = File::open(input_file_path)?;
        let input_lines = BufReader::new(input_file).lines().map(|line| line.unwrap());

        let single_words_file = File::create(single_words_file_path)?;
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
            remove_file(&word_pairs_file_path)?;
        }

        for i in (0..=(words.len() - 2)).step_by(2) {
            let slice = &words[i..i + 2];
            word_pairs.push(slice.join(" "));
        }

        let word_pairs_file = File::create(word_pairs_file_path)?;
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
        for ngram_length in 1..6 {
            let ngrams = Self::load_ngrams(ngram_length);
            let unique_ngrams = Self::identify_unique_ngrams(ngrams);
            Self::store_unique_ngrams(unique_ngrams, ngram_length, output_directory_path)?;
        }
        Ok(())
    }

    fn load_ngrams(ngram_length: usize) -> HashMap<Language, HashSet<String>> {
        let mut result = HashMap::new();
        for language in Language::iter() {
            if let Some(model) = load_ngram_probability_model(language, ngram_length) {
                let ngrams = model.ngrams.keys().map(|key| key.to_string()).collect();
                result.insert(language, ngrams);
            }
        }
        result
    }

    fn identify_unique_ngrams(
        ngrams: HashMap<Language, HashSet<String>>,
    ) -> HashMap<Language, HashSet<String>> {
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
    }

    fn store_unique_ngrams(
        unique_ngrams: HashMap<Language, HashSet<String>>,
        ngram_length: usize,
        output_directory_path: &Path,
    ) -> io::Result<()> {
        let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
        let file_name = format!("unique_{ngram_name}s.json.br");
        for (language, ngrams) in unique_ngrams {
            let language_dir_path =
                output_directory_path.join(language.iso_code_639_1().to_string());
            if !language_dir_path.exists() {
                fs::create_dir(language_dir_path.as_path())?;
            }
            let file_path = language_dir_path.join(&file_name);
            let file = File::create(file_path)?;
            let mut compressed_file = CompressorWriter::new(file, 4096, 11, 22);
            let sorted_ngrams = ngrams.into_iter().sorted().collect_vec();
            let obj = json!({"language": language, "ngrams": sorted_ngrams});
            compressed_file.write_all(obj.to_string().as_bytes())?;
        }
        Ok(())
    }
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
    use crate::minify;
    use brotli::Decompressor;
    use std::fs::read_dir;
    use std::io::Read;
    use std::path::PathBuf;
    use tempfile::{tempdir, NamedTempFile};

    fn create_temp_input_file(content: &str) -> NamedTempFile {
        let mut input_file = NamedTempFile::new().unwrap();
        input_file
            .write_all(content.as_bytes())
            .expect("Text could not be written to temporary input file");
        input_file
    }

    fn read_directory_content(directory: &Path) -> Vec<PathBuf> {
        let mut files = read_dir(directory)
            .unwrap()
            .map(|it| it.unwrap().path())
            .collect_vec();

        files.sort();
        files
    }

    fn assert_file_content(
        file_path: &Path,
        expected_file_name: &str,
        expected_file_content: &str,
    ) {
        assert!(file_path.is_file());

        let file_name = file_path.file_name().unwrap();
        assert_eq!(file_name, expected_file_name);

        let compressed_file = File::open(file_path).unwrap();
        let mut uncompressed_file = Decompressor::new(compressed_file, 4096);
        let mut uncompressed_file_content = String::new();
        uncompressed_file
            .read_to_string(&mut uncompressed_file_content)
            .unwrap();

        assert_eq!(uncompressed_file_content, minify(expected_file_content));
    }

    mod language_model_files {
        use super::*;

        const TEXT: &str = "
            These sentences are intended for testing purposes.
            Do not use them in production!
            By the way, they consist of 23 words in total.
        ";

        const EXPECTED_UNIGRAM_MODEL: &str = r#"
        {
            "language":"ENGLISH",
            "ngrams":{
                "1/10":"n o s",
                "1/100":"b g l m",
                "1/20":"d r",
                "1/25":"h",
                "1/50":"f w",
                "13/100":"t",
                "3/100":"a c p u y",
                "3/50":"i",
                "7/50":"e"
            }
        }
        "#;

        const EXPECTED_BIGRAM_MODEL: &str = r#"
        {
            "language":"ENGLISH",
            "ngrams":{
                "1/1":"by he",
                "1/10":"nc nd ng no ns od of os si",
                "1/13":"ta to",
                "1/14":"ed em ey",
                "1/2":"fo wa wo",
                "1/3":"al ar ay ce co ct po pr pu uc ur us",
                "1/5":"de do ds du nt on or ot rd re ro rp st",
                "1/6":"io is",
                "2/13":"ti",
                "2/3":"in",
                "2/5":"se",
                "2/7":"es",
                "3/13":"te",
                "3/14":"en",
                "4/13":"th"
            }
        }
        "#;

        const EXPECTED_TRIGRAM_MODEL: &str = r#"
        {
            "language":"ENGLISH",
            "ngrams":{
                "1/1":"are ces con cti ded duc for ion ist nce nde not nsi nte odu ose pos pro pur rds rod rpo sis tal the tot uct urp use way wor",
                "1/2":"ons ord ota sti tin tio",
                "1/3":"enc end ent tes",
                "1/4":"ese est hem hes hey ing int sen ses",
                "2/3":"ten"
            }
        }
        "#;

        const EXPECTED_QUADRIGRAM_MODEL: &str = r#"
        {
            "language":"ENGLISH",
            "ngrams":{
                "1/1":"cons ctio duct ence ende ente esti hese inte nces nded nsis nten oduc onsi ords oses otal pose prod purp rodu rpos sent sist stin test ting tion tota ucti urpo word",
                "1/2":"tenc tend",
                "1/4":"them thes they"
            }
        }
        "#;

        const EXPECTED_FIVEGRAM_MODEL: &str = r#"
        {
            "language":"ENGLISH",
            "ngrams":{
                "1/1":"consi ction ducti ences ended enten estin inten nsist oduct onsis poses produ purpo roduc rpose sente sting tence tende testi these total uctio urpos words",
                "1/2":"ntenc ntend"
            }
        }
        "#;

        #[test]
        fn test_language_model_files_writer() {
            let input_file = create_temp_input_file(TEXT);
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result = LanguageModelFilesWriter::create_and_write_language_model_files(
                input_file.path(),
                output_directory.path(),
                &Language::English,
                "\\p{L}",
            );

            assert!(result.is_ok());

            let files = read_directory_content(output_directory.path());

            assert_eq!(files.len(), 5);

            let unigrams_file_path = files.get(4).unwrap();
            let bigrams_file_path = files.get(0).unwrap();
            let trigrams_file_path = files.get(3).unwrap();
            let quadrigrams_file_path = files.get(2).unwrap();
            let fivegrams_file_path = files.get(1).unwrap();

            assert_file_content(
                unigrams_file_path,
                "unigrams.json.br",
                EXPECTED_UNIGRAM_MODEL,
            );

            assert_file_content(bigrams_file_path, "bigrams.json.br", EXPECTED_BIGRAM_MODEL);

            assert_file_content(
                trigrams_file_path,
                "trigrams.json.br",
                EXPECTED_TRIGRAM_MODEL,
            );

            assert_file_content(
                quadrigrams_file_path,
                "quadrigrams.json.br",
                EXPECTED_QUADRIGRAM_MODEL,
            );

            assert_file_content(
                fivegrams_file_path,
                "fivegrams.json.br",
                EXPECTED_FIVEGRAM_MODEL,
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

        const EXPECTED_SENTENCES_FILE_CONTENT: &str = indoc! {r#"
            There are many attributes associated with good software.
            Some of these can be mutually contradictory, and different customers and participants may have different priorities.
            Weinberg provides an example of how different goals can have a dramatic effect on both effort required and efficiency.
            Furthermore, he notes that programmers will generally aim to achieve any explicit goals which may be set, probably at the expense of any other quality attributes.
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

            assert_file_content(
                &test_data_files[0],
                "sentences.txt",
                EXPECTED_SENTENCES_FILE_CONTENT,
            );

            assert_file_content(
                &test_data_files[1],
                "single-words.txt",
                EXPECTED_SINGLE_WORDS_FILE_CONTENT,
            );

            assert_file_content(
                &test_data_files[2],
                "word-pairs.txt",
                EXPECTED_WORD_PAIRS_FILE_CONTENT,
            );
        }

        fn assert_file_content(
            file_path: &Path,
            expected_file_name: &str,
            expected_file_content: &str,
        ) {
            assert!(file_path.is_file());

            let file_name = file_path.file_name().unwrap();
            assert_eq!(file_name, expected_file_name);

            let mut test_data_file = File::open(file_path).unwrap();
            let mut test_data_file_content = String::new();
            test_data_file
                .read_to_string(&mut test_data_file_content)
                .unwrap();
            assert_eq!(test_data_file_content, expected_file_content);
        }
    }

    mod unique_ngrams_writer {
        use super::*;
        use rstest::{fixture, rstest};

        #[fixture]
        fn unique_ngrams() -> HashMap<Language, HashSet<String>> {
            hashmap!(
                Language::English => to_string(hashset!("th")),
                Language::German => to_string(hashset!("rz", "äu"))
            )
        }

        #[rstest]
        fn test_store_unique_ngrams(unique_ngrams: HashMap<Language, HashSet<String>>) {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result =
                UniqueNgramsWriter::store_unique_ngrams(unique_ngrams, 2, output_directory.path());
            assert!(result.is_ok());

            let english_dir_path = output_directory
                .path()
                .join(Language::English.iso_code_639_1().to_string());
            assert!(english_dir_path.exists());

            let english_unique_ngram_files = read_directory_content(&english_dir_path);
            assert_eq!(english_unique_ngram_files.len(), 1);
            assert_file_content(
                &english_unique_ngram_files[0],
                "unique_bigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["th"]}"#,
            );

            let german_dir_path = output_directory
                .path()
                .join(Language::German.iso_code_639_1().to_string());
            assert!(german_dir_path.exists());

            let german_unique_ngram_files = read_directory_content(&german_dir_path);
            assert_eq!(german_unique_ngram_files.len(), 1);
            assert_file_content(
                &german_unique_ngram_files[0],
                "unique_bigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["rz","äu"]}"#,
            );
        }

        #[rstest]
        fn test_identify_unique_ngrams(unique_ngrams: HashMap<Language, HashSet<String>>) {
            let ngrams = hashmap!(
                Language::English => to_string(hashset!("th", "en", "es")),
                Language::German => to_string(hashset!("äu", "en", "rz")),
                Language::Spanish => to_string(hashset!("es", "en"))
            );
            let actual_unique_ngrams = UniqueNgramsWriter::identify_unique_ngrams(ngrams);
            assert_eq!(actual_unique_ngrams, unique_ngrams);
        }

        fn to_string(ngrams: HashSet<&str>) -> HashSet<String> {
            ngrams.into_iter().map(|ngram| ngram.to_string()).collect()
        }
    }
}
