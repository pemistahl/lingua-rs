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

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::panic;
use std::path::PathBuf;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};

use crate::builder::{
    LanguageDetectorBuilder, MINIMUM_RELATIVE_DISTANCE_MESSAGE, MISSING_LANGUAGE_MESSAGE,
};
use crate::detector::LanguageDetector;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use crate::language::Language;
use crate::result::DetectionResult;
use crate::writer::{LanguageModelFilesWriter, TestDataFilesWriter};

#[pymodule]
fn lingua(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<ConfidenceValue>()?;
    m.add_class::<DetectionResult>()?;
    m.add_class::<IsoCode639_1>()?;
    m.add_class::<IsoCode639_3>()?;
    m.add_class::<Language>()?;
    m.add_class::<LanguageDetectorBuilder>()?;
    m.add_class::<LanguageDetector>()?;
    m.add_class::<LanguageModelFilesWriter>()?;
    m.add_class::<TestDataFilesWriter>()?;
    Ok(())
}

/// This class describes a language's confidence value.
///
/// Attributes:
///
///     language (Language):
///         The language associated with this confidence value.
///
///     value (float):
///         The language's confidence value which lies between 0.0 and 1.0.
#[pyclass]
struct ConfidenceValue {
    language: Language,
    value: f64,
}

#[pymethods]
impl ConfidenceValue {
    #[new]
    fn new(language: Language, value: f64) -> Self {
        Self { language, value }
    }

    /// Return the language of the associated confidence value.
    #[getter]
    fn language(&self) -> Language {
        self.language
    }

    /// Return the confidence value for the associated language.
    ///
    /// The confidence value is a value between 0.0 and 1.0.
    #[getter]
    fn value(&self) -> f64 {
        self.value
    }
}

#[pymethods]
impl DetectionResult {
    /// Return the start index of the identified single-language substring.
    #[pyo3(name = "start_index")]
    #[getter]
    fn py_start_index(&self) -> usize {
        self.start_index()
    }

    /// Return the end index of the identified single-language substring.
    #[pyo3(name = "end_index")]
    #[getter]
    fn py_end_index(&self) -> usize {
        self.end_index()
    }

    /// Return the number of words being part of the identified
    /// single-language substring.
    #[pyo3(name = "word_count")]
    #[getter]
    fn py_word_count(&self) -> usize {
        self.word_count()
    }

    /// Return the detected language of the identified single-language substring.
    #[pyo3(name = "language")]
    #[getter]
    fn py_language(&self) -> Language {
        self.language()
    }
}

#[pymethods]
impl IsoCode639_1 {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.to_string().cmp(&other.to_string()))
    }

    #[getter]
    fn name(&self) -> String {
        self.to_string().to_uppercase()
    }
}

#[pymethods]
impl IsoCode639_3 {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.to_string().cmp(&other.to_string()))
    }

    #[getter]
    fn name(&self) -> String {
        self.to_string().to_uppercase()
    }
}

