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
use std::fs::{create_dir, remove_file, File};
use std::io;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;

use crate::constant::{DIGITS_AT_BEGINNING, MULTIPLE_WHITESPACE, NUMBERS, PUNCTUATION};
use crate::detector::split_text_into_words;
use crate::file::read_test_data_file;
use crate::model::{
    get_utf8_slice, load_ngram_probability_model, NgramCountModel, NgramModelType,
    TrainingDataLanguageModel,
};
use crate::ngram::Ngram;
use crate::Language;
use brotli::CompressorWriter;
use counter::Counter;
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

        let input_lines_count = BufReader::new(File::open(input_file_path)?).lines().count();
        let input_lines = BufReader::new(File::open(input_file_path)?)
            .lines()
            .map(|line| line.unwrap());

        let sentences_file = File::create(sentences_file_path)?;
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

    fn identify_unique_ngrams(ngrams: HashMap<Language, HashSet<String>>) -> Vec<NgramCountModel> {
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
            .map(|(language, ngrams)| NgramCountModel { language, ngrams })
            .sorted_by_key(|model| model.language)
            .collect()
    }

    fn store_unique_ngrams(
        unique_ngrams: Vec<NgramCountModel>,
        ngram_length: usize,
        output_directory_path: &Path,
    ) -> io::Result<()> {
        store_ngram_count_models(
            unique_ngrams,
            ngram_length,
            output_directory_path,
            NgramModelType::Unique,
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
        for ngram_length in 1..6 {
            let mut most_common_ngrams = vec![];
            for language in languages.iter() {
                let ngrams = Self::identify_most_common_ngrams(
                    *language,
                    ngram_length,
                    most_common as usize,
                );
                most_common_ngrams.push(ngrams);
            }
            Self::store_most_common_ngrams(
                most_common_ngrams,
                ngram_length,
                output_directory_path,
            )?;
        }
        Ok(())
    }

    fn identify_most_common_ngrams(
        language: Language,
        ngram_length: usize,
        most_common: usize,
    ) -> NgramCountModel {
        let sentences = read_test_data_file(language, "sentences.txt").unwrap();
        let words = split_text_into_words(sentences);
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

        let most_common_ngrams = counter
            .k_most_common_ordered(most_common)
            .iter()
            .map(|(ngram, _)| ngram.to_string())
            .collect();

        NgramCountModel {
            language,
            ngrams: most_common_ngrams,
        }
    }

    fn store_most_common_ngrams(
        most_common_ngrams: Vec<NgramCountModel>,
        ngram_length: usize,
        output_directory_path: &Path,
    ) -> io::Result<()> {
        store_ngram_count_models(
            most_common_ngrams,
            ngram_length,
            output_directory_path,
            NgramModelType::MostCommon,
        )
    }
}

