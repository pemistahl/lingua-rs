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

    #[getter]
    fn language(&self) -> Language {
        self.language
    }

    #[getter]
    fn value(&self) -> f64 {
        self.value
    }
}

#[pymethods]
impl DetectionResult {
    #[pyo3(name = "start_index")]
    #[getter]
    fn py_start_index(&self) -> usize {
        self.start_index()
    }

    #[pyo3(name = "end_index")]
    #[getter]
    fn py_end_index(&self) -> usize {
        self.end_index()
    }

    #[pyo3(name = "word_count")]
    #[getter]
    fn py_word_count(&self) -> usize {
        self.word_count()
    }

    #[pyo3(name = "language")]
    #[getter]
    fn py_language(&self) -> Language {
        self.language()
    }
}

#[pymethods]
impl Language {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    #[pyo3(name = "all")]
    #[classmethod]
    fn py_all(_cls: &PyType) -> HashSet<Self> {
        Self::all()
    }

    #[pyo3(name = "all_spoken_ones")]
    #[classmethod]
    fn py_all_spoken_ones(_cls: &PyType) -> HashSet<Self> {
        Self::all_spoken_ones()
    }

    #[pyo3(name = "all_with_arabic_script")]
    #[classmethod]
    fn py_all_with_arabic_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_arabic_script()
    }

    #[pyo3(name = "all_with_cyrillic_script")]
    #[classmethod]
    fn py_all_with_cyrillic_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_cyrillic_script()
    }

    #[pyo3(name = "all_with_devanagari_script")]
    #[classmethod]
    fn py_all_with_devanagari_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_devanagari_script()
    }

    #[pyo3(name = "all_with_latin_script")]
    #[classmethod]
    fn py_all_with_latin_script(_cls: &PyType) -> HashSet<Self> {
        Self::all_with_latin_script()
    }

    #[pyo3(name = "from_iso_code_639_1")]
    #[classmethod]
    fn py_from_iso_code_639_1(_cls: &PyType, iso_code: &IsoCode639_1) -> Self {
        Self::from_iso_code_639_1(iso_code)
    }

    #[pyo3(name = "from_iso_code_639_3")]
    #[classmethod]
    fn py_from_iso_code_639_3(_cls: &PyType, iso_code: &IsoCode639_3) -> Self {
        Self::from_iso_code_639_3(iso_code)
    }

    #[pyo3(name = "iso_code_639_1")]
    #[getter]
    fn py_iso_code_639_1(&self) -> IsoCode639_1 {
        self.iso_code_639_1()
    }

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
    #[pyo3(name = "from_all_languages")]
    #[classmethod]
    fn py_from_all_languages(_cls: &PyType) -> Self {
        Self::from_all_languages()
    }

    #[pyo3(name = "from_all_spoken_languages")]
    #[classmethod]
    fn py_from_all_spoken_languages(_cls: &PyType) -> Self {
        Self::from_all_spoken_languages()
    }

    #[pyo3(name = "from_all_languages_with_arabic_script")]
    #[classmethod]
    fn py_from_all_languages_with_arabic_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_arabic_script()
    }

    #[pyo3(name = "from_all_languages_with_cyrillic_script")]
    #[classmethod]
    fn py_from_all_languages_with_cyrillic_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_cyrillic_script()
    }

    #[pyo3(name = "from_all_languages_with_devanagari_script")]
    #[classmethod]
    fn py_from_all_languages_with_devanagari_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_devanagari_script()
    }

    #[pyo3(name = "from_all_languages_with_latin_script")]
    #[classmethod]
    fn py_from_all_languages_with_latin_script(_cls: &PyType) -> Self {
        Self::from_all_languages_with_latin_script()
    }

    #[pyo3(name = "from_all_languages_without")]
    #[pyo3(signature = (*languages))]
    #[classmethod]
    fn py_from_all_languages_without(_cls: &PyType, languages: &PyTuple) -> PyResult<Self> {
        let vector: Vec<Language> = languages.extract().unwrap();
        let result = panic::catch_unwind(|| Self::from_all_languages_without(&vector));
        match result {
            Ok(builder) => Ok(builder),
            Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
        }
    }

    #[pyo3(name = "from_languages")]
    #[pyo3(signature = (*languages))]
    #[classmethod]
    fn py_from_languages(_cls: &PyType, languages: &PyTuple) -> PyResult<Self> {
        let vector: Vec<Language> = languages.extract().unwrap();
        let result = panic::catch_unwind(|| Self::from_languages(&vector));
        match result {
            Ok(builder) => Ok(builder),
            Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
        }
    }

    #[pyo3(name = "from_iso_codes_639_1")]
    #[pyo3(signature = (*iso_codes))]
    #[classmethod]
    fn py_from_iso_codes_639_1(_cls: &PyType, iso_codes: &PyTuple) -> PyResult<Self> {
        let vector: Vec<IsoCode639_1> = iso_codes.extract().unwrap();
        let result = panic::catch_unwind(|| Self::from_iso_codes_639_1(&vector));
        match result {
            Ok(builder) => Ok(builder),
            Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
        }
    }

    #[pyo3(name = "from_iso_codes_639_3")]
    #[pyo3(signature = (*iso_codes))]
    #[classmethod]
    fn py_from_iso_codes_639_3(_cls: &PyType, iso_codes: &PyTuple) -> PyResult<Self> {
        let vector: Vec<IsoCode639_3> = iso_codes.extract().unwrap();
        let result = panic::catch_unwind(|| Self::from_iso_codes_639_3(&vector));
        match result {
            Ok(builder) => Ok(builder),
            Err(_) => Err(PyValueError::new_err(MISSING_LANGUAGE_MESSAGE)),
        }
    }

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

    #[pyo3(name = "with_preloaded_language_models")]
    fn py_with_preloaded_language_models(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.with_preloaded_language_models();
        self_
    }

    #[pyo3(name = "with_low_accuracy_mode")]
    fn py_with_low_accuracy_mode(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.with_low_accuracy_mode();
        self_
    }

    #[pyo3(name = "build")]
    fn py_build(&mut self) -> LanguageDetector {
        self.build()
    }
}

#[pymethods]
impl LanguageDetector {
    #[pyo3(name = "unload_language_models")]
    fn py_unload_language_models(&self) {
        self.unload_language_models()
    }

    #[pyo3(name = "detect_language_of")]
    fn py_detect_language_of(&self, text: String) -> Option<Language> {
        self.detect_language_of(text)
    }

    #[pyo3(name = "detect_multiple_languages_of")]
    fn py_detect_multiple_languages_of(&self, text: String) -> Vec<DetectionResult> {
        self.detect_multiple_languages_of(text)
    }

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

    #[pyo3(name = "compute_language_confidence")]
    fn py_compute_language_confidence(&self, text: String, language: Language) -> f64 {
        self.compute_language_confidence(text, language)
    }
}

#[pymethods]
impl LanguageModelFilesWriter {
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