#[pymethods]
impl Language {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.to_string().cmp(&other.to_string()))
    }

    /// Return a set of all supported languages.
    #[pyo3(name = "all")]
    #[classmethod]
    fn py_all(_cls: &PyType) -> HashSet<Self> {
        Self::all()
    }

    /// Return a set of all supported spoken languages.
    #[pyo3(name = "all_spoken_ones")]
    #[classmethod]
    fn py_all_spoken_ones(_cls: &PyType) -> HashSet<Self> {
        Self::all_spoken_ones()
    }

    /// Return a set of all languages supporting the Arabic script.
    #[pyo3(name = "all_with_arabic_script")]
    #[classmethod]
    fn py_all_with_arabic_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_arabic_script()
    }

    /// Return a set of all languages supporting the Cyrillic script.
    #[pyo3(name = "all_with_cyrillic_script")]
    #[classmethod]
    fn py_all_with_cyrillic_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_cyrillic_script()
    }

    /// Return a set of all languages supporting the Devanagari script.
    #[pyo3(name = "all_with_devanagari_script")]
    #[classmethod]
    fn py_all_with_devanagari_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_devanagari_script()
    }

    /// Return a set of all languages supporting the Latin script.
    #[pyo3(name = "all_with_latin_script")]
    #[classmethod]
    fn py_all_with_latin_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_latin_script()
    }

    /// Return the language associated with the ISO 639-1 code
    /// passed to this method.
    ///
    /// Raises:
    ///     ValueError: if there is no language for the given ISO code
    #[pyo3(name = "from_iso_code_639_1")]
    #[classmethod]
    fn py_from_iso_code_639_1(_cls: &PyType, iso_code: &IsoCode639_1) -> Self {
        Self::from_iso_code_639_1(iso_code)
    }

    /// Return the language associated with the ISO 639-3 code
    /// passed to this method.
    ///
    /// Raises:
    ///     ValueError: if there is no language for the given ISO code
    #[pyo3(name = "from_iso_code_639_3")]
    #[classmethod]
    fn py_from_iso_code_639_3(_cls: &PyType, iso_code: &IsoCode639_3) -> Self {
        Self::from_iso_code_639_3(iso_code)
    }

    /// Return the ISO 639-1 code of this language.
    #[pyo3(name = "iso_code_639_1")]
    #[getter]
    fn py_iso_code_639_1(&self) -> IsoCode639_1 {
        self.iso_code_639_1()
    }

    /// Return the ISO 639-3 code of this language.
    #[pyo3(name = "iso_code_639_3")]
    #[getter]
    fn py_iso_code_639_3(&self) -> IsoCode639_3 {
        self.iso_code_639_3()
    }

    #[getter]
    fn name(&self) -> String {
        self.to_string().to_uppercase()
    }
}