fn store_ngram_count_models(
    ngram_count_models: Vec<NgramCountModel>,
    ngram_length: usize,
    output_directory_path: &Path,
    model_type: NgramModelType,
) -> io::Result<()> {
    let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
    let file_name = format!("{model_type}_{ngram_name}s.json.br");
    for model in ngram_count_models {
        let language_dir_path =
            output_directory_path.join(model.language.iso_code_639_1().to_string());
        if !language_dir_path.exists() {
            create_dir(language_dir_path.as_path())?;
        }
        let file_path = language_dir_path.join(&file_name);
        let file = File::create(file_path)?;
        let mut compressed_file = CompressorWriter::new(file, 4096, 11, 22);
        let json_str = serde_json::to_string(&model)?;
        compressed_file.write_all(json_str.as_bytes())?;
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

    fn check_brotli_file(file_path: &Path, expected_file_name: &str, expected_file_content: &str) {
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

    fn to_string(ngrams: HashSet<&str>) -> HashSet<String> {
        ngrams.into_iter().map(|ngram| ngram.to_string()).collect()
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
                Language::English,
                "\\p{L}",
            );
            assert!(result.is_ok());

            let files = read_directory_content(output_directory.path());
            assert_eq!(files.len(), 5);

            check_brotli_file(&files[0], "bigrams.json.br", EXPECTED_BIGRAM_MODEL);
            check_brotli_file(&files[1], "fivegrams.json.br", EXPECTED_FIVEGRAM_MODEL);
            check_brotli_file(&files[2], "quadrigrams.json.br", EXPECTED_QUADRIGRAM_MODEL);
            check_brotli_file(&files[3], "trigrams.json.br", EXPECTED_TRIGRAM_MODEL);
            check_brotli_file(&files[4], "unigrams.json.br", EXPECTED_UNIGRAM_MODEL);
        }
    }

    mod test_data_files {
        use super::*;
        use indoc::indoc;
        use std::fs::read_to_string;

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
            let sentences_file_content = read_to_string(file_path).unwrap();
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
            let test_data_file_content = read_to_string(file_path).unwrap();
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
        fn unique_ngrams() -> Vec<NgramCountModel> {
            vec![
                NgramCountModel {
                    language: English,
                    ngrams: to_string(hashset!("th")),
                },
                NgramCountModel {
                    language: German,
                    ngrams: to_string(hashset!("rz", "äu")),
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
        fn test_store_unique_ngrams(unique_ngrams: Vec<NgramCountModel>) {
            let output_directory = tempdir().expect("Temporary directory could not be created");
            let result =
                UniqueNgramsWriter::store_unique_ngrams(unique_ngrams, 2, output_directory.path());
            assert!(result.is_ok());

            let english_dir_path = output_directory
                .path()
                .join(English.iso_code_639_1().to_string());
            assert!(english_dir_path.exists());

            let english_unique_ngram_files = read_directory_content(&english_dir_path);
            assert_eq!(english_unique_ngram_files.len(), 1);
            check_brotli_file(
                &english_unique_ngram_files[0],
                "unique_bigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["th"]}"#,
            );

            let german_dir_path = output_directory
                .path()
                .join(German.iso_code_639_1().to_string());
            assert!(german_dir_path.exists());

            let german_unique_ngram_files = read_directory_content(&german_dir_path);
            assert_eq!(german_unique_ngram_files.len(), 1);
            check_brotli_file(
                &german_unique_ngram_files[0],
                "unique_bigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["rz","äu"]}"#,
            );
        }

        #[rstest]
        fn test_identify_unique_ngrams(unique_ngrams: Vec<NgramCountModel>) {
            let ngrams = hashmap!(
                English => to_string(hashset!("th", "en", "es")),
                German => to_string(hashset!("äu", "en", "rz")),
                Spanish => to_string(hashset!("es", "en"))
            );
            let actual_unique_ngrams = UniqueNgramsWriter::identify_unique_ngrams(ngrams);
            assert_eq!(actual_unique_ngrams, unique_ngrams);
        }
    }

    mod most_common_ngrams_writer {
        use super::*;
        use crate::Language::{English, German};
        use rstest::{fixture, rstest};

        #[fixture]
        fn most_common_german_trigrams() -> NgramCountModel {
            NgramCountModel {
                language: German,
                ngrams: to_string(hashset!("der", "die", "ein", "ich", "sch")),
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
            assert_eq!(german_most_common_ngram_files.len(), 5);

            check_brotli_file(
                &german_most_common_ngram_files[0],
                "mostcommon_bigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["ch","de","ei","en","er"]}"#,
            );
            check_brotli_file(
                &german_most_common_ngram_files[1],
                "mostcommon_fivegrams.json.br",
                r#"{"language":"GERMAN","ngrams":["diese","ische","nicht","schen","ungen"]}"#,
            );
            check_brotli_file(
                &german_most_common_ngram_files[2],
                "mostcommon_quadrigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["chen","eine","icht","lich","sche"]}"#,
            );
            check_brotli_file(
                &german_most_common_ngram_files[3],
                "mostcommon_trigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["der","die","ein","ich","sch"]}"#,
            );
            check_brotli_file(
                &german_most_common_ngram_files[4],
                "mostcommon_unigrams.json.br",
                r#"{"language":"GERMAN","ngrams":["e","i","n","r","s"]}"#,
            );

            let english_most_common_ngram_files = read_directory_content(&english_dir_path);
            assert_eq!(english_most_common_ngram_files.len(), 5);

            check_brotli_file(
                &english_most_common_ngram_files[0],
                "mostcommon_bigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["an","he","in","re","th"]}"#,
            );
            check_brotli_file(
                &english_most_common_ngram_files[1],
                "mostcommon_fivegrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["ation","canad","ction","ement","tions"]}"#,
            );
            check_brotli_file(
                &english_most_common_ngram_files[2],
                "mostcommon_quadrigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["atio","ment","that","tion","with"]}"#,
            );
            check_brotli_file(
                &english_most_common_ngram_files[3],
                "mostcommon_trigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["and","ing","ion","the","tio"]}"#,
            );
            check_brotli_file(
                &english_most_common_ngram_files[4],
                "mostcommon_unigrams.json.br",
                r#"{"language":"ENGLISH","ngrams":["a","e","i","o","t"]}"#,
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
        fn test_identify_most_common_ngrams(most_common_german_trigrams: NgramCountModel) {
            let actual_most_common_trigrams =
                MostCommonNgramsWriter::identify_most_common_ngrams(German, 3, 5);
            assert_eq!(actual_most_common_trigrams, most_common_german_trigrams);
        }
    }
}