#[pymethods]
impl LanguageDetectorBuilder {
    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages.
    #[pyo3(name = "from_all_languages")]
    #[classmethod]
    fn py_from_all_languages(_cls: &PyType) -> Self {
        Self::from_all_languages()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in spoken languages.
    #[pyo3(name = "from_all_spoken_languages")]
    #[classmethod]
    fn py_from_all_spoken_languages(_cls: &PyType) -> Self {
        Self::from_all_spoken_languages()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages supporting the Arabic script.
    #[pyo3(name = "from_all_languages_with_arabic_script")]
    #[classmethod]
    fn py_from_all_languages_with_arabic_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_arabic_script()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages supporting the Cyrillic script.
    #[pyo3(name = "from_all_languages_with_cyrillic_script")]
    #[classmethod]
    fn py_from_all_languages_with_cyrillic_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_cyrillic_script()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages supporting the Devanagari script.
    #[pyo3(name = "from_all_languages_with_devanagari_script")]
    #[classmethod]
    fn py_from_all_languages_with_devanagari_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_devanagari_script()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages supporting the Latin script.
    #[pyo3(name = "from_all_languages_with_latin_script")]
    #[classmethod]
    fn py_from_all_languages_with_latin_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_latin_script()
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with all built-in languages except those passed to this method.
    #[pyo3(name = "from_all_languages_without", signature = (*languages))]
    #[classmethod]
    fn py_from_all_languages_without(_cls: &PyType, languages: &PyTuple) -> PyResult<Self> {
        match languages.extract::<Vec<Language>>() {
            Ok(vector) => match panic::catch_unwind(|| Self::from_all_languages_without(&vector)) {
                Ok(builder) => Ok(builder),
                Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
            },
            Err(err) => Err(err),
        }
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with the languages passed to this method.
    #[pyo3(name = "from_languages", signature = (*languages))]
    #[classmethod]
    fn py_from_languages(_cls: &PyType, languages: &PyTuple) -> PyResult<Self> {
        match languages.extract::<Vec<Language>>() {
            Ok(vector) => match panic::catch_unwind(|| Self::from_languages(&vector)) {
                Ok(builder) => Ok(builder),
                Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
            },
            Err(err) => Err(err),
        }
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with the languages specified by the ISO 639-1 codes passed
    /// to this method.
    ///
    /// Raises:
    ///     ValueError: if less than two ISO codes are specified
    #[pyo3(name = "from_iso_codes_639_1", signature = (*iso_codes))]
    #[classmethod]
    fn py_from_iso_codes_639_1(_cls: &PyType, iso_codes: &PyTuple) -> PyResult<Self> {
        match iso_codes.extract::<Vec<IsoCode639_1>>() {
            Ok(vector) => match panic::catch_unwind(|| Self::from_iso_codes_639_1(&vector)) {
                Ok(builder) => Ok(builder),
                Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
            },
            Err(err) => Err(err),
        }
    }

    /// Create and return an instance of LanguageDetectorBuilder
    /// with the languages specified by the ISO 639-3 codes passed
    /// to this method.
    ///
    /// Raises:
    ///     ValueError: if less than two ISO codes are specified
    #[pyo3(name = "from_iso_codes_639_3", signature = (*iso_codes))]
    #[classmethod]
    fn py_from_iso_codes_639_3(_cls: &PyType, iso_codes: &PyTuple) -> PyResult<Self> {
        match iso_codes.extract::<Vec<IsoCode639_3>>() {
            Ok(vector) => match panic::catch_unwind(|| Self::from_iso_codes_639_3(&vector)) {
                Ok(builder) => Ok(builder),
                Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
            },
            Err(err) => Err(err),
        }
    }

    /// Set the desired value for the minimum relative distance measure.
    ///
    /// By default, Lingua returns the most likely language for a given
    /// input text. However, there are certain words that are spelled the
    /// same in more than one language. The word 'prologue', for instance,
    /// is both a valid English and French word. Lingua would output either
    /// English or French which might be wrong in the given context.
    /// For cases like that, it is possible to specify a minimum relative
    /// distance that the logarithmized and summed up probabilities for
    /// each possible language have to satisfy.
    ///
    /// Be aware that the distance between the language probabilities is
    /// dependent on the length of the input text. The longer the input
    /// text, the larger the distance between the languages. So if you
    /// want to classify very short text phrases, do not set the minimum
    /// relative distance too high. Otherwise you will get most results
    /// returned as None which is the return value for cases where
    /// language detection is not reliably possible.
    ///
    /// Raises:
    ///     ValueError: if distance is smaller than 0.0 or greater than 0.99
    #[pyo3(name = "with_minimum_relative_distance")]
    fn py_with_minimum_relative_distance(
        mut self_: PyRefMut<Self>,
        distance: f64,
    ) -> PyResult<PyRefMut<Self>> {
        if !(0.0..=0.99).contains(&distance) {
            Err(PyValueError::new_err(MINIMUM_RELATIVE_DISTANCE_MESSAGE))
        } else {
            self_.with_minimum_relative_distance(distance);
            Ok(self_)
        }
    }

    /// Preload all language models when creating the LanguageDetector
    /// instance.
    ///
    /// By default, Lingua uses lazy-loading to load only those language
    /// models on demand which are considered relevant by the rule-based
    /// filter engine. For web services, for instance, it is rather
    /// beneficial to preload all language models into memory to avoid
    /// unexpected latency while waiting for the service response. This
    /// method allows to switch between these two loading modes.
    #[pyo3(name = "with_preloaded_language_models")]
    fn py_with_preloaded_language_models(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.with_preloaded_language_models();
        self_
    }

    /// Disable the high accuracy mode in order to save memory
    /// and increase performance.
    ///
    /// By default, Lingua's high detection accuracy comes at the cost
    /// of loading large language models into memory which might not be
    /// feasible for systems running low on resources.
    ///
    /// This method disables the high accuracy mode so that only a small
    /// subset of language models is loaded into memory. The downside of
    /// this approach is that detection accuracy for short texts consisting
    /// of less than 120 characters will drop significantly. However,
    /// detection accuracy for texts which are longer than 120 characters
    /// will remain mostly unaffected.
    #[pyo3(name = "with_low_accuracy_mode")]
    fn py_with_low_accuracy_mode(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.with_low_accuracy_mode();
        self_
    }

    /// Create and return the configured LanguageDetector instance.
    #[pyo3(name = "build")]
    fn py_build(&mut self) -> LanguageDetector {
        self.build()
    }
}

#[pymethods]
impl LanguageDetector {
    /// Clear all language models loaded by this LanguageDetector instance.
    ///
    /// This helps to free allocated memory previously consumed by the models.
    #[pyo3(name = "unload_language_models")]
    fn py_unload_language_models(&self) {
        self.unload_language_models()
    }

    /// Detect the language of given input text.
    ///
    /// If the language cannot be reliably detected, `None` is returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// `detect_languages_in_parallel_of` instead.
    #[pyo3(name = "detect_language_of")]
    fn py_detect_language_of(&self, text: String) -> Option<Language> {
        self.detect_language_of(text)
    }

    /// Detects the languages of all given input texts.
    ///
    /// If the language cannot be reliably detected for a text,
    /// `None` is put into the result list.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// `detect_language_of` instead.
    #[pyo3(name = "detect_languages_in_parallel_of")]
    fn py_detect_languages_in_parallel_of(&self, texts: Vec<String>) -> Vec<Option<Language>> {
        self.detect_languages_in_parallel_of(&texts)
    }

    /// Attempt to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A list of `DetectionResult` is returned containing an entry for each
    /// contiguous single-language text section as identified by the library.
    /// Each entry consists of the identified language, a start index and an
    /// end index. The indices denote the substring that has been identified
    /// as a contiguous single-language text section.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// `detect_multiple_languages_in_parallel_of` instead.
    #[pyo3(name = "detect_multiple_languages_of")]
    fn py_detect_multiple_languages_of(&self, text: String) -> Vec<DetectionResult> {
        self.detect_multiple_languages_of(text)
    }

    /// Attempt to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A list of `DetectionResult` is returned for each text containing an
    /// entry for each contiguous single-language text section as identified by
    /// the library. Each entry consists of the identified language, a start index
    /// and an end index. The indices denote the substring that has been identified
    /// as a contiguous single-language text section.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// `detect_multiple_languages_of` instead.
    #[pyo3(name = "detect_multiple_languages_in_parallel_of")]
    fn py_detect_multiple_languages_in_parallel_of(
        &self,
        texts: Vec<String>,
    ) -> Vec<Vec<DetectionResult>> {
        self.detect_multiple_languages_in_parallel_of(&texts)
    }

    /// Compute confidence values for each language supported
    /// by this detector for the given text.
    ///
    /// The confidence values denote how likely it is that the
    /// given text has been written in any of the languages
    /// supported by this detector.
    ///
    /// A list is returned containing those languages which the
    /// calling instance of `LanguageDetector` has been built from.
    /// The entries are sorted by their confidence value in
    /// descending order. Each value is a probability between
    /// 0.0 and 1.0. The probabilities of all languages will sum to 1.0.
    /// If the language is unambiguously identified by the rule engine,
    /// the value 1.0 will always be returned for this language. The
    /// other languages will receive a value of 0.0.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// `compute_language_confidence_values_in_parallel` instead.
    #[pyo3(name = "compute_language_confidence_values")]
    fn py_compute_language_confidence_values(&self, text: String) -> Vec<ConfidenceValue> {
        self.compute_language_confidence_values(text)
            .iter()
            .map(|tup| ConfidenceValue {
                language: tup.0,
                value: tup.1,
            })
            .collect()
    }

    /// Compute confidence values for each language supported by this detector for all the given
    /// input texts.
    ///
    /// The confidence values denote how likely it is that the given text has been written
    /// in any of the languages supported by this detector.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// `compute_language_confidence_values` instead.
    #[pyo3(name = "compute_language_confidence_values_in_parallel")]
    fn py_compute_language_confidence_values_in_parallel(
        &self,
        texts: Vec<String>,
    ) -> Vec<Vec<ConfidenceValue>> {
        self.compute_language_confidence_values_in_parallel(&texts)
            .iter()
            .map(|vector| {
                vector
                    .iter()
                    .map(|tup| ConfidenceValue {
                        language: tup.0,
                        value: tup.1,
                    })
                    .collect()
            })
            .collect()
    }

    /// Compute the confidence value for the given language and input text.
    ///
    /// The confidence value denotes how likely it is that the given text
    /// has been written in the given language. The value that this method
    /// computes is a number between 0.0 and 1.0. If the language is
    /// unambiguously identified by the rule engine, the value 1.0 will
    /// always be returned. If the given language is not supported by this
    /// detector instance, the value 0.0 will always be returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// `compute_language_confidence_in_parallel` instead.
    #[pyo3(name = "compute_language_confidence")]
    fn py_compute_language_confidence(&self, text: String, language: Language) -> f64 {
        self.compute_language_confidence(text, language)
    }

    /// Compute the confidence values of all input texts for the given language.
    ///
    /// A confidence value denotes how likely it is that a given text has been
    /// written in a given language.
    ///
    /// The values that this method computes are numbers between 0.0 and 1.0. If the language is
    /// unambiguously identified by the rule engine, the value 1.0 will always be returned.
    /// If the given language is not supported by this detector instance, the value 0.0 will
    /// always be returned.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// `compute_language_confidence` instead.
    #[pyo3(name = "compute_language_confidence_in_parallel")]
    fn py_compute_language_confidence_in_parallel(
        &self,
        texts: Vec<String>,
        language: Language,
    ) -> Vec<f64> {
        self.compute_language_confidence_in_parallel(&texts, language)
    }
}

#[pymethods]
impl LanguageModelFilesWriter {
    /// Create language model files and write them to a directory.
    ///
    /// Args:
    ///     input_file_path: The path to a txt file used for language
    ///         model creation. The assumed encoding of the txt file is UTF-8.
    ///     output_directory_path: The path to an existing directory where the
    ///         language model files are to be written.
    ///     language: The language for which to create language models.
    ///     char_class: A regex character class such as \\p{L} to restrict the
    ///         set of characters that the language models are built from.
    ///
    /// Raises:
    ///     Exception: if the input file path is not absolute or does not point
    ///         to an existing txt file; if the input file's encoding is not
    ///         UTF-8; if the output directory path is not absolute or does not
    ///         point to an existing directory; if the character class cannot
    ///         be compiled to a valid regular expression
    #[pyo3(name = "create_and_write_language_model_files")]
    #[classmethod]
    fn py_create_and_write_language_model_files(
        _cls: &PyType,
        input_file_path: PathBuf,
        output_directory_path: PathBuf,
        language: &Language,
        char_class: &str,
    ) -> PyResult<()> {
        convert_io_result_to_py_result(panic::catch_unwind(|| {
            Self::create_and_write_language_model_files(
                input_file_path.as_path(),
                output_directory_path.as_path(),
                language,
                char_class,
            )
        }))
    }
}

#[pymethods]
impl TestDataFilesWriter {
    /// Create test data files for accuracy report generation and
    /// write them to a directory.
    ///
    /// Args:
    ///     input_file_path: The path to a txt file used for test data
    ///         creation. The assumed encoding of the txt file is UTF-8.
    ///     output_directory_path: The path to an existing directory where
    ///         the test data files are to be written.
    ///     char_class: A regex character class such as \\p{L} to restrict
    ///         the set of characters that the test data are built from.
    ///     maximum_lines: The maximum number of lines each test data file
    ///         should have.
    ///
    /// Raises:
    ///     Exception: if the input file path is not absolute or does not point
    ///         to an existing txt file; if the input file's encoding is not
    ///         UTF-8; if the output directory path is not absolute or does not
    ///         point to an existing directory; if the character class cannot
    ///         be compiled to a valid regular expression
    #[pyo3(name = "create_and_write_test_data_files")]
    #[classmethod]
    fn py_create_and_write_test_data_files(
        _cls: &PyType,
        input_file_path: PathBuf,
        output_directory_path: PathBuf,
        char_class: &str,
        maximum_lines: u32,
    ) -> PyResult<()> {
        convert_io_result_to_py_result(panic::catch_unwind(|| {
            Self::create_and_write_test_data_files(
                input_file_path.as_path(),
                output_directory_path.as_path(),
                char_class,
                maximum_lines,
            )
        }))
    }
}

fn convert_io_result_to_py_result(
    io_result: Result<Result<(), io::Error>, Box<(dyn Any + Send + 'static)>>,
) -> PyResult<()> {
    match io_result {
        Ok(_) => Ok(()),
        Err(err) => {
            let panic_info = match err.downcast::<String>() {
                Ok(message) => *message,
                Err(err) => match err.downcast::<&str>() {
                    Ok(message) => message.to_string(),
                    Err(_) => "Unknown error occurred".to_string(),
                },
            };
            Err(PyException::new_err(panic_info))
        }
    }
}
