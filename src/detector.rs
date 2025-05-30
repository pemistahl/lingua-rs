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

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::LazyLock;

use ahash::AHashMap;
use compact_str::CompactString;
use dashmap::DashMap;
use fraction::{ToPrimitive, Zero};
use itertools::Itertools;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::alphabet::Alphabet;
use crate::constant::{
    CHARS_TO_LANGUAGES_MAPPING, JAPANESE_CHARACTER_SET, TOKENS_WITHOUT_WHITESPACE,
    TOKENS_WITH_OPTIONAL_WHITESPACE,
};
use crate::language::Language;
use crate::model::{
    create_lower_order_ngrams, create_ngrams, load_ngram_count_model, load_ngram_probability_model,
    NgramModelType,
};
use crate::ngram::NgramRef;
use crate::result::DetectionResult;

type ProbabilityMap = AHashMap<CompactString, f64>;
type LanguageModelMap = DashMap<Language, ProbabilityMap>;
type CountModelMap = DashMap<Language, HashSet<String>>;

static UNIGRAM_MODELS: LazyLock<LanguageModelMap> = LazyLock::new(DashMap::new);
static BIGRAM_MODELS: LazyLock<LanguageModelMap> = LazyLock::new(DashMap::new);
static TRIGRAM_MODELS: LazyLock<LanguageModelMap> = LazyLock::new(DashMap::new);
static QUADRIGRAM_MODELS: LazyLock<LanguageModelMap> = LazyLock::new(DashMap::new);
static FIVEGRAM_MODELS: LazyLock<LanguageModelMap> = LazyLock::new(DashMap::new);

static UNIQUE_UNIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static UNIQUE_BIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static UNIQUE_TRIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static UNIQUE_QUADRIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static UNIQUE_FIVEGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);

static MOST_COMMON_UNIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static MOST_COMMON_BIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static MOST_COMMON_TRIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static MOST_COMMON_QUADRIGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);
static MOST_COMMON_FIVEGRAM_MODELS: LazyLock<CountModelMap> = LazyLock::new(DashMap::new);

static LANGUAGES_WITH_SINGLE_UNIQUE_SCRIPT: LazyLock<HashSet<Language>> =
    LazyLock::new(Language::all_with_single_unique_script);

/// This struct detects the language of text.
///
/// A single instance of [`LanguageDetector`] can be used
/// safely in multiple threads. Multiple instances of
/// [`LanguageDetector`] share thread-safe access to the
/// language models, so every language model is loaded
/// into memory just once, no matter how many instances
/// of [`LanguageDetector`] have been created.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(module = "lingua"))]
pub struct LanguageDetector {
    languages: HashSet<Language>,
    minimum_relative_distance: f64,
    is_low_accuracy_mode_enabled: bool,
    is_built_from_one_language: bool,
    languages_with_unique_characters: HashSet<Language>,
    single_language_alphabets: HashMap<Alphabet, Language>,
    unigram_language_models: &'static LanguageModelMap,
    bigram_language_models: &'static LanguageModelMap,
    trigram_language_models: &'static LanguageModelMap,
    quadrigram_language_models: &'static LanguageModelMap,
    fivegram_language_models: &'static LanguageModelMap,
    unique_unigram_language_models: &'static CountModelMap,
    unique_bigram_language_models: &'static CountModelMap,
    unique_trigram_language_models: &'static CountModelMap,
    unique_quadrigram_language_models: &'static CountModelMap,
    unique_fivegram_language_models: &'static CountModelMap,
    most_common_unigram_language_models: &'static CountModelMap,
    most_common_bigram_language_models: &'static CountModelMap,
    most_common_trigram_language_models: &'static CountModelMap,
    most_common_quadrigram_language_models: &'static CountModelMap,
    most_common_fivegram_language_models: &'static CountModelMap,
}

impl LanguageDetector {
    pub(crate) fn from(
        languages: HashSet<Language>,
        minimum_relative_distance: f64,
        is_every_language_model_preloaded: bool,
        is_low_accuracy_mode_enabled: bool,
    ) -> Self {
        let is_built_from_one_language = languages.len() == 1;
        let mut detector = Self {
            languages: languages.clone(),
            minimum_relative_distance,
            is_low_accuracy_mode_enabled,
            is_built_from_one_language,
            languages_with_unique_characters: collect_languages_with_unique_characters(&languages),
            single_language_alphabets: collect_single_language_alphabets(&languages),
            unigram_language_models: &UNIGRAM_MODELS,
            bigram_language_models: &BIGRAM_MODELS,
            trigram_language_models: &TRIGRAM_MODELS,
            quadrigram_language_models: &QUADRIGRAM_MODELS,
            fivegram_language_models: &FIVEGRAM_MODELS,
            unique_unigram_language_models: &UNIQUE_UNIGRAM_MODELS,
            unique_bigram_language_models: &UNIQUE_BIGRAM_MODELS,
            unique_trigram_language_models: &UNIQUE_TRIGRAM_MODELS,
            unique_quadrigram_language_models: &UNIQUE_QUADRIGRAM_MODELS,
            unique_fivegram_language_models: &UNIQUE_FIVEGRAM_MODELS,
            most_common_unigram_language_models: &MOST_COMMON_UNIGRAM_MODELS,
            most_common_bigram_language_models: &MOST_COMMON_BIGRAM_MODELS,
            most_common_trigram_language_models: &MOST_COMMON_TRIGRAM_MODELS,
            most_common_quadrigram_language_models: &MOST_COMMON_QUADRIGRAM_MODELS,
            most_common_fivegram_language_models: &MOST_COMMON_FIVEGRAM_MODELS,
        };

        if is_every_language_model_preloaded {
            detector.preload_language_models(&languages);
        }

        if is_built_from_one_language {
            detector.preload_unique_ngram_models();
            detector.preload_most_common_ngram_models();
        }

        detector
    }

    fn preload_unique_ngram_models(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = self.languages.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = self.languages.iter();

        languages_iter.for_each(|language| {
            load_count_model(
                self.unique_unigram_language_models,
                *language,
                1,
                NgramModelType::Unique,
            );
            load_count_model(
                self.unique_bigram_language_models,
                *language,
                2,
                NgramModelType::Unique,
            );
            load_count_model(
                self.unique_trigram_language_models,
                *language,
                3,
                NgramModelType::Unique,
            );
            load_count_model(
                self.unique_quadrigram_language_models,
                *language,
                4,
                NgramModelType::Unique,
            );
            load_count_model(
                self.unique_fivegram_language_models,
                *language,
                5,
                NgramModelType::Unique,
            );
        });
    }

    fn preload_most_common_ngram_models(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = self.languages.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = self.languages.iter();

        languages_iter.for_each(|language| {
            load_count_model(
                self.most_common_unigram_language_models,
                *language,
                1,
                NgramModelType::MostCommon,
            );
            load_count_model(
                self.most_common_bigram_language_models,
                *language,
                2,
                NgramModelType::MostCommon,
            );
            load_count_model(
                self.most_common_trigram_language_models,
                *language,
                3,
                NgramModelType::MostCommon,
            );
            load_count_model(
                self.most_common_quadrigram_language_models,
                *language,
                4,
                NgramModelType::MostCommon,
            );
            load_count_model(
                self.most_common_fivegram_language_models,
                *language,
                5,
                NgramModelType::MostCommon,
            );
        });
    }

    fn preload_language_models(&mut self, languages: &HashSet<Language>) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = languages.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = languages.iter();

        languages_iter.for_each(|language| {
            load_probability_model(self.trigram_language_models, *language, 3);

            if !self.is_low_accuracy_mode_enabled {
                load_probability_model(self.unigram_language_models, *language, 1);
                load_probability_model(self.bigram_language_models, *language, 2);
                load_probability_model(self.quadrigram_language_models, *language, 4);
                load_probability_model(self.fivegram_language_models, *language, 5);
            }
        });
    }

    /// Clears all language models loaded by this [`LanguageDetector`] instance
    /// and frees allocated memory previously consumed by the models.
    ///
    /// The freed memory will not be returned back to the operating system
    /// but will be reused e.g. for language models loaded by different
    /// [`LanguageDetector`] instances.
    pub fn unload_language_models(&self) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = self.languages.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = self.languages.iter();

        languages_iter.for_each(|language| {
            self.trigram_language_models.remove(language);

            if !self.is_low_accuracy_mode_enabled {
                self.unigram_language_models.remove(language);
                self.bigram_language_models.remove(language);
                self.quadrigram_language_models.remove(language);
                self.fivegram_language_models.remove(language);
            }

            if self.is_built_from_one_language {
                self.unigram_language_models.remove(language);
                self.unique_bigram_language_models.remove(language);
                self.unique_trigram_language_models.remove(language);
                self.unique_quadrigram_language_models.remove(language);
                self.unique_fivegram_language_models.remove(language);
                self.most_common_unigram_language_models.remove(language);
                self.most_common_bigram_language_models.remove(language);
                self.most_common_trigram_language_models.remove(language);
                self.most_common_quadrigram_language_models.remove(language);
                self.most_common_fivegram_language_models.remove(language);
            }
        });

        self.trigram_language_models.shrink_to_fit();

        if !self.is_low_accuracy_mode_enabled {
            self.unigram_language_models.shrink_to_fit();
            self.bigram_language_models.shrink_to_fit();
            self.quadrigram_language_models.shrink_to_fit();
            self.fivegram_language_models.shrink_to_fit();
        }

        if self.is_built_from_one_language {
            self.unigram_language_models.shrink_to_fit();
            self.unique_bigram_language_models.shrink_to_fit();
            self.unique_trigram_language_models.shrink_to_fit();
            self.unique_quadrigram_language_models.shrink_to_fit();
            self.unique_fivegram_language_models.shrink_to_fit();
            self.most_common_unigram_language_models.shrink_to_fit();
            self.most_common_bigram_language_models.shrink_to_fit();
            self.most_common_trigram_language_models.shrink_to_fit();
            self.most_common_quadrigram_language_models.shrink_to_fit();
            self.most_common_fivegram_language_models.shrink_to_fit();
        }
    }

    /// Detects the language of given input text.
    /// If the language cannot be reliably detected, [`None`] is returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`detect_languages_in_parallel_of`](#method.detect_languages_in_parallel_of)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let detected_language = detector.detect_language_of("languages are awesome");
    ///
    /// assert_eq!(detected_language, Some(English));
    /// ```
    pub fn detect_language_of<T: Into<String>>(&self, text: T) -> Option<Language> {
        self.detect_language_from_languages(text, &self.languages)
    }

    /// Detects the languages of all given input texts.
    /// If the language cannot be reliably detected for a text,
    /// [`None`] is put into the result vector.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// [`detect_language_of`](#method.detect_language_of) instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let detected_languages = detector.detect_languages_in_parallel_of(&[
    ///     "languages are awesome",
    ///     "Sprachen sind großartig",
    ///     "des langues sont géniales",
    ///     "los idiomas son geniales"
    /// ]);
    ///
    /// assert_eq!(
    ///     detected_languages,
    ///     vec![
    ///         Some(English),
    ///         Some(German),
    ///         Some(French),
    ///         Some(Spanish)
    ///     ]
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn detect_languages_in_parallel_of<T: Into<String> + Clone + Send + Sync>(
        &self,
        texts: &[T],
    ) -> Vec<Option<Language>> {
        texts
            .into_par_iter()
            .map(|text| self.detect_language_of(text.clone()))
            .collect()
    }

    fn detect_language_from_languages<T: Into<String>>(
        &self,
        text: T,
        languages: &HashSet<Language>,
    ) -> Option<Language> {
        let confidence_values =
            self.compute_language_confidence_values_for_languages(text, languages);

        if confidence_values.is_empty() {
            return None;
        }

        let (most_likely_language, most_likely_language_probability) =
            &confidence_values.first().unwrap();

        if confidence_values.len() == 1 {
            if most_likely_language_probability.is_zero() {
                return None;
            }
            return Some(*most_likely_language);
        }

        let (_, second_most_likely_language_probability) = &confidence_values.get(1).unwrap();

        if (most_likely_language_probability - second_most_likely_language_probability).abs()
            < f64::EPSILON
        {
            return None;
        }

        if (most_likely_language_probability - second_most_likely_language_probability)
            < self.minimum_relative_distance
        {
            return None;
        }

        Some(*most_likely_language)
    }

    /// Attempts to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A vector of [`DetectionResult`] is returned containing an entry for each contiguous
    /// single-language text section as identified by the library. Each entry consists
    /// of the identified language, a start index and an end index. The indices denote
    /// the substring that has been identified as a contiguous single-language text section.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`detect_multiple_languages_in_parallel_of`](#method.detect_multiple_languages_in_parallel_of)
    /// instead.
    /// ```
    /// use lingua::Language::{English, French, German};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German
    /// ])
    /// .build();
    ///
    /// let sentence = "Parlez-vous français? \
    ///     Ich spreche Französisch nur ein bisschen. \
    ///     A little bit is better than nothing.";
    ///
    /// let results = detector.detect_multiple_languages_of(sentence);
    ///
    /// if let [first, second, third] = &results[..] {
    ///     assert_eq!(first.language(), French);
    ///     assert_eq!(
    ///         &sentence[first.start_index()..first.end_index()],
    ///         "Parlez-vous français? "
    ///     );
    ///
    ///     assert_eq!(second.language(), German);
    ///     assert_eq!(
    ///         &sentence[second.start_index()..second.end_index()],
    ///         "Ich spreche Französisch nur ein bisschen. "
    ///     );
    ///
    ///     assert_eq!(third.language(), English);
    ///     assert_eq!(
    ///         &sentence[third.start_index()..third.end_index()],
    ///         "A little bit is better than nothing."
    ///     );
    /// }
    /// ```
    pub fn detect_multiple_languages_of<T: Into<String>>(&self, text: T) -> Vec<DetectionResult> {
        let text_str = text.into();

        if text_str.is_empty() {
            return vec![];
        }

        let tokens_without_whitespace = TOKENS_WITHOUT_WHITESPACE
            .find_iter(&text_str)
            .map(|mat| mat.as_str())
            .collect_vec();

        if tokens_without_whitespace.is_empty() {
            return vec![];
        }

        let mut results = vec![];
        let mut language_counts = HashMap::new();

        let language = self.detect_language_of(&text_str);
        if let Some(lang) = language {
            increment_counter(&mut language_counts, lang, 1);
        }

        for word in tokens_without_whitespace.iter() {
            if word.chars().count() < 5 {
                continue;
            }
            let language = self.detect_language_of(*word);
            if let Some(lang) = language {
                increment_counter(&mut language_counts, lang, 1);
            }
        }

        let languages = language_counts
            .keys()
            .cloned()
            .collect::<HashSet<Language>>();

        if languages.len() == 1 {
            let result = DetectionResult {
                start_index: 0,
                end_index: text_str.len(),
                word_count: tokens_without_whitespace.len(),
                language: *languages.iter().next().unwrap(),
            };
            results.push(result);
        } else {
            let mut current_start_index = 0;
            let mut current_end_index = 0;
            let mut word_count = 0;
            let mut current_language = None;

            let last_index = TOKENS_WITH_OPTIONAL_WHITESPACE.find_iter(&text_str).count() - 1;
            let token_matches = TOKENS_WITH_OPTIONAL_WHITESPACE.find_iter(&text_str);

            for (i, token_match) in token_matches.enumerate() {
                let word = token_match.as_str();
                let language = self.detect_language_from_languages(word, &languages);

                if i == 0 || (current_language.is_none() && language.is_some()) {
                    current_language = language;
                }

                if let Some(lang) = language {
                    if let Some(current_lang) = current_language {
                        if lang != current_lang {
                            let result = DetectionResult {
                                start_index: current_start_index,
                                end_index: current_end_index,
                                word_count,
                                language: current_lang,
                            };
                            results.push(result);
                            current_start_index = current_end_index;
                            current_language = Some(lang);
                            word_count = 0;
                        }
                    }
                }

                current_end_index = token_match.end();
                word_count += 1;

                if i == last_index {
                    if let Some(current_lang) = current_language {
                        let result = DetectionResult {
                            start_index: current_start_index,
                            end_index: text_str.len(),
                            word_count,
                            language: current_lang,
                        };
                        results.push(result);
                    }
                }
            }

            if results.len() > 1 {
                let mut mergeable_result_indices = vec![];

                for (i, result) in results.iter().enumerate() {
                    if result.word_count == 1 {
                        mergeable_result_indices.push(i);
                    }
                }

                merge_adjacent_results(&mut results, &mut mergeable_result_indices);

                if results.len() > 1 {
                    mergeable_result_indices.clear();

                    for i in 0..results.len() - 1 {
                        if results[i].language == results[i + 1].language {
                            mergeable_result_indices.push(i + 1);
                        }
                    }

                    merge_adjacent_results(&mut results, &mut mergeable_result_indices);
                }
            }
        }

        results
    }

    /// Attempts to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A vector of [`DetectionResult`] is returned for each text containing an
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
    /// [`detect_multiple_languages_of`](#method.detect_multiple_languages_of)
    /// instead.
    #[cfg(not(target_family = "wasm"))]
    pub fn detect_multiple_languages_in_parallel_of<T: Into<String> + Clone + Send + Sync>(
        &self,
        texts: &[T],
    ) -> Vec<Vec<DetectionResult>> {
        texts
            .into_par_iter()
            .map(|text| self.detect_multiple_languages_of(text.clone()))
            .collect()
    }

    /// Computes confidence values for each language supported by this detector for the given
    /// input text. These values denote how likely it is that the given text has been written
    /// in any of the languages supported by this detector.
    ///
    /// A vector of two-element tuples is returned containing those languages which the
    /// calling instance of [`LanguageDetector`] has been built from, together with their
    /// confidence values. The entries are sorted by their confidence value in descending order.
    /// Each value is a probability between 0.0 and 1.0. The probabilities of all languages will
    /// sum to 1.0. If the language is unambiguously identified by the rule engine, the value
    /// 1.0 will always be returned for this language. The other languages will receive a value
    /// of 0.0.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`compute_language_confidence_values_in_parallel`](#method.compute_language_confidence_values_in_parallel)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let confidence_values = detector
    ///     .compute_language_confidence_values("languages are awesome")
    ///     .into_iter()
    ///     .map(|(language, confidence)| (language, (confidence * 100.0).round() / 100.0))
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     confidence_values,
    ///     vec![
    ///         (English, 0.93),
    ///         (French, 0.04),
    ///         (German, 0.02),
    ///         (Spanish, 0.01)
    ///     ]
    /// );
    /// ```
    pub fn compute_language_confidence_values<T: Into<String>>(
        &self,
        text: T,
    ) -> Vec<(Language, f64)> {
        self.compute_language_confidence_values_for_languages(text, &self.languages)
    }

    /// Computes confidence values for each language supported by this detector for all the given
    /// input texts. The confidence values denote how likely it is that the given text has been written
    /// in any of the languages supported by this detector.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// [`compute_language_confidence_values`](#method.compute_language_confidence_values)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let confidence_values = detector
    ///     .compute_language_confidence_values_in_parallel(&[
    ///         "languages are awesome",
    ///         "Sprachen sind großartig"
    ///     ])
    ///     .into_iter()
    ///     .map(|vector| {
    ///         vector
    ///             .into_iter()
    ///             .map(|(language, confidence)| {
    ///                 (language, (confidence * 100.0).round() / 100.0)
    ///             })
    ///             .collect::<Vec<_>>()
    ///     })
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     confidence_values,
    ///     vec![
    ///         vec![
    ///             (English, 0.93),
    ///             (French, 0.04),
    ///             (German, 0.02),
    ///             (Spanish, 0.01)
    ///         ],
    ///         vec![
    ///             (German, 0.99),
    ///             (Spanish, 0.01),
    ///             (English, 0.0),
    ///             (French, 0.0)
    ///         ]
    ///     ]
    /// );
    #[cfg(not(target_family = "wasm"))]
    pub fn compute_language_confidence_values_in_parallel<T: Into<String> + Clone + Send + Sync>(
        &self,
        texts: &[T],
    ) -> Vec<Vec<(Language, f64)>> {
        texts
            .into_par_iter()
            .map(|text| self.compute_language_confidence_values(text.clone()))
            .collect()
    }

    fn compute_language_confidence_values_for_languages<T: Into<String>>(
        &self,
        text: T,
        languages: &HashSet<Language>,
    ) -> Vec<(Language, f64)> {
        let mut values = Vec::with_capacity(languages.len());

        for language in languages {
            values.push((*language, 0.0));
        }

        let text_str = text.into();
        let words = split_text_into_words(&text_str);

        if words.is_empty() {
            return values;
        }

        if self.is_built_from_one_language {
            if let Some(language) = self.detect_language_with_unique_and_common_ngrams(&words) {
                update_confidence_values(&mut values, language, 1.0);
                values.sort_by(confidence_values_comparator);
                return values;
            }
        }

        if let Some(language) = self.detect_language_with_rules(&words, languages) {
            update_confidence_values(&mut values, language, 1.0);
            values.sort_by(confidence_values_comparator);
            return values;
        }

        if self.is_built_from_one_language {
            return values;
        }

        let filtered_languages = self.filter_languages_by_rules(&words, languages);

        if filtered_languages.len() == 1 {
            let filtered_language = filtered_languages.into_iter().next().unwrap();
            update_confidence_values(&mut values, filtered_language, 1.0);
            values.sort_by(confidence_values_comparator);
            return values;
        }

        let character_count: usize = words.iter().map(|word| word.chars().count()).sum();

        if self.is_low_accuracy_mode_enabled && character_count < 3 {
            values.sort_by(confidence_values_comparator);
            return values;
        }

        let ngram_length_range = if character_count >= 120 || self.is_low_accuracy_mode_enabled {
            3..4usize
        } else {
            1..6usize
        };

        let mut unigram_counts: Option<HashMap<Language, u32>> = None;
        let mut all_probabilities: Vec<HashMap<Language, f64>> = vec![];

        for ngram_length in ngram_length_range {
            if character_count >= ngram_length {
                let ngram_model = create_lower_order_ngrams(&words, ngram_length);
                if ngram_length == 1 {
                    unigram_counts = Some(self.count_unigrams(&ngram_model, &filtered_languages))
                }
                let probabilities =
                    self.compute_language_probabilities(&ngram_model, &filtered_languages);
                all_probabilities.push(probabilities);
            }
        }

        let summed_up_probabilities =
            sum_up_probabilities(&all_probabilities, &unigram_counts, filtered_languages);

        if summed_up_probabilities.is_empty() {
            values.sort_by(confidence_values_comparator);
            return values;
        }

        compute_confidence_values(&mut values, all_probabilities, summed_up_probabilities);

        values
    }

    /// Computes the confidence value for the given language and input text. This value denotes
    /// how likely it is that the given text has been written in the given language.
    ///
    /// The value that this method computes is a number between 0.0 and 1.0. If the language is
    /// unambiguously identified by the rule engine, the value 1.0 will always be returned.
    /// If the given language is not supported by this detector instance, the value 0.0 will
    /// always be returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`compute_language_confidence_in_parallel`](#method.compute_language_confidence_in_parallel)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let confidence = detector.compute_language_confidence("languages are awesome", French);
    /// let rounded_confidence = (confidence * 100.0).round() / 100.0;
    ///
    /// assert_eq!(rounded_confidence, 0.04);
    /// ```
    pub fn compute_language_confidence<T: Into<String>>(&self, text: T, language: Language) -> f64 {
        self.compute_language_confidence_values(text)
            .iter()
            .find(|(lang, _)| *lang == language)
            .map(|(_, confidence)| *confidence)
            .unwrap_or(0.0)
    }

    /// Computes the confidence values of all input texts for the given language.
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
    /// [`compute_language_confidence`](#method.compute_language_confidence)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let confidence_values = detector.compute_language_confidence_in_parallel(
    ///     &[
    ///         "languages are awesome",
    ///         "Sprachen sind großartig",
    ///         "des langues sont géniales",
    ///         "los idiomas son geniales"
    ///     ],
    ///     French
    /// )
    /// .into_iter()
    /// .map(|confidence| (confidence * 100.0).round() / 100.0)
    /// .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     confidence_values,
    ///     vec![
    ///         0.04,
    ///         0.0,
    ///         0.92,
    ///         0.07
    ///     ]
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn compute_language_confidence_in_parallel<T: Into<String> + Clone + Send + Sync>(
        &self,
        texts: &[T],
        language: Language,
    ) -> Vec<f64> {
        texts
            .into_par_iter()
            .map(|text| self.compute_language_confidence(text.clone(), language))
            .collect()
    }

    fn detect_language_with_unique_and_common_ngrams(&self, words: &[String]) -> Option<Language> {
        for ngram_length in (1..6usize).rev() {
            let ngrams = create_ngrams(words, ngram_length);
            let mut optional_language: Option<Language> = None;
            for language in self.languages.iter() {
                if ngram_length == 1 {
                    let is_hindi = cfg!(feature = "hindi")
                        && language == &Language::from_str("Hindi").unwrap();
                    let is_marathi = cfg!(feature = "marathi")
                        && language == &Language::from_str("Marathi").unwrap();
                    let is_japanese = cfg!(feature = "japanese")
                        && language == &Language::from_str("Japanese").unwrap();

                    if is_hindi
                        || is_marathi
                        || (is_japanese && self.is_built_from_one_language)
                        || LANGUAGES_WITH_SINGLE_UNIQUE_SCRIPT.contains(language)
                    {
                        optional_language = self.search_unique_and_most_common_ngrams(
                            *language,
                            &ngrams,
                            ngram_length,
                        );
                    }
                } else if ngram_length == 2 {
                    optional_language = search_unique_ngrams(
                        self.unique_bigram_language_models,
                        *language,
                        &ngrams,
                    );
                } else {
                    optional_language =
                        self.search_unique_and_most_common_ngrams(*language, &ngrams, ngram_length);
                }
                if optional_language.is_some() {
                    return optional_language;
                }
            }
        }
        None
    }

    fn search_unique_and_most_common_ngrams(
        &self,
        language: Language,
        ngrams: &HashSet<NgramRef>,
        ngram_length: usize,
    ) -> Option<Language> {
        let (unique_language_models, most_common_language_models) = match ngram_length {
            5 => (
                self.unique_fivegram_language_models,
                self.most_common_fivegram_language_models,
            ),
            4 => (
                self.unique_quadrigram_language_models,
                self.most_common_quadrigram_language_models,
            ),
            3 => (
                self.unique_trigram_language_models,
                self.most_common_trigram_language_models,
            ),
            2 => (
                self.unique_bigram_language_models,
                self.most_common_bigram_language_models,
            ),
            1 => (
                self.unique_unigram_language_models,
                self.most_common_unigram_language_models,
            ),
            _ => panic!("unsupported ngram length detected: {}", ngram_length),
        };
        match search_unique_ngrams(unique_language_models, language, ngrams) {
            Some(language) => Some(language),
            None => search_most_common_ngrams(
                most_common_language_models,
                language,
                ngrams,
                self.is_built_from_one_language,
            ),
        }
    }

    fn detect_language_with_rules(
        &self,
        words: &[String],
        languages: &HashSet<Language>,
    ) -> Option<Language> {
        let mut total_language_counts = HashMap::<Option<Language>, u32>::new();
        let half_word_count = (words.len() as f64) * 0.5;

        for word in words {
            let mut word_language_counts = HashMap::<Language, u32>::new();

            for character in word.chars() {
                let mut is_match = false;

                for (alphabet, language) in self.single_language_alphabets.iter() {
                    if alphabet.matches_char(character) {
                        increment_counter(&mut word_language_counts, *language, 1);
                        is_match = true;
                        break;
                    }
                }

                if !is_match {
                    if cfg!(feature = "chinese") && Alphabet::Han.matches_char(character) {
                        increment_counter(
                            &mut word_language_counts,
                            Language::from_str("Chinese").unwrap(),
                            1,
                        );
                    } else if cfg!(feature = "japanese")
                        && JAPANESE_CHARACTER_SET.is_char_match(character)
                    {
                        increment_counter(
                            &mut word_language_counts,
                            Language::from_str("Japanese").unwrap(),
                            1,
                        );
                    } else if Alphabet::Latin.matches_char(character)
                        || Alphabet::Cyrillic.matches_char(character)
                        || Alphabet::Devanagari.matches_char(character)
                    {
                        self.languages_with_unique_characters
                            .iter()
                            .filter(|it| it.unique_characters().unwrap().contains(character))
                            .for_each(|it| increment_counter(&mut word_language_counts, *it, 1));
                    }
                }
            }

            if word_language_counts.is_empty() {
                increment_counter(&mut total_language_counts, None, 1);
            } else if word_language_counts.len() == 1 {
                let counted_languages = word_language_counts.keys().collect_vec();
                let language = *counted_languages.first().unwrap();
                if languages.contains(language) {
                    increment_counter(&mut total_language_counts, Some(*language), 1);
                } else {
                    increment_counter(&mut total_language_counts, None, 1);
                }
            } else if cfg!(feature = "chinese")
                && cfg!(feature = "japanese")
                && word_language_counts.contains_key(&Language::from_str("Chinese").unwrap())
                && word_language_counts.contains_key(&Language::from_str("Japanese").unwrap())
            {
                increment_counter(
                    &mut total_language_counts,
                    Some(Language::from_str("Japanese").unwrap()),
                    1,
                );
            } else {
                let sorted_word_language_counts = word_language_counts
                    .into_iter()
                    .sorted_by(|(_, first_count), (_, second_count)| second_count.cmp(first_count))
                    .collect_vec();
                let (most_frequent_language, first_count) = &sorted_word_language_counts[0];
                let (_, second_count) = &sorted_word_language_counts[1];

                if first_count > second_count && languages.contains(most_frequent_language) {
                    increment_counter(&mut total_language_counts, Some(*most_frequent_language), 1);
                } else {
                    increment_counter(&mut total_language_counts, None, 1);
                }
            }
        }

        let unknown_language_count = *total_language_counts.get(&None).unwrap_or(&0) as f64;

        if unknown_language_count < half_word_count {
            total_language_counts.remove(&None);
        }

        if total_language_counts.is_empty() {
            return None;
        }

        if total_language_counts.len() == 1 {
            return *total_language_counts.iter().next().unwrap().0;
        }

        let sorted_total_language_counts = total_language_counts
            .into_iter()
            .sorted_by(|(_, first_count), (_, second_count)| second_count.cmp(first_count))
            .collect_vec();
        let (most_frequent_language, first_count) = sorted_total_language_counts[0];
        let (second_frequent_language, second_count) = sorted_total_language_counts[1];
        if cfg!(feature = "chinese")
            && cfg!(feature = "japanese")
            && hashset!(most_frequent_language, second_frequent_language)
                == hashset!(
                    Some(Language::from_str("Japanese").unwrap()),
                    Some(Language::from_str("Chinese").unwrap())
                )
        {
            return Some(Language::from_str("Japanese").unwrap());
        }

        if first_count == second_count {
            return None;
        }

        most_frequent_language
    }

    fn filter_languages_by_rules(
        &self,
        words: &[String],
        languages: &HashSet<Language>,
    ) -> HashSet<Language> {
        let mut detected_alphabets = HashMap::<Alphabet, u32>::new();
        let half_word_count = (words.len() as f64) * 0.5;

        for word in words.iter() {
            for alphabet in Alphabet::iter() {
                if alphabet.matches(word) {
                    increment_counter(
                        &mut detected_alphabets,
                        alphabet,
                        word.chars().count() as u32,
                    );
                    break;
                }
            }
        }

        if detected_alphabets.is_empty() {
            return languages.clone();
        }

        if detected_alphabets.len() > 1 {
            let mut distinct_alphabets = hashset!();
            for count in detected_alphabets.values() {
                distinct_alphabets.insert(count);
            }
            if distinct_alphabets.len() == 1 {
                return languages.clone();
            }
        }

        let most_frequent_alphabet = detected_alphabets
            .into_iter()
            .sorted_by(
                |(first_alphabet, first_count), (second_alphabet, second_count)| {
                    let ordering = second_count.cmp(first_count);
                    match ordering {
                        Ordering::Equal => first_alphabet.cmp(second_alphabet),
                        _ => ordering,
                    }
                },
            )
            .next()
            .unwrap()
            .0;

        let filtered_languages = languages
            .iter()
            .cloned()
            .filter(|it| it.alphabets().contains(&most_frequent_alphabet))
            .collect::<HashSet<_>>();

        let mut language_counts = HashMap::<&Language, u32>::new();

        for (characters, langs) in CHARS_TO_LANGUAGES_MAPPING.iter() {
            let relevant_languages = filtered_languages
                .intersection(langs)
                .collect::<HashSet<_>>();

            for word in words.iter() {
                for character in characters.chars() {
                    if word.contains(character) {
                        for language in relevant_languages.iter() {
                            increment_counter(&mut language_counts, language, 1);
                        }
                    }
                }
            }
        }

        let languages_subset = language_counts
            .into_iter()
            .filter(|(_, count)| (*count as f64) >= half_word_count)
            .map(|(language, _)| *language)
            .collect::<HashSet<_>>();

        if !languages_subset.is_empty() {
            languages_subset
        } else {
            filtered_languages
        }
    }

    fn compute_language_probabilities(
        &self,
        model: &[Vec<NgramRef>],
        filtered_languages: &HashSet<Language>,
    ) -> HashMap<Language, f64> {
        let mut probabilities = hashmap!();
        for language in filtered_languages.iter() {
            let sum = self.compute_sum_of_ngram_probabilities(*language, model);
            if sum < 0.0 {
                probabilities.insert(*language, sum);
            }
        }
        probabilities
    }

    fn compute_sum_of_ngram_probabilities(
        &self,
        language: Language,
        ngram_model: &[Vec<NgramRef>],
    ) -> f64 {
        let mut sum = 0.0;
        for ngrams in ngram_model.iter() {
            for ngram in ngrams {
                if let Some(probability) = self.look_up_ngram_probability(language, ngram) {
                    sum += probability;
                    break;
                }
            }
        }
        sum
    }

    fn look_up_ngram_probability(&self, language: Language, ngram: &NgramRef) -> Option<f64> {
        let ngram_length = ngram.value.chars().count();
        let language_models = match ngram_length {
            5 => self.fivegram_language_models,
            4 => self.quadrigram_language_models,
            3 => self.trigram_language_models,
            2 => self.bigram_language_models,
            1 => self.unigram_language_models,
            0 => panic!("zerogram detected"),
            _ => panic!("unsupported ngram length detected: {}", ngram_length),
        };

        if !load_probability_model(language_models, language, ngram_length) {
            return None;
        }

        language_models
            .get(&language)
            .unwrap()
            .get(ngram.value)
            .copied()
    }

    fn count_unigrams(
        &self,
        unigram_model: &[Vec<NgramRef>],
        filtered_languages: &HashSet<Language>,
    ) -> HashMap<Language, u32> {
        let mut unigram_counts = HashMap::new();
        for language in filtered_languages.iter() {
            for unigrams in unigram_model.iter() {
                if self
                    .look_up_ngram_probability(*language, unigrams.first().unwrap())
                    .is_some()
                {
                    increment_counter(&mut unigram_counts, *language, 1);
                }
            }
        }
        unigram_counts
    }
}

pub(crate) fn split_text_into_words(text: &str) -> Vec<String> {
    TOKENS_WITHOUT_WHITESPACE
        .find_iter(&text.trim().to_lowercase())
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn increment_counter<T: Eq + Hash>(counts: &mut HashMap<T, u32>, key: T, value: u32) {
    let counter = counts.entry(key).or_insert(0);
    *counter += value;
}

fn load_count_model(
    language_models: &'static CountModelMap,
    language: Language,
    ngram_length: usize,
    model_type: NgramModelType,
) -> bool {
    if language_models.contains_key(&language) {
        return true;
    }
    match load_ngram_count_model(language, ngram_length, model_type) {
        Some(model) => {
            language_models.insert(language, model.ngrams);
            true
        }
        None => false,
    }
}

fn load_probability_model(
    language_models: &'static LanguageModelMap,
    language: Language,
    ngram_length: usize,
) -> bool {
    if language_models.contains_key(&language) {
        return true;
    }
    match load_ngram_probability_model(language, ngram_length) {
        Some(model) => {
            language_models.insert(
                language,
                model
                    .ngrams
                    .iter()
                    .map(|(key, value)| (key.clone(), value.to_f64().unwrap().ln()))
                    .collect(),
            );
            true
        }
        None => false,
    }
}

fn search_unique_ngrams(
    language_models: &'static CountModelMap,
    language: Language,
    ngrams: &HashSet<NgramRef>,
) -> Option<Language> {
    match language_models.get(&language) {
        Some(entry) => ngrams
            .iter()
            .find(|ngram| entry.value().contains(ngram.value))
            .map(|_| language),
        None => None,
    }
}

fn search_most_common_ngrams(
    language_models: &'static CountModelMap,
    language: Language,
    ngrams: &HashSet<NgramRef>,
    is_built_from_one_language: bool,
) -> Option<Language> {
    if !is_built_from_one_language {
        return None;
    }
    match language_models.get(&language) {
        Some(entry) => ngrams
            .iter()
            .find(|ngram| entry.value().contains(ngram.value))
            .map(|_| language),
        None => None,
    }
}

fn sum_up_probabilities(
    probability_maps: &[HashMap<Language, f64>],
    unigram_counts: &Option<HashMap<Language, u32>>,
    filtered_languages: HashSet<Language>,
) -> HashMap<Language, f64> {
    let mut summed_up_probabilities = hashmap!();
    for language in filtered_languages.iter() {
        let mut sum: f64 = probability_maps
            .iter()
            .map(|it| match it.get(language) {
                Some(probability) => *probability,
                None => 0.0,
            })
            .sum();

        if let Some(counts) = unigram_counts {
            if counts.contains_key(language) {
                sum /= *counts.get(language).unwrap() as f64;
            }
        }

        if sum != 0.0 {
            summed_up_probabilities.insert(*language, sum.exp());
        }
    }
    summed_up_probabilities
}

fn collect_languages_with_unique_characters(languages: &HashSet<Language>) -> HashSet<Language> {
    languages
        .iter()
        .filter(|it| it.unique_characters().is_some())
        .cloned()
        .collect()
}

fn collect_single_language_alphabets(languages: &HashSet<Language>) -> HashMap<Alphabet, Language> {
    Alphabet::all_supporting_single_language()
        .into_iter()
        .filter(|(_, language)| languages.contains(language))
        .collect()
}

fn confidence_values_comparator(first: &(Language, f64), second: &(Language, f64)) -> Ordering {
    let sorted_by_probability = second.1.partial_cmp(&first.1).unwrap();
    let sorted_by_language = first.0.partial_cmp(&second.0).unwrap();
    sorted_by_probability.then(sorted_by_language)
}

fn compute_confidence_values(
    values: &mut Vec<(Language, f64)>,
    probability_maps: Vec<HashMap<Language, f64>>,
    probabilities: HashMap<Language, f64>,
) {
    let denominator: f64 = probabilities.values().sum();

    // If the denominator is still zero, the exponent of the summed
    // log probabilities is too large to be computed for very long input strings.
    // So we simply set the probability of the most likely language to 1.0 and
    // leave the other languages at 0.0.
    if denominator.is_zero() {
        // For very long inputs, only trigrams are used, so we safely access them at index 0.
        let probability_map = &probability_maps[0];
        let most_likely_language = *probability_map
            .iter()
            .max_by(|(_, first_probability), (_, second_probability)| {
                first_probability.total_cmp(second_probability)
            })
            .unwrap()
            .0;

        update_confidence_values(values, most_likely_language, 1.0);
    } else {
        for (language, probability) in probabilities {
            for value in &mut *values {
                if value.0 == language {
                    // Apply softmax function
                    let normalized_probability = probability / denominator;
                    value.1 = normalized_probability;
                    break;
                }
            }
        }
    }

    values.sort_by(confidence_values_comparator);
}

fn update_confidence_values(
    values: &mut Vec<(Language, f64)>,
    language: Language,
    probability: f64,
) {
    for value in values {
        if value.0 == language {
            value.1 = probability;
            break;
        }
    }
}

fn merge_adjacent_results(
    results: &mut Vec<DetectionResult>,
    mergeable_result_indices: &mut Vec<usize>,
) {
    mergeable_result_indices.sort();
    mergeable_result_indices.reverse();

    for idx in mergeable_result_indices {
        let i = *idx;

        if i == 0 {
            results[i + 1] = DetectionResult {
                start_index: results[i].start_index,
                end_index: results[i + 1].end_index,
                word_count: results[i].word_count + results[i + 1].word_count,
                language: results[i + 1].language,
            };
        } else {
            results[i - 1] = DetectionResult {
                start_index: results[i - 1].start_index,
                end_index: results[i].end_index,
                word_count: results[i - 1].word_count + results[i].word_count,
                language: results[i - 1].language,
            };
        }

        results.remove(i);

        if results.len() == 1 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use rstest::*;
    use std::sync::OnceLock;

    use crate::builder::LanguageDetectorBuilder;
    use crate::language::Language::*;
    use crate::ngram::NgramRef;

    use super::*;

    // ##############################
    // INPUT STRINGS
    // ##############################

    const VERY_LARGE_INPUT_TEXT: &str = "!WA 2!S37c4TXX5HIYfBKDcJBQe1B5xRPSKiTiPiEsszlldWhIuMVma1tlFiwaSK7AbID9UasIk2j2kUUQ51jHPnNM2MeFysCDDBACzs9PUo9yhDVT3E6TnNjS96onnTjHUXj1Xn5g5 kXPv3 z2DXcqqkWhowjxXpBkU7mZ m7m VN 5FeoCDwXd3sWwARUjQltDzo npBk50NmJPUrN6z1npOOO4WIT2syJ6sRNlVPE2Skz6uvlBgtLCVIXnpIAHCzum1YnUuFqfK2T0ESKNusoxo9c5sRySPU7sQlnldLSshiR PHYtafKXO5UbaznHIPYUSK6QqUXvGg3Jj0fYzT2RuxMYJRNBSSAJRMxkHFJ6Io2yANrQzPI9h03dDWesp4dkft3Yck6iYwQsrpn07n0PSv gVWa7wk6HgziPbIg UnK6QhN XsQHtdGVfn99KzSm2VRrJn2PEbZ8s6JjLiVSPLuVkznumNcGYiAWavYOU7PG8jXVun8lnp sJ1nIS54k5vYiLa gZAjfW4g7 mgz1T0oLILKk8bizLw3urQHXn1ly0OrDUflnKzkObT0QXwDMbGpWsH3CP0nHV3y2RjgRnwwTZEwzZmsJORNnVMH5zgASXSuYlmDE90NsX0stp3weESptkEdvnN0PCtLXHYgzsdL4SHY08AKq7SkR5c8 Mx132m8N4iacccIZkxiVQU5qg5biyv7uS2JR3BX31nMPCA2RJe3cM3YxBmTCA5Rn1yW yPw75YBQn 4W4AB700 pFSPZOKQWyJHdfZE7U H75q9hlpoUKlykh4Ci2s2 Yyv7CwfsPCkLC5tuax Nz0oJMyKrtms04JeRa0jPmYkpPIPB1g2ubQw8ed3D 9hN9Mu9QiNnVA8CYtOaJ7c5Shc1oJCU0Wx0W6AqL6S7bhP74xe tfZCYzpmFw8rIBLwoRI4uY50Mqg eJqK6HSRy50ZPCbfzlLe5bIGXZRETZMbg4yngf mmTuGPMmwNdRd rs6i2eYA5G2s6GS3RLC7K7GSp4pRV83uVwMaa M9kt3zgxXyDjEGcYa6tpfYMv6iQA5vMbMXNa7B58YIxK D0d kSRsLOZ4D39G2RG9LdwofduqlZmzmJ2A2S3Fk1E6jwwD5m9eZcw43u8jkayr1gdMZtVjYbRnE6SYww4FLcOeHzq8pNgMGJZWEG5qlLSJXM2iHNg m144eZOLBmDt 8uTPGcgtB8ARl9tkC3NFheXJBCJE4ua0SdBcZPzLgcWyXrJX2kT4bKnpjsjEenGxXnx5Y6hzoKd5eSRsRaBYGZUP5sHutvrfDblPoY6tHmk6RRPo6deQR7 SHpzeYg69fK LNSg4hYM79fI l9ry zisyYnWivcuWmRX1RMpVH1E3ZEC4Z42 9eKCZaAuTK6RLCl1s2 22j5wFmYoi7eWeAG0yTKBJSBslK9qAI0mPvIFsGAjbRLe5riTZW aCfeLXbd6oN3BQV83yAZu4AjdwlP3Ei9HROK7M97 z Ea2V95YQpLfWimTskTjm0nZtAtKSjvYDrIs2czRKTTgY1 2szPmba24 BP1wcq2iYVbqDjVfYqW8qmrsNKUszayTasY jDlsIxl5gRL0Zmnyo8Hh5SDCSXgBqYW1sUNnndGZKwPBKu1kj5Mm9hS9WD4V1ZVNPrESjyJJBOgYrjhJitEltNUGvE9jgr5m5n0gdKVGiBn0slT0yTsWpz1b6vj5jabm53hGMQKoFdT1KBXMk5lyMtkAQuMkNstoVs iR2blmrkycQH865y vd823OKUPuRnwRsUmW68D9qtmYDvtnc468SPbgVz0pDUeNwZqjPXAloOoKrgaSPkW hYXj3Bm8Rijz7dcRYNFtM2m2sfiuRHcfCkKBhJX02itmlq5KwvXkHQ Phk3fSy tu4d8ukiQWmfSugnpNJV60iZtEfjgZIqyuDGRJCEDZ633 qePlIvNZKTtKEw9cyFX6ETmwBlvWoIeYVFSwDFgG7Dc 4kjjW2IhRGPvEgM3om7nA8bqEZyxqodBmy5u1g32we TV95G3DauyWOPXznve9DU06tKslNYa8VOPXVL 1pTcawaXEg 01OQAzuIJYqZa43ZKrZ6 zRNNsjGC2SgRtM93JW5wpGEgLFNAaIGlEsffJOiVQ8Xronjx2ufZzLxl9jNSlB 2QKta0h3 Mi7AwGxhQbekIlxMXSzKokYSuKSRPWszVXwIj8D7iK0wu9ONsZslvwLKonvbyJtIoR9t6mrSIV4SJdSD6Xu5bkOKl9Kd wacPWKDzcWhlyg8xh1wY95WhqM7ZMQqEapLX0uEYPXx3pta0gz1 y21Fk8bxT3eVaUg58gvwHjyc2ydHruH5HCkwwIXWh5L3pS8zTowheJRgX53MXnLhqXhL9loN rngfxaTFsWyRZRcw2 bPJPClR DyvHAA4uZhEo2RDLYedvHZy9SxcAHOJsysFeTm5vJffE8ICGODgLmSx ig36cnmZQmUC6jhDSS6W0Wfyv4 bCrTXMeKKzQNhOzrI4ydo0GDBiTqqzc9mJc4mkj5lwynmACHQCMjbshT0JMx1ubidYMzYPz1kT7IZu2Tkx Pb9cHAzbkWAClleaHAKwd5MlAuhFEsvotXvEJ165LEuFJ92tdRKhXu24ChX p4ZboOR2T1JErbgzOHNL SthmR7WLP82qhAK 7BWUl wuzYMTBJd6BjJuNxE Ts DfjF863Hb5jts KKhN8FNmn5t9iKpTk5ZacrEcYVj5tqEyGK93I8ucKFB2Kf53H8zj 6cKpNa5PzFyKFpYmKpFsYxaAXZiq Fq dipBJK qbYZ50bMjYxixoGjZLVxM0TxIpOmqju5EkvSHgzKHgWT7UrE3Lsvb1B4dq698BevVa5jUZuwWFLvbKAEWnr2AYPqgO5a9sntoxw4FY6LTnJLnWkazJckOKrngyRLcizENMDfDGOhO74jJns0bV7KXznnGF Ts6BNKVuCqoB jbb4MC91cGcZnLr9Lva94 sD9wTLV7rI 5yCYHLQKtNrdQzAqhWjbyLAIjZdSdihmc5 DYPsRcCIb2lkjHpotDT08paw3RoLXPmkQGO5yCBaCb zbvKvzcmShjjJbMjGFo1oht2KB3QsUpY Ll4itfv5v9CopPIkjSfYjzQvWuGi2XoCWmbZ0AMwpySZQRpbQVsqsNtDOb7BKEIhDGUbjhfviEyMkiQXLZAOklgdzFK0ydoqhj2bttLbK mQKrjjlQitcWcfl58OWbfvqHM4Ng5T8bmQdg7DA3CqOuCgFG41cl4OswkQXlsydkkExPVTFRhUMA8Hs3qlvqdMsfiyBHBVnY9Sts6KgBrB8CGTH22vXudQBttDtqxiulXEsXfw6BQm6JY t14wm7N7OpenbM2vor0mzGgFIeQWxBMbuYOjpkd 9eSAEI 1nhx KJoOo8tl6Jp 5ayXkjjP nnn8aOaILmmTtULDsEL6RTwY3v98nrueyADTHV2phzmdIRYwCvP0a9PvKpzuuPIw6ZYE8pKHcubP8Qk5Cu2cuFI1aNwLtpVeQkcGPFQwantYJQzv6lUdPdOKhT4gavZs BjmdGmaG uAL3xXk7wJeqN2PQC b4Fxnu 70mFBpu98kGdiMoD9ZuMJOMKOrUFuxnYjbf1UjqrTNXewLheqFujfyQkfmsiMcrOk5O65Gw6NojtjDq5CuT9DqMeuIbA6 k7rNw1HX6zA1X1nAiBA96UZz8826AfbccDeEi0bq3iJizGI1k5mo6ubAcrExKFGkmy2QD sEpIO9bpQkJmzoUQm7fuLH8yN gawtKFdqThGtB6s5bwbHWPDfxxc3XN6ADycI8jFgVCJT5TxIqcqFDGTjWRezzI8irUZaVuGD8Z6h5w PvhJ8lIa78KFjoCFVjjVpW I3Vi55boOpb5Vij5p3lVRxmj5lVjoBjcqN vsAZn7cQeQQTvmiZqYFg xadfgxJ gvYxnjz4dUZ4GEOPpzpKBvSEDJ6UJ4NsoBbL6cEUtjBQHK ZPCgdnUIGmx9aC a2EwGTzVEKPz0km76JnmqetE1LaTxNzvKnLAM9kK2BwoWq4NtjBQbauyfZZzyIYbaM4V6oj i3rj51Rli5ht pi N444GIW8ev4TySwqPJwc0sr1ik2gu5 xVUafBueYLOc1aJ4Wv9ioGNrlvWhL1PEhy8oX kQt8x5obnXdzZdwl nvrBijFno 9s4npc5Rls hssEPpaAPzpZmrNMhCYj7184zgG8Tux2geWy4Ut4ZiyjFgdbFgUS6ob7t5empyADIOAM9QBMd iiFpvBd6 2P8hmCBrAhSTfyWtEzgLYpZXonYsHDkYTQW2iVwvYd9LTR fXkGC5Cyo VM0PLqt xagNyZ NR7oxzSbj 7vp3oYp8kpEBlihdh6sQGqfjKxiomuH1ufSuOcxZsJ9av4AxYK6hQmsDnTd39izJ2rU2gBftQxBPeSDukbBZG 75ZccCHLyBNpr2mdf3qYwBUrwqdHbsGwzKay 8yMGkwOH0jNd 9O5DS3EnxK9mZzUmXlK oMv8JYvLog62eMf4JIJdRnY0BM9C6IgYZAy8HbnSZoDXYoyV7 p9x9x9fQPMg6D V0R9Ax6sx6I9U V3F xVMAQ5AebT MdD YOG29m4bkBIEIePvp AzERPZSG67WRDiWyuVifNd5iqaIuKaHkNo Fd0jXM80Lo zCPJRc6EG41O fEITOr3af qK NXqBnnwBpMANv6EkiNbDfU0iJWi53gJKFftPDohZz3ksZnBjwVkoxPEIayKSLyOe5Lnk50f9GXJEDg7CHNjkXjgNZXKEYmgxFfBJJtvm2 cdYI uaiXgUZhOHuN5WDnIvAYgeqYrtBNKeR U7zKI0MhgjF5U61MObiMNXtV4B2s8nXnuZmL7wcUlGr7G29cp6MVluoFZuH3X8OlPc 8Lr4rfwBsQW6wdvOoYFhvy9uHnqf24gPcBsLkSzQWwOcB9GU0fEizMnE8rV0L siLcvyBfPo xPcxpNOWwYeIqppYbgkpsSaeefLoTufeceeuHBWbFNkCJuHBA1e9MkCZmeBQGe0r3cIbtfQFzJ sf2 JEDuHBfMz3bzgQWojpmvyxicLv2uhB8 Fa NDivBekQqJuHBRiM0rQeMevy3GWIMUHAOcnxkYcvOLjpFxm OfWEFOy FYFpUeS3426b ymULARnHIIuug ujaTIVvpaYyETCJBvlyQ7UMbm0pTPgtJlyoj1eGgEfMG8sRXgTIC85Vzo5J0AB9PeSJ23JFgM25xhdf7cmgWoEkZbHBTjjFVKC8nWWAoc46EBZzZ73X9VOSpo3F7TTL8k aDrV95PsDrDrhw 0kMS SeTsnubE3OoP8HZrDhoBkjhF 7pk Z7xe R3n ZNcw 9firphLu y2Kkao RaZ RpgkNGnQxl37a GhyESmaozdOKSLBUNd93jU59A5Ma321 gmrFD1nhTDo3g0tw5XTak3Ep MiVGnUPX6r5AC GGgDhxro7eQZ4XtyXceS9GK pvj XOlQ6T7O93DC0NtOpM6lJLrlJhkyWMKGF3k 7aT3KuGw7aFtGWbWFhcFFGaS30QFGUzghxubnp5SA5SYdgWYfcdeuRJ5bl07Bouq F7zZY YSF xI ILnQphlMguYQq RktRnob6Fn BJ1X E92NjrIW5CXHDMLqcYuJMhnqTUmZ6ojIO4 rTM gQCAiMBCd0DoBhzH0omIrgpYPCrwbEL7Nm2fRT0Fy0yOo7CMaB dIAbCZKc3wQGDeU92cKYF4aT7Vd0ZFjYlN KwQKhuDAM7ntJAjK8CoB9DD NITvu4 ahdnRazrAwihBF4jFJuwSMxxqMEmYPS0ntHEZijj32mw7bYQnbgJfo7 kjW2mQJHHlHEREVsju1uYMb4GYN WTasBmqSaggaQetIHlVz MqHmvroP2gp4SRt409mwOR 5Bs3C8rLJVzQ o3hPXTTDNDWAyfZdbWpj5hb208qK3fdXOa7npGkYK riNZOo79 JVvv9RpUwA0jAVh2gwT81r9xamGr0LD Z37DTRl9JE3x6s viJBobxapeCuH9pTnrORdi7zg1dkFw TFQdBEQdfN1I63HX02n1TAmHamoYF7TCDV4ts vvT5cdSGRtI1o 1VJCGfv8EWMNl5L69UU l9JF4AQ5wiJY2i4wbt q 5RHHq63gHmjj26zS2bRYMYfhhLMbAW 1ekH9o6tuY ysqoX5VvI471yDrZoUsE1DjBcmXLZOmXKZvAm6mxVYym3apEyz4d6IXlKAsabAmfPH11ZAzS1emvoMqh4 hRGPMCwPdW2xplJTnIYegzvW2cgelDqa8GPUMGn35V4iWlIE2cMkaylKnVXgWxmO jvlKrc4NQKdJEhm8aa6Rch7OHNy4GhI5GABzg8ibaeDuF5IoSJZhNaDApbdzBmYTWShzjhoaLfqj35LpetiNOB7abyEBlGDObumqaywBwHT NmAQGWcU qffIFnosWbz8DlY41GMH3ZmhnsS2V 4gPd219WwJrB o N2Yq28Km9D3i6BKn8 Fo7jVzzLeh3ooGPIa54uHjbelw8a4zFZMgifTJakvBB pxkz7qwcTKfWxVaduCbGhtWwLjqBOb8Yoh5rtW9NbuzYHH9Mi z2S Pcha4o3l8VDhdfZySwOFJMbvjwjJsMb0YPIMJBIi2a Y5CIolExpc TgnuumogzAGYhXKnpXH6R5Ip1CcSbKVHT1VOHV9mtRdhSJ03DNhPXohK8TndfUT2d5pK3VQJkqokv4mW091qutASUsM7y k XrpfWI GxeqeEfQqssRuHJeaxDDud9DtEyhvrbunEOuzz4h3tLrzXsflSR6S3Odoy39Zd kPYkK9se58voJW156DRrBsI F8OgOlxVDz SKCUmf9aL9lTUx7 4 k1tvRkU7Q6C5LNOfjVzbf3374 nLSwfFPDfht25L2 eJlFLrlJskciNBYsfkSmPDDDP1VROJlTmyoUY72BM01LwpJaFdkrVzvI36D0m01 wip5Nmj5hY3yt2 DQ64FRpNa5z7OORXo wqi XnhJZhR8Uh6tNUh9HMT7082pp1Ky aa21pAxyOdns3Xh1E3v 8jzUuB74yh67NxCogQDSSWelIxhiAxDvoTO8zy7Lij MsXWGSo4ojND64AJLpLPImy OnE1wwZ33akj2bGgsf yBznxyxWJgRddRcduLdm6WCl8f ZGcNnQvEtq4rmgGyV zoMTyYPyCMSWWzjEFhO3rCfJX5Xvus7mSN7LTpX7JX2 WKxX4x2gV irhnrIrhP34Weq DxeRT03Vpx8rCViyQ83qGGTgIJ725OSTVOLOJbSBAa09Tm m78tzWSeuxEip99scA6dgG0Y8maPlfqNJSwCp5B pJoEHCbk6mFMPcgNzEe2aT278HQTw0qmhEcsA5KmK1mTAWT28qhK3iRgLYOxBjeTLYLOfxEf7JtQId VBsqwAC2ST6flglpUoEGlBBuQWzrjw2llAzYOKl GDFyW4I HKO0Sxse0YgKTrfm46QmJUPgWAKXCD6EhkEFhFObhjA OyTVAsWOZxgJtsBN3FBpbzxX1YzuipUBIDbgzMeaym0AtJ1oOUup2RmYz1YpjyX3yOIg cCsLjjNfWnrBksLLhBptXcrt0jGQOA48i r9Js2vkEeRrf DudmH8zA42 1 FDPyaXGbo2dmHwoq7QNgqwjN9CSi8sQVUOcFoHzyHELcZoZKyJNg1BNhRDINdeXMv2WsrfGXmST7M CsZC6dOyI7t8zn2blY0a9NLonU7eOkXwsPr2PaERLcFT1I17 3McuTxlppmy5bomleKLnnLNeypSvsFB887Nk8GLPhav4HCf tfEx2AK UDe2tfEyhX8uHhzvq4oyRiv49WKMtfEuQWVav4XOc ILjrUtZahT37 SQ6rgcKi7pK 2cfiev49wICyQW7BHK7sfE lGyxQWhGk8bTfHw7voIqdCLIauQWh6nB5NuHp8krCjvyQvdjLuHpY8Lpsf LPc kuHpQJmqGY72ajxx8EkGb9jirlrEtTtQGYUeB99aLmvcv90N80YNI UMoPRzFWa9t4kfJk8jTF9J7k0Ik8F3rcbvyApm9PcFko3xQWNgOl mZNBlv4jOc MZJd7d4yn4tsf TkYqLk8uiJuQWVnWNczGsf SuHFxGlg0LpnYPKk87rfMzJuHpp0FFbKRiv43hzQrfEggdrQW drwd dC5gsfEwKpiv4pmjVMphu8xuLvR i4v pvX688C2xuHxaDV5xIkCHRK4wfQ1QGBLMCJfApU JCKcbD5w12ptZTk4v5w9tRCRqT1nwR7PVWJU2IV1Vdu2CX4ia2EielW3RUbwJgwjNcECyyS5u1zLjwlJ9h3VHL2QVxDVzYQZ8QS6wIEDPAy1LA8WNr TI7pqaxxLe5NPz1f6QS6 PvwDhh4mDbGD2zrgwXlyX9OWN904RtKx3CcVoA4h1cYJRU46Ymht0JMPsQILo7HWg1tw8GS5Tr7g5yoGE28G5zX0lKndANi2jVV3m59zDvEFln18ceSk49nXGtEYtMWCWSnQHQ51AO2d26QUrPuH KRsXVIuUzQy6zMukbqQY0c5ibz6U4 sPgWZXEJSYo HYMza5SkNspRsU0mt08DVmhAEyDTmsmDHyVCe6BMe05Vkb9sJGUv vbb9HN4ut0XroPL bom5BNksRbdhOJvFDzEkxDzq 3HBff6yUlJAn7BFb7ayV0kt9gVTBF7R fBdJQVuTvMVQfEv8yzQf1Q6Uev96tTq6u5vVQ59jwXjHs0e7Q6AT65emM rS4eOl9cPymoVM1 1y8FzB8pQuLh1NUM3XBQodRWvzQU0Sqmq7vbt1roOwOdnMP6X0Z7AHO RQL0vA2fDXH1umtZ9KJSzglMvlpz O qrZLg3 nmT5WE7C3gBFBz7T1yM6alApwa9V9LzK3DAoPvEz0d4ksOjuSY 7FowzDBwWip3MklRcMmLSwZlN5nts tDvs LcPpgNNvbP 9F6tEKwV HZoP J5s6h8QK xX5sK46SiZMLREyeJVsZihHdROQGb5OuNQygBHvY pDJCeIzQlNrkrE9jm4VDRyWA0NPEom aNWqjdV2p j5UwbpXgyI1YMpWlu BM06N NMO13bgpM1x078DXsUmGYZ8J4A0tjRLfJ3tJBDbRZHYj7wRBexIxyGCZvQyVT g8KiNSRw AGaawUOXO5kmg8KRNgYWq3slVIn KrdL2BlsnxpMm4k2ZSOTXooqhj6ewhpBOGDGXb6nrYuAGGUT5Dgc4qzpbd8t aan6QraGgada0Wfda0RfJ8t7yafKfeVwmmqXqafdfuEGF2ZmNAIoJ2E7hU1OHZD BpvblfZrNuG VNrabWy WJs6fht3uXBq707 FMnTjPAQzR9U VY6wh8hHefn2qXC0qm3G8rDglmtOXd8YKGOiF1WMGJGKTVnY3xfpQUiz7zUMzkAxP8KPkKcWR4rOK33hJ9 HYbLmViuYPv1oFaXsoGle0yOnloizT3ttXpTE0YQyvomXcSpo5vSSo5LLGLnSuYuP 0lik2AwQxetLJlWhbJeRIFeoTSyJsRzMoRYP0SkiNTm4Xl6WSIgci Mp4824ARTzGBaEy zhNNeWWJpAK6Kfe s8OVsI 5I vZnxR2ysvEfLhYyfn9Vs1cXKGbL9HEErRgJlfBkUE7 S ADT5FXGDvd265EeOK5GyAJFeTXTqFC0vSbyVsUqgnDPXa0f7p SyNK9 bHLkzAOE7MuFJf 05Ponwt2Nud1xmQ Cod8NaA0PHgDc7ZaVZXn6erNyYtCIDKbJ p7Zeooe02N MKc7e8EL 55IlFVAyy7HD6E5N5Vg08CQYHzU5p0HpXM7eyoS0g7oT6eorT4jGNS0sl g2iVsdSv3oXgriQLL245KIYwYTj2lfJVICb2NK kH3xzog1ZZtfCSVQb2bwcWMHhj19yJbVXQ67IGlHUi6ePy2nYrzR i0sOp6stoR 4vpSdVuM lyHNPiNqB9WM2zSN63r65S t7SdKrhLfXV2Uy8YWBbbTXMDa18tgmF KxWPq8aufir0Jr2 Na0u5pbpsRoNcwudf7JYkphnKRw XDTS07W kOU5HuDpBS2NL2 L6alTU368tEq PgFMXlMwb556bQqQ6VZ9zSEpfumN6yFASOckg3P7CGDYETKPc73DV VpMVLrNf78Qa3hBogmC7CW gftDxp6MqvTIF4Z5iutDkxz0jVGxrtQZwIGuX5kvwU6YwFnV650JRErQKHF8nUtYpkc5wVHAi y7Z3glp3qEDJZ4ofyv0b WNCl80Qi7l3c E9 8clsUC5KvYH3ojcK ZTTjmbp81lnJqGzdIxVEqDrwkHOhQV3gG7s9DDWh6RCgH0ZpDOSiknvPrdNtRZjre52MwrlBXykNDI AP0S8dOPVR 8BSXYafgt ycHLPe3cr9uwBGsJZNv2NKvE0aHwS5Bx oSK2rgPDyvFtEGCtUdyVVTXYNIDb7cgGgl64ZEHH67NJ67DalnXam08vC1juLxDU7RU6SIwDQ4Qa13nuH1Gy d2sfPrm JxXlcMLokRiaC1PVkaWkcjSYF 17pyLrbp4vrbxjOGlPLGavAjG676VSn0FLB4nEzX5BTY48uF38LTpBD51N11XY7JSU2xE9xG2wUFI112Y8lmIB7y1ZZxidoL3U7WPDW48 XKbvV7 V7 UVhUMA6T39 TchE x6s nISKdLV1GLkii07 h Ro9UU0L NyPmQpdk13Oop5Mo70klu7KUffS8I2eWARl3tYgA1tdvYqOW3xtlUwYncDHDYbpTEgfVty4NV9m21Ho1CHwIc5zjcRiR5WW7rxT58 gaJZ6ar33w1pDgQAMoP 258MmQc8UG8rs5TRmkRQDKgSQhPZFKeyjnNeyjnNCzqTE9a BjSNbZTkwjulQVgkB28W82KIPgV1TnpIt amej yQWgCZXAiOxcFM xL m5713NXMrW7 d NwtnxZ5mDo0 oPROOKbpiFhS4jJ5Pd26oK6sEcWqYfEZLQqEjnxrtrDZuZ9U V nFZ3vn10wV7 Lz gE325OVNtK8d 5W93Qyk673h3tk2 vjqE2BHTpu3qn2JP4D2F3rJVWdhQp L2TFJwBfyhoxL62DuA3gUID7nWHiBR8SlS0DZZZ SbD5tM0ndv CILKlczGR8Cri8jbDkpRuJz2WN hCoDE6I3ABWG 1wXFUdrwsUc jIqQ0M6gJYsa9H8ZUMaIxCz8JQEr7mBaMLXWdETXg6S Odm8idfR ODE3r7QR gPVd3nUpbgRROx7Ds8bTDzZLM2106eXb1Hg1FRgNPL8JltUXJFkYUsLxFCWgCQWNdeH8znoWiQAwsNwlBwGTiUlqSe3b76gIBOTf8c58sygStAs9cMstWvqtofmfWFJQICgCpU8Tb2MRDwJ64PPepUs2I8sIqNt9TxOJvXoE4bKpjd429wzWZthYYpfsrhSlwL5BcqwLXYlb6CYQa ozGnc2S9i4M6KLTz5GoVUKwU0zlKbSjN3 5nA2lgSsySmnMeVn2UMXsAOb7 ysWeG Wnj5pqtsD0KuKMKabBnjfSjmk3y976T73nJFvCiaF0ws2jJQ8ZRd3JDsYMpd0jZ7a2FBYSvB7VDvz8MacJZCOx2ODVSr7g70oUBfG 4Xqqhkblp G3xuL6ctMxsXUlCtHAkytbAYVuNOdFWeVnGyWVCpsXYJVwhNkioBjUrCUs94xkUZ5IZ8Jg8vjYgHBIpcTBPD IBplE5LmHgMgUv5XwWfnm1hhtn 20cKVU4uPCYebXJ1F3dYYUntZU JyUnsykRSY5YJLtf2 m2uGPZIjReECjitUwsVVmv4aoX G3aDGkelwGWbBJhDt zj QVCTuHEXStfMN3CtSB8TT7pFNS0a3b3uxVvOT9NKmk1NW5sRpb7cfGVBZ BgRR4rH3H mYsi IXFl 7GKyVDYl9dyPqUEMrjyGepqOtEQWQDGBO9rPcxIk8KZgDWr6RZ hQZ7 i9LOBotMZWzYyNZ wBXHJZDcWTt9jkq99w8SvGGcNaBRzRRoGvBxaBRVHHbOv66fP UgvQVR1Zo8DyqELPTdnjFpvB5w8D9lPZ2RUmDA553OJxlFqwM3M5PsQVF 6ALvhxNts NR7oyWcbn2fCa2iMVA9JDl0vOZnP wlQBGRplP wh62rQV6SD4yTlB3fs9TfwAZVccOP 2JxrZ71RO5yjI2t3lbPG vJmYOx6sFtyf5B apaSICRfLcElvi 4 MUuqQVGvw h1xioAj1xyMepQViuFTXslw AN6JTF yqC47UTzhq91pWbG6BaQVbzb1KVHaOmmqOt9Dpvl9n1xCG6M6lXLL2MLIhDPQP gHtp Vt9DiQVdxcP4yJnzI oLU8eh7b8qks9DuQVJ5L2J674liXgm18SBDRGgo OE3 F6w2c8h jvGy7QBN v3o9v42P32v3o9RUD6xD70V62P M92P74NJAP E7ChL 9tAR9yvRV4fxwBY9RXvH0v9rqTqV7K980kegQRheeVby4Tr4F3eiyEZQRqnf9UlYuFBL86uFBBzSlUvZoeeA55lBQVBA5U abRCdLUIBxqO ULFgFpqO Q NY2ceqt4 Mp2h7fQPM27D FTV2x7sx6sV8LDlqO 2U9oEq9DRC3Ys9TJLGRJlFsBxvNNJxKDeG67NiUZN672QgFXt9T7LIJ0P AA57hDQVMxQ qN6RLLVdWlZ 6sC9n1xRvT11aI3x6d HF8AQ5J27 Z n namqTLLRTP cSu6Z 6 6 6V LU0l07 B B BbK9x8kC AVsSO 4UE4e9Sm13T71NWuF3r5 aL6BFE9zl13Do ASFVExL67UMFnI61ZOuFXS9ck1xNy96I6RB0vMuFaI3bO 6L6RVYDwj13bT9q5BAUJOsUA01Pe Wk4uIJf ak5NO7J1zOjx2oLO69ai139UID 3fFd15FV2vEo J67ex11F 0JR rvIkRXdOx44To0qfIF5PYImLSzAEEKfq5eVnJDvHdmf78uv bGQ spQuvQo8Zdf0 fti tpwHSz5BausmV8MwznfPAxIh9PzsRNTWe5sGqbVW3W7KH XmmlEnVc2UtcBALxgJNxbar CJBQLbtpf1hVRL jMAHpRuPmLZOvW6rVoVhAkebap4udbsNBtC60kOluydI J7EuQUG9fBelAMjBBnXSW7A oiDlgNLWCF7WIAJrcEwt1opG dD6PWpew Y TpxAD2Ths EKtNx3mv8OD13HsCot9tBpPGNplaFX9SzrfEDt5CAtyFz2IO0XGU90X6AOJmiYUzdcxe79ENmfoBHBx3oP sdKdPuDocBStWfYd58j4N6xGY7g3wPBuy3DXVfN4BuyoNuUGCUjLuGLgmMFKsZokF5reL8wE3jvVBF40SAXaG92iUnBqBFuahH1EjmljBXocJTmjzZVQ3tGSVA8EumXRsP54BA8bWvm8wbM8OoxwrpmWDGYD1nWUSJoaUBF E3 3HFDqXpRY3i7UU5reWOpN3 HkMlOXJVzVVi5fkv6ihLHVvEH6aO1bIEFavUd9UdXoJ 2dkf gzIXTrOEQWJgpHEwCnepxZ4rm19Gnx8uQ6CDV4CKMVergwZjJz8SlwWlHH8igrivksjlusun6uZwWe I3S0sbwKJJ3rkDFgGvGvdfAC zsL5fNtQaUMsUgk0KuhnUq59e 9sd76KSv2HY tu fWfi7DA fWtcLSJP6WHGj bt6C SbX0 DURUqbV35 4c1pS9n1l51C0cW E36rWt0RIXrlEicM uhd9KH8nuUkgw8nK358fK EdCFLEvzII653pw36Y33n2xxIt5CLxeaWeaQu7lil2bkgLY7C7Xe0 CoxXBjUIh0JIhmBNRhmNBNr2Tr9L9oETtzGzSLI9T8kxkb76rLDXr5UvrcVqPZKCofLTrgENIRWUy42UART2YXSq386n1g1KuUct0OhFiNoRSLf6uyecyW4Y8oiMW8ru5oXsznSr3L sBrfiy4r blfbXApBvzOu3FdL0Yg9s8WBY8ix4WlpFZmos0rpu XAprHunGdnCcHpY88n5bKSwvXuzKnIjRp3lgmpigqF5ydZo1ZO0a20gx4CxuPuNmhNAFFGrf2nZDkSZ4oEdVHQQGNN Bdpz oRDIF4X3hiSMDNWDsv 9neEJ9HsN8LIN6fWdhFO15K1iW0xC04bZoYjTYQXUzF 2BXxDFmV3xqF1KSlTQfwIe13KmRNoRJ1tVZQvyb13dU8eqq99qxgXbuFVRLVWaQV39Yssa13dxvCuP EeVCsFTLcN0lwroPuFNBvLLh137P641r99OvbhnQVFHs5or99yRUy3PxaS7H7Yy8qboUF5erwuSBNB7s2IjOzzPx3L8RTn8J5JYVE E1V4x8Butn7c JF8 XnFPlDPIx8R49oOZTsyB85s 19hEnzUPKo3jMBCJ87VJEDzRYUIAXBtWY4rZEpMNeQilA3wXMo53KMmCUZV5I2ttfsRc4A8nf5FHBrTdkPRv13jh3drlpn sMLHT1OZTUjhBeVSkIY23l8wWdru5xj a4uXkHD4PtAduBuCaHL312WIHYJDGNlItBfcoYIPqG3zILS aImAJYUPpJ69M 0XqqV8kqdhbEfVOnJLIwnwQ1MfVLpDUBn9CpFUmichUw T8jWIKDdDYUJp7UZUKd2LLU 8ManI9TjxzMG8J58FSZqeLB6qiCYJB3aBU05Azd1vrXeJ6Z8Qs6ogAYJIs8 fzkxSSYINZIjXd0abgT6huLX6NZUNtNXyy6Wjby1 MgX2bxTi2QT23yi3ETYj3O EVRaQSlwzQmQV33plqpKR92h GdnzuRuQli9W XIrpq99bykU8bDuC5 6sbj 4U5LlbYMV1ByjQOZ xBEZs1uZVsV7 VHfgyVOyLvW5jkZvW9UtCT4GBADjxAlisIZznHUEE1HTdY2KO og7PcgQKxpVC2f6wHgTugDoqULO rojakCXW2dz2kyaQk9T3I4JVrBfMCLnZPQQ7CaNOjqfYf4xjPJRKtXulDcvKen5SL84CwS PtDyKIrWmQOe3rxVcJqatQyTASfNYLCMlKAOpOtAePAUa7AJ1UlH5i4 KbBPwttfpQ3njXU6 AsQLwAXvgf7vyisuFN9gDKrhiXbgDGHgmbExRDSU7V 6zsLGgujJAMN5rUa2XEd7RGupxFx5msJJWMynUcEdW6ERVEmqCENZZefwy0ZdISMMxHPzbjFMfS8Yo6M7tksB7290Bo Q7er3Ox5WycbrQYu 8RNNtmAK2kT4hQs5Dw3WNUfOZzxswk2xmSEMI0Y0iOfqLTtZ KIIZ64e18NKAXuzcyghDvluNf5iRwHMAG2N7mhwHYlDsEbngSAUebkbZBK49DGd0D8ffxZojmmyHjAY5VtW87emAGMmpZTNiLmAGCPjpxCvayy0qjaOciHUnVIJ fBDatKcvmRdBdswMvWzIUIRWEwgkDWxScZdz27Q Ux0L4fzjCrZtX8Lmef2Zhc 4L7J5nh0XQoNlxMFlaTr ra38gbztFaJB2gVCVbKQiVXlYprPhw2uJD xmANZ0AElpfzj5HFrXxw7dXvs6ja 9nzuxX7 6EvKZYUdSPcNijiaydOpEyBJcZt4Aw51sBHkzXz dYpWLMKt56RCsZZIL4 maOmjDZcLCdxgEXmCjVBYSlMS5cy8cA7Yc7bZTyIalLS5YxhQ4BA2IJdasQmvVpzsYl9w2aM1eXSihQa4Xzkac AwRcPkEyZof59lol AZ3 QnnaoC2f58KFjUe73BsY7JeL8lkE DXpSd7WFlgg368oNHEuWyc9mJIzCLKKp1JyinVAcTNxp Zcg34camBxR WSDZKV5LgnSyDCwLXLtp5OJLvxhMi CfZLMp7nsEQjnQFXARZGzNl84TuW KjD26vJDSya2WujTMfGdjAV5SgBFH7cpv p0jvKEhS9PLLKqFoHk8r42y35DJDVWvAEA juHdn6Wfp0OGQGFbQWNWO5f7BCEav046R4qWDj1odO RlGBq953O92AFXbrNWwmlr6uTfotQ6nfQI7OnBfp2G65uiFHP7BWbb2r93DpJGZG qCVCdeM8SxpltVksE M5 McTkOeNDnkZzxmJoI8ig7AXvNU4sMXTU410gfJzjSXox866UwIMfzCBlELlbXK7Hn0SBMZZy xRCxgCNU72OSukCpfXZ zEt868NFRKY2eYsxl4NEu90PlWpLIpHZPu02RbEQjWpeS5Y4MxWrTlAW301JF7o1oyLXmW7 K5OGnHNrgMj LvGPloUl1m YzipFt9lP1E28dBfVDvTg94eO7fG190FLZtEAYNN87 voNTkUO6NqzXLToRmEd7786GSx D 4FXx9t8HELzE V679t7usiwjFNp4V335d9KV8h P h d 4qjpepX3ABeRJVnR4Cg7SrwKI8BLemtoHAQdnrJhUJoIuWjj4UnNO37oQC07focgTE 4uCvhiVUalEn9gBGyKS9xCJ3iy5A6E3 F5h5J841uJpwyBk8WKVQQTtKp DvCldQLS5EVlZEfQb POPYg3q5zS1HhO74XJD0MJUh rylT7K8Qg3 IUFydkRFAv7DdZ k xQz2lz7k23Jn5Dl16MOSz2IN0jrz37 jFOp0N8sxA2E3 0 23Ofnf3Rr4 fjX8HDjHNWxg DeY 7Ky6Z RKed1eHVPTP3uvmZz7AbETor1YNbTEEa8IcDrSEgSgERF3pa3Y5PCNnCCJI3eO97anq gIdyK9LXOUJiFkM 86u3KLW11ZrYMkyhT1w7br9EW704rzxOhQO2lxK9i7YiEqS98WZkqTSQXGO65sBFNJyAFoROchjM WT02fDbvcqna9SQZ5byaTHkzxtXvVnR 4uFFsM3Ty6w5aXKtxS2XtPBz5N060C1UK6xF84mGBu3PT Ks7 jnJNHC8WrcfQDY7e36epJau5ovstXqWI6OnDXxtAM7dbLZaQXX cGVtujUsYI4GotOQtE0TQS3u 8EZjUfYGV3nHmo nzApDkuViQVNK673cuBA kiD9O4m3unEr9dyHMLQRylqlQNAsBz7nvtXOY6pYrAhO4YZusEa tGIVWa5IhAwpNW53g oK3MWOcIj90FWGYT wJ3U73TcD1TvAx9Kjp AViOj9jhLDOTrXskGCYzzVYnNvCb2JNX OiZRFXeVNQXALla2cGjq8r53l3jNJvN8QWVZPyzTgYUmb9sXxIBfXO2hwCvJ1ZQhWqmnllMZ6 PIMFya9fEK3VtA3V8NStqo29GBlR3FyE52GgiuNH9mFaTp d2hD9PzpK292N 8RL67FavpW33cyF8YW TvP ojP hxdIE8kuFFx4fVk13 e133dkgpem wFO2O bihbFxe4i Aqn NxjGRToQm4ipFV3vxV7 9N sx6sLLD4Lj73XJC2772g6D F9 ZAySRXlzGhFFLK017zM lCpwbc27a9nyb06Hrt5NNQQFPEVRZqQ5HH2XvzcZA1VvhhWYLlGsi wQmj6C5r7T0OzNDKsqPnmrnriWGTgqkq7vTuGe51n8icGNg5XJAZRwwhvx7ZVFO 3wKge8Y1aU7njXRftM7 Ag7yeLjmY6UtlS7bNekMgOtBAwkkdB38Kg3k8PKr 0tOdg1gRayjCwwswpxglV1ARDc8YyxoRjab9LwuVYzZYceSdlJb8GNIUHOzrTp2fM73td9 et6T0BEeyynO jvlKrcipuYzHPQspvyByfIEwGsaaDHSLmS3w8cPMuQFTXuKggKe4DenlE2f3tcf2DnGLD6B3F68NkyQo8F3S8voFBFkM 2NNYaUvFpbA1okgzIYxKOymqTUXCmpbUZmRTr7CXEFZl chOQZf7X93sKwcumvS 2w IlSPg2GaqhfVXg6PJe Zpup9H5YDqrP41ANj3VjJ9ueLNhmEWCjBp3oXH6R54YyyeA 2 a9whT9Z3bFN1uq3UlkaAZ IuHN6Xx7HrMfkZEkBFSWIWrvYZlII77vbJk1K8WK ZRYUteSV9dEA2LIWN7CdoCNsdgDGU53hcFb8 m4J6C3jWVCdEAMAiDroBkSU91f wtaRnEFk8fuhZj27pOrdpy6E6zSXI4pCR2zTKh0CVsa98UYZsLmaDIML0e6PmUlSMTgks7X6zaDyKIj4dSUtQxam5aDwMoup7cKy cDhoWjpVI1EGbKXDIqiC7PcgPDh8seaY2 TTjmUnskLcO4IIL0P4dmJUMF7L2TextLsbK2ySOAssvgDCjEb5W3nvyScZbRHC2PCaayA1tph2qmrdYp63KxAnUtqpLr9yNgSNoBR7yyvWK5clOoh0zy GMgBgRt0o6kqB JM90O5z8VnJT0kEP12Qedgb7iyB9Cm8 9XPKWlyjrwWsABblP9fSKoQujBflXFRvQOwpKLc71T1AKXYeK9Ddyh5LZLg CzLW HpH9GGHLFUnso7SLuryI9c40GZZoNneCA92m2gdBQ10kP7a4EMld0ZaRwTSzGUNv2yzsNkIYa6z0gttXTm8WHG3JS4t2NuF v7Xthpf oOt4uxuXRjjETRiETaT0AG3LctBrkMKFqsmQIeVr21 bRK5v7 eGk9GUJ1FT5kouBUzy4f5bhzjYiyV J8G5x6TRTLz7ISmBx4Lz7cTmBxWLz7cSmBNFw74OqxCytnDtT8tYivT2Rnwu9d7N8smrpfx PIa Y2PI3kvChuXDsf3fvSbQyJuXBJdlFUnoWkPXbxjnoKDJ5x2nGrgV0AOc r4fCdIQ 015hVuBCUJBszPHQbXUmC6BJlabdv8wCiwUjoXI4crS843Apq9cex 0OCiWc3BMkypGcILySwojuBHuADagNXkxVu2mGlTq T4hl8ChqhKWuX WDgBBgRd7Pod5V7Gr3UFPl8CoSmCMPQZE9n4Jz VUt21ljz09YJ0wQyNI94shmTT6aMiRoGj8QdycT6aMGRoGjWQdy8xjWWrVsY5FWp IFwFU6qCCeZANEKPzR3tp6GXWLpGU75xpMV1DVgOKgZuFcjUwBTfPTWmzUdHBeKTKzpaR55dS gR4vov5vVVlt1X08xgMEEyf7iDM0JPCpfukOmVE2LM7PRFe 20WUZh A1jnNxQ0h0K9y0GbBTZo2EaNhJ2vW2cS9G2pY1aB7HSFSNqT3zSOBpSZJWlI0 2Jy iWPP1U6A7Tz yNr6iwKU2E7LmUoG fG5HZ4OsmpcuEBwXCn492kMRXkbmrwDat4vhWeA1bmbxDatGvhW4VsGzbuEi4sGXHNX0kNXHhGvnmo8u9vkJJkmNamocubghuX7KkUFQ4DrfJsfJrf7Kk2fvSBVe2bwicBBokvIWo4QpHnR3w5e2RaWez1bmHxDatOvhWeC1bmbwDaJ kbMfqX LeHDXX0QaHDrGvve2fR kMWE ZjaHDWLjHDWfGW2r3GkryhAvNWM3BRyc7vcyIS6aMWRoGj0QdycU6aMaRoGXFLaZcy09sHW2ZyALty7byvdHTNQVsjSRWCcqyhAzsyVq2X7OLFLiSdVQtyZ7TvmH9kbmrwDat4vhWeA1bmbxDatGvhW4VsGPCc7WlDcBpJPvoHThGvne2EQ kLWUcZjaHD4LjHD4fIW22E9kryhz1NWM1BRCc7vayIS6aMWRoGj0QdycU6aMaRoGXFLat5e2rwge2fhtRce2fbwvryxS6Ryc75pNae2rwMe2rwacBhpVvjc72w1jS592kMWELaMiRoGj8QdycT6aMGRoGjWQdy8xjWuoHDBlDcBpJPvoHThGvne2EQ kLWUcZjaHDBltc72wOTuZ2h6vIWU9vDcBEVTY3lTvayIS6aMWRoGj0QdycU6aMaRoGXFLat5e2TV0jS9mMwf2oTIaRQ2oTIvFfVDAZFobiSBFzsy7mpwHae4M4BTLNaeGk2u5bhGk55xdFl fDcoaybQEF2Hgaawv A2ZTg2J r4zmbwMAf b 6AFHJ8s877lMSKUx4cWqk2e6PSGHazS6RTwYMB6TYIpXIjZh 5rGNfBWarCNKxbRi8IT bXGXFDuHJuximoeTVMmrKoFoNbDQqsamSdHolJnmhVoDzFyg9XcAr7qzep KVYzERJ5EQcTpZ5A54H2uFzQWrpp6HgzObIg Uh9aDpYOdmuSe1ZoPHs1w2XNUiqyxNi68Bj3gSpa6T3eECs7ZPo5gtVHgBerD6C0bMeJd7HZwGDWR7SGPj01juKTWuFhclpdpExBF4gvYLrQ5MLgMLNrK0SShgswfmmWKjwgPmfYnUIEXtfjEHtmAts5msJ1ZqwZ7r72 m6GRwnkLrxQKdMk0KwybJO0 k 37DCRcPWla4DxFts1xFJ4OXuzcmViYYuMwLac7oxltX DI3LWwgz1Y3q93E9nveYnk9qZFm4mzdvQ0i9t6oTVdlMF9lEEOzXvgV BY7tbk5PGL8uOsEkCjpfPKNARKNAVIde7bpREfV4nQ8Gg X7qgM393AZqZV369x 91YW RMwfO8wfOAAvWYBvWQPvHkVvHQyRCoT2ENhcS0MhM39p9Ez hG 40 byZqm8j dJ4VzVpEa4XgR2jWQS7c4T7cuA3fOk7UGvF3f0B3fS0Ulyv2DHQ UlK3UluPDxOQS7cx9DxyVDx4s7UWvz3fP67UiE7UiL2DrQYURTQV7AZB31wPDxBvz31E13DT7T7AV0UR9fQ7kpPz48tXSesI0Y56gftA1qrHKr7QlUqYMarG5kmrkfZg0Y0ytvGRn DEdT4MkYwOPFcZVE tcLpbSiPMb 1Vu56jCr7tLTJQiQyEByDZRAKGBIRWZJqf qubfFK3dMBOjpQ6Y OqCCQOpmjemNk5E2jEWz ht6KmQASy2gSr8er07tqUGDkqAnKd 3NmP KVLRl5EsUoNC54xgpWLemLhrWSMVQ9b9WoLqEaNdPP3mJe uGANZcLjVnwkRQNzo6HYe4OkNm3bhPtE2i7LAKSfYjzhttmfgzFAmDZ6u4rzA3 iFJKKVgM8cHICtbv 3zz9j8msHN5JySemL40p MpvD 7P Hdiw WbH pe8 fPIsI1f48s1pc8 rG3FG2H 1p8 Tb Fe4 dd pu3rAxSo41haRIFX6W jGyDWlHYpauVda1 aq9oa88WWRdjwxBI1bfmmuWW4 g2Ut5dG6cy3 3WgPIVnQV EQ4AxdE Vx79gBOejUpPJOzPibueAxbpSJhUJwd7hg5y0qoR9apbW5zmLRSh1TfiI9O c2J6Hvm1YEf9GosBbdypOn2O9OUFTcs9KfpcWkYM5VcF4hjqzykhRWUYQN7k7rDOGD4mPVEheCvz0g3U0lK6k6P8ibcgUD7b fTpL6sDzQd2OFLnQshT3wqNj9n4mW7x7uy R4iY5vmVIE03wRT3QZOFT2JEWE7ePn1ZELmhMubATv bBZro067sXWu uWiFaqfStRa P5v6JEgxD OVh5Szb7gzOlwgGIPxHJ10r4WHTh8xOx5c5qpOICAUcFqhQnBu94huto34xHpA7WHD409RyzPyEf eC7b97i Hk Xv2S983QZq9IdOLdD92pfiKKZ0ZVR0gaHO5r0lKwf47DGcYMzaS6R05BeWHP36WehqNMtMso1Ks9QFwLR0h6Hc7mXZg6hqpBgGNDmfL8zvUsFOhQKH G6zYGCShrB8R8f04QCvD4qpHEUXtPzMr6EkOCLpgJRe 1IJ9rKZEsCAVhnl1ROvfpCq UskpkkPK8TuR4TwtNEU1AStmZF7T8wfhK8vvzP55fpHtW3ep7ZT7cMEjOIV1fjXsaJxChdRDHZOeuX3ov86OI CuX3bpxlqf 5zTCTLKyfOIRJkwhvC9uXnqfHjInrf3mvCluXTsf3gpHcqfVEwlVHsYDa4jAg3ZsCVnPI3S9ofdRMl8H 3ZzzEjCW VSh AQ4U xDUXRW2cVcpZ4uXMR8rbNk2YcvWEQ85dNk26IDYWPI xcNeCQii4eyfcmYbXqaFfGaf0pSJl j9LkUxQ4TtfVdQ4 UChd2QzF79xHVKEQMJLkv8auXEPI9rfbIY7Mk2pvCaQ4Gv1XoKkoevCyQ49qfJ gZzYJkMOAodEuXrOIhIkEyQ4rOIaxTJrfpov8ERQZyfv8euX7Jk FHko6BmhajQyYQ5ahrfLPIPOIPPIzOIa1WyuXXPIQv1bkHkQrfVFQ4jPIzFJ50wqfNOAoDfuXCurDQObv8bOIMurlQyEQyHQk65PINIkEAQ4zOIt gtOLtfpB1ek5uX3jv8bPIpev8DrfF3uXhMk iuXZvvHkmv89qfFuQ4VaffG grC0sf fRM4MLkEEQ4Vev89sfFFuX3pv8dqf GuX RvvCrsf quXpmvCkQ4h5nMGgKk lxnbjiv8xHk rPI 3OI QuXpgveeD9Rtf nQQGaJk XPIFcQ4NKk 4 uBesTdmcPYO0EWwBDbt6CluasrfNEvj8OOIFQfmSOOIF65hkuuXpdv8jW9LKk B Ze7d5xTA2hYjKpWrN0 adEwLGR49H8hw59HC9xjBOsBHd76ghJn7SHhYzKZjlflRUEMXkyoP9hW165dynxH8bmtBH7isKBVTWHy7l51IFgxVKe7dP5OD119j1LYyAzr XBOzQL k71I2d46sn7LIbvLZM9kCJDXjFNT6Bo7XDm9CktiF6FsnS J9UA2nUrocdK4haj51761olaZMdoalma5qoCgEuJ0OFYA5rzMzTGtUKEi7HKA6HnttsjrD0pbb wa2t6rqpcoNYnJfia 4cdKZbqhZMaKU5WUP0yHeNy4GAw0VbFDHIv1S 6V6 jy76y2z5WnM9OZJkiWg3TTKZ1o9NDJC9HX5f8Em0PvX myFkmBlJ VAleJjgG8ru2Cqd6oMMUQHPTfLLMHngnlophZiXWMoeEHYUY5EFX2vBw Picb4qUJSG4B7FwuX8vJ64UwMoYXw zhi2Y98l9LziZxsJdG9mJ5vgB0UXG54fm7Gr5GVnKxDy YdjOWcHhJIaEl owkor SHyus8CKq9a1vYHFzC bWwMR Px 3D4i XhGtGFQEdfd39A4VFsgb4sGG3oI3psc nUo4RQvYG pECRb Nqlr8B8c8pBDv349INrJfDLqNM86 A0B P fntoAMC0m5OzYrZKJMjhntoAMC0m5SYYKZdem5SWjBV5IbstFZhD6B tVp03m mu43q LJB1 Wpy6B 73o9n3hYQz1Y0PvZaeEO8TnHYzuIXwOuXsT3yxmlGc tHCyWPpO umM BGNHWlEVtNYa 4NwCiuZL 1hIndojoJYmoatqjbSImOFeuc8pDJTlYMDkk3pYytMpkb2 tEf47OeP7N0dHiHvRtxS4rH4hnf0S6Sa Q3r0gsTOBSv14Jl8bESVDhtPkJF5bcYJpQe0u5SaYYkNVQNqugg9tWSWsyJ6oo2Q69RnQiln2pwqUWtPtaTg6Rc4TvFF G9NJcRSoOvUkKEY4 L9iOZfQrF 5PGg0wTCDLDME1 jtXSeeb8O286CJCxznCJ9t 9fXzX5ug4lNRQi RqNNfHyXfapG76jPB9IQI4JqtWGpm7izK QMlSu1 e85hYS7xYlg6x8O8seSHSB7UYc4xP ECGFbb6fT5axgRFZD fGhRURkyLNdR JN13btMuFnQPcFBe 1Nv6p6j4qH8J2taFOJAL RKUwt9Ur4KCgWV5QJTP8 BUAbFVvonadNL9FhXPIP58v KQHSTykp iBkB FBeB 5BoITQXQvpITfdoREkB 6NU Vz2zZog5FGyeGQzS96V 3TyMzAQS1uzRPYwtLTMkBnv2AQS1uzRPYgYuzVhIqYeJE9zicL9VNkBfVPccrhWVN97YB6puX0DnTPlU4 cMt silHtphpxZLE0s7vopa4pjcV2DTTxYkl3RKDmOT1kpVUQ5pl9cYDWjSsgSd1Un8jExdJF7Hd7 4vjW3E5315U0Gh 1v m3m6jisa0paY5GDzn heucA kYZ11soDM3UzsiMfrfVfVcGxtXZRLYPzxV eAIpbDQXHi zIhJcWJ4MQklkKFeLfc8zH1vfXFvL weTJpIc yT614TBmjalEC4c9Wb8GTLTY9NtxBpAcLHdm2mXhNbBseh61PLPYS U3aY8Yf6zPFcjomkhNGdao8vUzjBeLKLJHE5BfLaPRnkhJjhZRMSll 8ZjLWosBQUG1LJ4z1Zn2MHMHb Lc9SueDna2H3oD7UsavE1UwMwEQhFCvUWuE7qEfVbtiX5ygc6f9CBFnz8D9dGGKR4emWLXNLRTQiY68 WgX 5mMXMzPXmusoWRI09wG wrO5PXuOx62X8DHUybZKnh2o6R4p6LUfWvniVmlp2h2zlwEkZJVjKFyblEAmg4Ly4PlQ3ESc6dlsW i2jqx5c8kmklB6Ek0pkkgtcMJIdWGpFVLQzQpWHQJ6 3FQkspoQqyRb InoaWZZ9SvTsxRb muAkWroVvKvDK IQrw17ykd651osNW4YkiVhIeqbFy MyEikPGmf6yww8 fvyEmLTSaIQ0nuL H 5V cTXlOvRIwTkA1QOvRIwTkA1QOvRIwTkGxTkFHqTk22E2D2ztsiop6rIzO5JcWZl1EVWhDHH I3hVx45UHLtSSp06SYpyVx4FC7EVWdb9Gy8S6y7UKSw2IHof4 uGNP77A5399JOl2FBmuind2penpxNUTw2PfgCcMfYONMiO2ngNKd8JGRJ6i8GByIZ7vKeYknEcklc4tI0ZXszPWsn09m8zO5a 9a61XPXgNV3MHHm0jvUUATvopLuc8KNRPlG1gLI9ZzIh0DYj0zqpLHdDfrnFstVDr9NzjcvnaE4B3ivQC xOyEKFyLusaFO3sTput3Oj0pc7xWIZbEkFlBB1Kk3KKgHm6JcbUJx4522PBnOFBDxjJEmuIFe0Bo55O0XOW7FWg7iUya4ZWD7yTeUhYlTdFC3XRRIpYMviUWdgMYtAcao6nDKRY63CN aWlJ4Qww7xP8yPvGxpTxhBvX7VBf6gIE2kLt9YHOSZrIUoaps7zEn9hLIj12AqpuEobJod89CuIy8n35pcES624F9nH6r yEbqg4ZtAk79Tq 2tQnlv66JXmMW KeRJUvGlr1TLAbwZgFACg4tm49dvc66BsyfKsdXgFEcLod4z0n61 YNftW1Dy0R1an76A211SRRzxxZUUMDDn76A21xXzx3OHD9XryJNsNfveymy6uCsw8jynb7F0jy V9 qW 0TDslE zJCp28dMG9 5TtWUfO HKUUUwUlXV4UmusjWhHKa3kZx4ZLRZGiXWOdReSmEEkg4UPFxxB7L9SJdqSIIzGh2YN O0FugJXmeWrTvJTEEx5k0grqZqmJVoc0pGTjwE2oQv58ebQbE0MJxxj95d7HYraFSmT72yLlRTnod7ZQuRqpr6b4HU7Tb9brfjapjPTNYOLUI305emHYcmoGMmd48x41yB Fx9v7wfi4QvhfNZPBuJ7Eyuw C0 5r VyVRLECRBR Of1BMff2njgOiOaecinsiAe0ZRKcYea5rAgpXJMzQKg76 dMmGsIs6wD1Lc3hJTCQ7hM riB6IUl7lqtwz0LzZ 9 dqBpK8 wuZd7XaufsiXJeVKh JKhE45CUhsQa95 85 Gp2TCDikpQ93bVV1d1njJu97SzbVIwsELhtTmTPURbGVwKlTzpWFi2CazDGrVy1AbNbq yNAoUp9fkEO6wRCWMv J8a jgyDtBh6S5eLFz0IbrTGViDCFQdXv9bE4z8Y qXXALGqdr5XA9sfRKuai Osr53u6S4pVy9Yjfa6bE1xayPsbxRpV5x9JkowRujGl5bIoAQj6P6ggQFiXXoEr8ys81I9zyRH0XkVMjfsiPgGT9Ccw5duRisQ xkpwZVUG1kvQgetfNyPpdBG23vX99x9Y1IOFThQ5sOT5hYmkp4RLa862oS7W0 g4HeLh8bwb WqWl68bPik 7OIw9Gq6lsdW7mZPpZGEL3Lv wk8dD0TjMQ zXfh1bl1 tcudl7yPtvDUfWAyFULpUl11yc783IPfiQPRRfXu5HFGvLFsC4Vp2Wzgw99Xz vG 2yZHyv3vNq FWzD5hfh 9CNP8Cpf1Qeg9ZHwVblvTWCSIZ6uUuHWt VOG rulkv5XELq2MiyVl1GAow9rVM0E pSLUUYREt0tyzMsnUwJjOEuhYmLxVGW76YtNoHyiE9eRdECjY5y4Bfq3BtMP86huRsHarKcK3KGDMSwOkSMw5WBD2 2MOOFtu0Vjk63ef9BII nrr MOOFtu0Vjk6xZJI pnEu034vUwe0tXUgoBcG p4bq nja6NO3EKtPjLE J2 9oa6 3YdG FNQhi6LlNiiUpKSyMIB2xnTQH3WbpWd385xQ BWHH9axC8WGiq3aBpbTVfzXl0nhR8nGAj78pDwFlIH9uxdvh7Hxdm337eaAxQPTJL7ahLh6bU6Wh6TmiZiMoydv GflmIJ9qEV1GOiTqbnFHJV5EFKAZDupsO8C g28Z7VgooqVmw5pPlleCyf wyU4FYLD9lUwC6wwl8uI8 7AHhm1GT7TEk1YH8sLFgzq8OVR1cBqecApxvFlaaOUAKoCbS7nXAnGxPsHWfn nnjoutSdJzXRvO4bTw9iXjytGrNAQpaDGHHRTAFVVXcM0c18FpK Ci0NHhR4QjwTyenvyxwNaWzBAK1G3Rr9sbx2OZV JXwdzxeTRhC65ReC0x wWqOTpuh0vP7ClQRJYZUezhV4AyaC4aJoFtV8cfpOyRvTqLG33 9z3YEOUStn3Nqik oVOqelr 4UdWgK m1BpuN4ckFP66EvRwHGtB532DILozkBrPNnHymxXbE1cbXb5d6odByG8pqxb hLFUQ r7 p3ddAVqMIoKsv9T2aBAAW Y4 NcgnJ7BaMp7Q6xzWC56fYhCY1XCblIF4UQ4YwkvOu4TZi1a0ye6V7INBO6 iQUwLk1dmD R2eAdBcTHnH2WMqBytOnSj0g2eAdBcTHnH2W 7eAdDC7Vam3DebDcLAyVj0g GhAd 4nj0go4v XNvC23uTzBZ37qB4 ip0g TkUhckwj4oW6t9pG6ig6dTCTTXODNOBy8c1 oJN9E9kwQyYOf4j0P6y0JmWVImHshP4wExUwLKxfFTSMobB5TRvVKOBQ8nHYjoLjFYcl80Pk wWcerhErLWzxe3Egnj2ow0fQU7MQxRwW ykk83t3IFDQntX9ww4wz6NBsgPZ2YQw4XtWk 1wsnFNX 29mWZiGOsSPjr53etv1c6W EoWh7AyquDhWNxlyBs W9yrb2ilQTJ3Yte8LkVtHRRvxwdOfiGFI9eSY gCQwPuWlF6tES 4jH87OolUe7OR6IjR77L4F6aZrU2KHRdVaNQHhKfV5EpxHX6BU3E 3UqRdR6d69wo8nJ5D2S8nUJhtSmXUkV0WcfsEUYWUnQW2Rd3aFIyhqSRX9bV4ynzGb1DOQFORLVQKZLLD1z4FnyHiwdi8oBoSbW1eyi74Zu1JMqTI 2R x7oGAnyI6 A G 8GR4U7nw51R13s6FIB456XjGtT8HhmPpW7IbynsJC ab779KeoHGhE7EDWmxu9f Y 5Ox l 1JCnh4OtnrPZSfc yqhaYyNX iQfAA2lr6MO5yDl70yxl8feZlZnh0eDrBYS el7mOHuep Sp6JqxMBg2AkOQNZeIz4tyFMH2bnnifI5WTI xlIJRLjKjmysAkM q3BVQnZ34S2FbQXP9og1UrNNCol6iSy3QZIA5lr zRu5YndbG92ugrwzFoIAsaASfadHJ8w2rQpK53upVO YF NS7fa3IdOzinxhYy 1JyJHdPMTj224XKtSaBtFz6uTFEVhFWUmuWarQmBf0 zna6iouVFxh3 PtbFPd2Cmz3jKbt3UyU8qwfJddOMJ0hd5nOKIuYztCyX6QDijNr7Bdz87EDKAHruwA8E3ZkCo8D X70xKfn2vFyMGrd2OToKJczrLZNfvi2WQ3qlReOD2bDQPUZ NEqZ0PmHLcMBGDiWyGEgXqU34OJLljySvY48iSHnpdzf4Rym8Ljjz8YvPy5YkiBwCUgRVANY5zScjjE5RRyvKwSkxxX0KwmTRRyvLwSQxxXQjTy1UUIvxAXQFDfRqEPLRqE5durYxeMq2mo7emHFxalXzPWavUKKItsgbYII8qtYlD4p67ETOYFT3Jfml3b0P0DeDkFhrNk3r0r7oIovVJOtT3F6eHPYEozGoOm6RWmwWJyEulGdRyfo zQEno6jP8NgxRWsPviF5LsROwTLsttBPKQwsc14JCPECRgh2Qta9QMxsQAsZAfnPPwvwQALLMxrJSzJ2n2pAYfLLCLQvLLCH8fLNCj50EfjxwEYvKLSM0AutkL1KtyT6Ys2RZFjuutEY1LtA5ertEFT82sr5nMIAvKNSM8KLoGvvkqQMehVERWzp7taDxSDygWDqZyIY5fE7TrD4s0Tryu3EDo4O9rVq3zcBXDKFbrqjz gw7FbOQ3a2bhPJXcKUnOUyou015DO2owGYkPTb1Rh4KUwwuueaArLotMmC93sHyAHfhQmvT6Tyo5wcuwXq8YXuS5Z7c0nQoAFEueEG9bM7BrDNN WwxCsJo9E f72OxR9pUZZV4GJ6cZZ3TvgHwP 8IDFCJEP4GMxGuld8t4DnEcNsaDVkw0XKdq4mSt6uFM92FMDQSC1R2Z1WGv20PeDXPYKjrvis zyth7E02WYSlxncTiTOgmiWvAJ1k7a 2UHTT ytYWq6ZuSl1rhFuRu8M6viS9mO11nwQ8rXLsfgxpN1fDD1C0UepfCfkQZvXwaFqXubxfqV0RotffkV49lHi6m3KcwIxqgspWMJ1dbZQiwr7 VEIuMtK0xbvyeib6wACnIXvHlRjNZkDMKRRYioFPt1D4oBe4AW7Fca8k61IwcrA28I4nsisInfWohiuzk6RbMfaisEmySm7TqaziknhcndFyS7fIUx aBqND7fot0hCIHRDh UmSR7my8FrieZnqyIzxc2MAY82ejAnWmpJt vp9nDSMPG76wvK U6gd5LWRX96yI931Ia2PXk2XM8 6Hrfy4YtryuywUDYh0mtGBaUqQBa462ZT NB2VV1H8 Y4PUmNqlVyMfcm78 hKE3M2NTGb2WA4cHP5dJi2YC a YtElBOLLjU0EUHoiPC7uEMVkGOFwDOMUS5nP5jiEgaTaZvfnYmO Ihi54caIx3XeSnQpbsYawYkly55cGIiXRgNG0DYH ayTSR qr8NI6Z2xI8EQv6WeOoIz z8M9554mccrMSHbtQJ3aAqdURd7Sg2Ai6r8SkTz51QGCZDmbV2mxGoXsNIew1HHEfBItS2TySBolt7ifDWmy4HPo m9Zt2HXl1xHQlKIYGDCwTZyz4CFMkMOyPp3rhwYLrcy2BpDMqYTxD AKw mIHIyWH7UkrN2oCGT0t5dqXsvMZo8AAsgdoIkAWCiLf5AZ2NywaUPZePfJ2rgzQjgzwcqan3x4xi0LydxqZhlGuJGPSI6 ylqghIoNnFFPclg3MgDsu9TJSs3X8IzlemXPeJl98mlriOXkBOBA0XTLke7t6LlX1pson W4seMvyzTch64XgCHHqZy65YlyH8BVQvyajitVnfYBDhEXw2VOlXWnv6JxMJljN2Q4cVXsmkUkjgTPd1hPt8KySvC2gGuRy2EImA7qMhLfdYzowAzssqOYbbWG67zrmNbKSmSjoNXk458H1mzx5Jz04kEmJ4zIor1HBQbt0jVLhmIW cVFa 7xI2JAb2a9d 9WoAdBsKsxmSFJN tTjI2b sSUbFt XNCYvBxmVmcXKpFh9HBo77LKJRDcVuEzjsVFhMWVz EI57XzdlLGn8GrOgbmGjy48wKkr3n7FppMACXNrCfvE5RQYHHFWCCM6v bxI77my8iotlsE 5pA1RJZHC8opbMVnvKi5SiCe8VIPMbA0T0ZVw9hV16rkT0CLJ9Z8Y6yUYYiNZtg33QsJncguXRgGs74zskpJ1Q5g3EweqZsi4xV DAPW41AK1a9wBIv5jxZm6AHBDfqNrgHbuR4HmrxR8HE8B74zdwGiWS83b2wrewdqYtMZqsF3XcG9uDl84MKl7rPgxatUAIDWjeXMfLkEcPYeRFXZeTUeNoy9H9WwJjobwg6ztyw0ZaEJgazSZ1Da2S1LGvV2Pt5 EoVOXrhD8PhTBlWenEpBCRjzvxC D4oFimyEsOWZv5hl2aFut6w2fzIoctHl3UDoxl0slpo34EfjQtzoJ94Z9Fl3xx288xrcMh 5 55JZh5oJ73dVsYgYxwIGNLvuDt2LTYoSIdLx4wsVMbFv1uCqWLmKuM6k8mJhnZe6uQvmJnoJHxXYEfRBMzRRA65nerEVMJ7cmjwwoZnsF54CP2WpdDmKCNrF8HTqI1jAZEB8IVEI34 2OdB1OzRoWx6b9mhaOd 6J5aY7hnfUHATZ1TcSkCLxwKnZ3xxJ 8Ud9Y2wXTAOTioiUk3i U4b5BIZJfDazUz iwKzPUdoEdYN zNaI o5UZyK1ESm5JFz8 dKvYGw2WxGJF BMeP5bXOj71x4XnKDfsixPtTfgKgmZ6MVEh00Gf5s 2EY9ShF8fIGWjzuOejwEswQLIGhGY e6BzXCeRUK9QQsVKp9zspyZG2LDQ6it2bRUrZe1ltRsk0ezlAqrILhFpG8PN0SrVwx0U13AQBK6MIyO sILJG4C LaqdTBa3 x27Rp444OkX9SBwXzfcX2XFfNVwKJTLtKK1UR0UscCswjTYsj6RSATDIDmsJ0os7ATANj7SR Wi7JKqzmWXXr4R7acHlqcFCaj4ecxiCWroGRU7 NUOQJAokU7ibUKchGRkUQ4i859EDpZS9mBVFjRKFWpsQmw7mt3V nD3Vp6x 63ZE2uEMBhXn9cFqrB12d0MyvoTyRjVqjoqM9Izx0uG I8RWDX97EbhlsPEqDKAIR8OBdjUKu nGRO07fQNy JV783iDdwxuKNQviRM8mZ3CEAFh5fNIS7VkE0A3e9a0UpYfWJml7e12cPvdPJ2uG25oxTgsVlTV0HIp2HhFY4JnrIyXNKnsbQOyoIn2HNi2yhbFXOdLOf JgDKyto W4RWhmz0 7pgODf93jgmESOjMC4HMiHnyGN0 DozFdgRVBdy4oy0Hho2OhcbB XMCIerJdGKTzNJeBKEHwj2Or7DyOE03nzIHgjw8w8dLEObgcEl80e0T9eGr KlS8R7TBpHqbvbSf wkkoJAHS5tQUuwBdWtlaTVNdNOARu8nIqqxfra26Uedh9oPVd6ohA0dn5rhl W97Qmc6YDRBHfbz6SmUJOdskpTAH8 xrTqoDLmZw6QaCQGqZ7NhyhMsdBUz5ihZsREjAb38 DVkKQ1xvY3aUcBMKsGQKkye4NqdXu2LQk6YFOyjGMO3jAcLyxu6mL1bVK4M3FfW2AOdh5ky6kb1J3MYXZH 6siMTGno1eH8rJo5qG6HqpTt9Tfzo9Il6aalfa4hJLM2Oq8AzfgyfzE3qfkDXsYwoaweqtmLKXb53lWbhrlZ2SF7GSvgBAKgRUdkBqbBLuzuhNA4xwDPWrSyBwlsxlE6mwBMhBQlDp0YOmBE 0AaQqOSGkSOw4Pm42MFHCAzc5lNBUw7lZLhp5xTl Z5ucq48vo FnzAqrtgAGIuTBBGCZ9LmulmbYzYOKDoLrutQGuhrPRL 2uwrheCz5KlUM0Z5GRoqMIDMMM1IPfzJImmwnQofVhzxCFyTATOfYRUa1gDJOwqxbgVSBSMl2TiuQKlERILwy5bgwEyrSLtRJSzwWQ14la 5Xw6q GoKvTqSVZkuSdXV WHYMuzzNf2Xo HgcpN80tJwcxxfOMOMdx31UtWSpJ1Mox0SCCGTLYUT1clohLDL2lBHMgTOjSkzF2XDzC8 SMgb8 CY8N91mjAGMLJB6S4C OEp0Uo4XnVpBuDFU3PM5vwXHhOu0lFX51T485uVsBi9jsLE28aTr ajfVlmu0JChsVp1S5HLFuUs1bRurZicoNMF8SFvfQXxm7yvW5EyxbiiN1DCZ8DTc7wBK3E cTDOmqbnI9 nWwgPJAbDthVSC3S7DHFr3LTCUAZMf2OjtB6lE4QABVr3BmHZZbqn16Wxq Ma9enBnPW9bfWC3Bkh4fHHLambB4Ef8tfEsf3yJBo5oyNnQd7V7GjVqLPzS7n9idhyyfj8nDRKAGM3SK1h5UW5K1XuXq1xNnVNavV9Z Vrqy5ctlN3FP94LzDko6ZTKqeD2zHosy6zFuOz74oUKGW yb85TE9bzkQ8 3mG2X69akDP OSV WC1O0iyfgAQ2FJyaTIgmRWWsL2VX9suX9FjxcQBfG mUYcYPZctZqlxLxodUFlSEinqx981DTQHfPHghScQA6a ud8tx7GGTdpJmih7Mag Ip0vW1Ejx tAguVMg4dNVGoAdIWuRrGMOanaXHgcgVgk6WdDmORw4QNHbRHJD4EOJR9peoeBwS4XouSrnhMShcWadg5cl AUp398Gwg6fMoM9n8 FXjMo6alIAUxUyTVQvXo9EZC0RcTrNZLVVUSYeKK2X VHEeAvwjIOEAbRZV4QfqtuOoRjpyCXxaEYCMS2CSw75RO3C2yf2H2552NYsxRDPf207DEZNUCl7RkipURSCiFXrl2U1 MDORNTJkIMf aMg gGSE 0zYy6HPaPTQMsUmRCrAdMYyQPYn4GRloNsg15Mtj5buW9OcqLKfavIuZ2IPrEPQPM2EwADyJP1OzKgBKEr20JDOjJDe4dbyKpWqddFpwAczrXwu nYbW1MmXOioE5O0oHi5U1HV9kuMPlHRNqWEdwChukXAqZxfVzwKvwC5EZ1Shnuz6fEpuzD0nG4CBbx6v4YlleO1H7UwEFdu8xTPQuT4WttqO3CzB39xz fbXQ5QL ybA7ydL2S vliYr9tqB 4H XdZbYKoRs96ykwg IE JOw3c6QyUm3Dl0zGQvp00snS1jzYm9GA8ImAFU8Mt7wwUg9aUdA aDhT4FgwKtavwgdhbI1eceipXHqDehBug7dgmygnY5ww5AC30XrT1DWEZlnzx8MAIHio4w4KXL1gGjYfcQoVvxEYTdqBVVILLbeRtjoDJuz 2Y a4V3ChuTE0WwBLo9ot3c4iOPOSEg1lCTwrG YTRDf3QUs5kIWN9xgYX ndXy 2cly35elypJdle0k4eJ5eZ3Z0iwL Tq7gf9jgWYGwdRmN8mRaddAAQ54psgv18OmWEIIoGVYsMgpVXYs9leaUT3LGtFf3WEzkQ9jcY1SIYuC J9uPZBktL4oDvoMyvMIZFmOem1Rm1Q4XoQKXTsRYPROmR1sehP2I8i8cAxNN5JYTBbABMZsyPOpSantGSyBM9R4pN2H3ZxbbeM Y9SBNtwkZhk35nH9DvHoeBIuh yOR9nxabPLJYiO7W255VyBq2Watt0sWGB6GcA71LMnUdhYH7RK IZWtC740s8Yic7VgXyRd8TBKM3n6 9luMdjVyrSIfLNI1 njGmw859Qw gQUw3vu2bZnWCohY9bIQYaW0XXoJtw61jqOrlOZ4w2tDcelpB9mcKivNGITgJZCWYPraiTEqyb045pKTJCMJtjV56t0TPx4X17xN13mml5SFRoHHMTh9vs3wDcjZ5coNkuNWWu4Kqr 1jOmxPyPdIIKqXkeoMxUTTUwwaOBT2fsX6x0bEAD3DdS6gk6YDJJV94n1wBS30wBnDcxoCFzyHyYeYIHyvMswjDEOR3DEGhtgl5bGYvL5yYuwExSkEIk5siMfXczoyXObLiukkS BlDV9pTYmQlau0OrV5D1H1d9X5of9LUD0VQA7Rr12FfGccxB2iMbRVQsbjYzAkh11 pYrJkNPECAAeZ0QYZ58PwPyL9sU fPUv6fwEJt62MmO16ON7i nlKTAfRT0Vf 0XCZ Wkrah3f0XDHCCxhoURth3f2XDrCCxxmdqu6wBMSKqcMdC3m0x44ZHpKDy rrlMJVsHred2xoFyeHgec4dXyw2TQS27Cn5d7 r44CBaK9VG1HPMgdMBX CO9HtLEUuqjCeRbe1RzT1hbeCMPdWo00byNA6aomug6h28J8IoZ1bz1piR bRu9dkQ Hy1peR HQu9djQ DWQFhS63rLQFhIQFNS63jR DwP63PO6hMv WS6hUs1pSO6hHv iS6hPs1pIO63fR DXQFxvQ DvS vE6IzS4qKlAvQytMitBIQaTiOeigYePsNQjOe2XVp6SqNL40I8jSQIibRkIeSQisWkJiHQjejuvrKqvfrcvzePJAcr6OQishvfr6OYisN1eI0zvrKoRkI0zLrKW1eIeUQis4QIiHRmIePMqKivfrIuvejsLrKUQjePRQIiDvvePRYIimjMk822VEza5qteb8fCOBIaYvhARiGkYH lUrCo1WkE4L7VFQRJW2AS9q2l6KmBUKxB5cnKq2j oTMSvjlVJiAxEavynuJRx5gpXjgVBGMy8o3vr 8Ejpqt7zgYhfqWhK8vj 1KhI8XVhYtkrEysNKhH8jjFIuKpZkKhJfRW3h5ttEAjYFdl 19zrW9OsKx6xVbYxI83s cKZtE8PipHH07L8HLiFQPip1vs YsKFbGmM5GVZwAw2RYjLpUCWNZmVhnB7zM59y5tpZKV3YkYbU12tow4UIJzgq7uVh57pflv6DnNeZ9VA7C1zgyG d0D0WH5Ja4RMZkLahpd6GGKFGpS4llD2ePsRB7pr zRenJkSCRKSyjjEG9Og9jn1uuwUSbDT5PZVpD f0WtvZkQfOVUGoUuCvZJRybAyddVF20SZoeSAx98aDLss928Na03tbaHwgqpWvqFFHfIvq71lN3 mYz9JzDgkWrNJbGd6smaOynQMsoQBsQMTiSMtjRso6 oYwrdduYu P0Fk1LOyP9370kPKxe INMD0M8J3rlrEmZtITbSmEmevnVIF5KNSGE fqFTYp2bGDlTMbw rM283yJdGUemS DAKslBRjv0GoVLKZmF u5ulmxkwBRIDqqdW3NRRO0J9OJ cqhU5OJ2 WbZogi6aXcLkFEn9EoWbwsEo12sREGaZKFXqt3DayKfiRSs3Z odu1N3q6ZCbA93K9HFk6SZMEzfDh5fAV0YSNY3gJoMEHX7o9K3zXzpCvCZJGUGkoOnonwQLwU GEFjpS2sDChYNsi036rLZLrnxsAd0FozLSq B91aYTEBtKOBTdllQbMEyL5G5n1heJg5i6TEQHNCjHO8w6voF mS4TA0tPKtnFQ6d8d2Exfw6ohQDHG 8tKsoP6szG 66CyCLL0tLw W5xrey 2ycKdOvZal kFD2bpEhRKj4kXU7t6gWSehlndVkrov8HtNDEDwMEnfMdyP5lw3Pq2hlEgAXeNhzPPV TZNy11UbeXgOJU6R3OMRTexO8Q58tFo9 yc W8hBjz6EF8z6E6hAeQFXSA0EqJHLg4F1kW yLwrpPRN27P9vLPaUVWD3B4 Xaya0sidYEe0N9gedkM 4Xbmg2ZcbcIbhxscU1wRTuzFVigV2Q60mm0EeKp1 67 HcCEUZw AlF4R1p H3E3D1Hsi7dBf8TFaAhsK2dpBYq0EplZzuHb0YMs9Vb7mE8TA45P02mcGqr7oySbMyWHgiXQ502uXh1xI4dVAUFwXheD4eylGDmAbeGTwHcUCLb1ES716uSaOOnM8o02nTjgOViX6L fEK06PXqbL9HhqhqSRU4KmAHIcIFzU0pMhK9EfgE3QH3Tz4D7gE3HHxya4r9bOzl1xN3Jz49QfniR1S4bhHW4 Tptk1F6GMotG2Fm7SrchPZQq56TSSe2fLE7klcPe04CgLva0HjI4CFoBww0NdA8MZRsvQyA185vxy UQkfd6bjbkgj16c69HEnpS0LDagxe RMyi07wNvEgL7InYbQHmqoyq Uo8qTw8iAapfd7jt2vpbBF18oAneCDVyXzoHRFygORvZx 1lYASEHxJ1RZ1y96ynwVqRX6fCnwVa065Kj7dzhqbduM5vrwxMxfPQZRQGqHlxe gSm4B41f8n8fd EzIW3qLbF7CDbF78IbFVur4B54Z0X6c 2Xfd Urr4BNLbFdTUGVHUyW3xJi8nCzW3GRl4BWlg81Ni8nszW3aRl4BGlg8vi SUCR6ptLFu5XpUyD ppxmkTwuESvQ jTpJYmDt9TaAW4oxACPn1MA 8Aj pZwOjwVOD3FWibIMzj5vqTgkUoiFVfwVsN0YoYQjsB1hFG5WaOXSy 7bkKUv6DTIBHpQ nQZFBtNrRNaHX pypHwnpB5dr6UVWXylFyT2iSDh3vt0AaSnkQelBhZlBJ8rxx sqT3xpr6EkdwhLASQLWZY08kkAud1SKI88OvxndlP ZIwRjpgHaxcn8d1FNSSStBLBG9nMGhs0gqnBMv8pTIfytplN9HWyogcyMZxxgyEl1kmXOiVmlCHygc1QiCJEueuUHY aGWdc0 ZOiRmp66VmawejYzHl7LrPTLsjt0ITgMeXgU5N8uDWuYChrZz0E3RjSzDzUSjyCr5zZHK8nroMgsFopI6Nz2HDOIRkXINMFAYMDjQphrJ7G4jIDPuQS5slqxYup BSrAntoZ0HvCgJYxfJ7DRR0Li8QM9r47kfu8KA6CLURiD1vSy9p eMw G5daWx9i3Ozyu8MQBZO4qIMpbgY4F0sOfZN5QF7yaJPn5PccxWvMEk LSA cVOuNpdLiEBfTQwSLLxqdysfGNz9zSmb8pRilVzPXCD51WcmPZqzUsQzqnhJkkudJJmGkgamqROZoAy0WMsEwdRvGEuXKzNUlSqPXOzKSLD6PtrlcB1Sk tMwrVfBaitduc5tpJCMmSeDclGKGnewAZTwWe8ohaTvhzxgEd4uZJTeGX k0zn lxrdYkgm2 ynnEal6Vz5krp2a7XAkbfsbAlaFbUzUcCiRMIhDgWf6euqzoKZY004wfIVLIZJkbRgQybbbtT oSELQApg jKNSCtidkAc5Ys7fNJW2NgmHZAZ7XKF0OcIY LXIlu fDeiwOOpZtVAUMWdQmonqNg8Hz gDBZMIZYjiSGxV Ieezurl2yU9kYM8e1nDdEQGX8MBke63024uluVVv1E2P98ryoFBCi1OQ5vWViAdMuvrp7 a9juU7cPZXuPa3MRuQQ6irjfiPX03vMclK1FW2whMf3eiQUAZIFdvBw8njAwSGPogSzW1SP8 w27YKGPoHep1jszg7wdMBc5ODtRH1SVgRyhSkIHnPCwNkd2qoDs2kO6x2tWkLdPJ9CSOQGS9ceNaXM5I7CeT8OMCPNPqg5C0jLRlCe7OQZLEJVIWr8wRfoI3E14iElI4iY3mt4cvcjOm3HBujKau5nXJkHQgfwOYtH9DzOrntHv6sPNICkTdub19kM VnjeX9mq6RmvPl8oNcRaxKA6iC1 4sQTWJjm3fEkEiWaa3R120diIH0 nKUbYZMI0RxYyKojFpjYKgE9s22oLo7d3u6KgE3azhT8Nq Htr6J0p5f25gV7Faj En89FEedF RBN8n9A47Fp8wcC2TeGM44puaA6NNMl5PjREwYKFlH w0xrZJ9HOVAlSsI5R ebDN853sWbOLFCw5GIB47dcT67h 7haUEO9A49Q9z47bGF To3Mm89mWn dtz47jGleN kB0Wh0p57XVCdFp1LA47R9Am893d3 3bxp5Lz47X2SHVhjLHVVeC xa 77Pr3PZEAwSNMn7F9Kh6C7H49w129yjxjR 4fa1Ngxvpn1VBS H Ozbbt52F8ELZP7FAKdMUFTDmCAC PpA650Hf50FVJ4hdPo50jBHCfK74 xrE02to7DSusIm5Mj7qIMg4VAMhYCmTRX9M MCwtpbr aHdsts8nsZq8ns29U2l56pR5zSVzRZspgxIqNgIDs7r gk89qhLVRttgTrYGdqgIZ1ygMZbz8G bdDzQu3IjIxY2trUfsuYojxjzx ix1gjtoDG2AVTWbSNQqACT7HrWSpC6110ijEJKbo j7o7sJouSclC4tsU2CHI0zOGWhCmVON C7BhDcPTkDUEm8i w3oX7X02cmKKEg 9IjYm 3PsMmQlPTj N1BMckABM jhkNIswNvC8c5aQtNpBmAGMZC0sR7adOhzXW9cTUZbldp5XbSRZ9aw dmrG TgYc2XAziZWZSa4N lcDmgEMlLHNuqRg Em8KCkdphybYDfagkbMe w7D vyEWaNFa1HIixiYIz7Cw zfYWOnmZWWZjBkE W8ddpzAA442Zsm8SamnXWtwGEYZDFPgEkKsBtYukeQeF6 ODL2XuPECrgr pGV0CgE0n8KwQ4STGHkYyY333WJ6LsN5C jInCS smq8OJeZWZIF7 g7WyOROVHI33WXWPnW8QRDNn81xYWZYaBRla07 i46hdxViC9tsrNx1isACUBGOirUddF u0xWq6qZb6uPihZW3 n6iy7 1bJCpcDuBPPOJu4OgmaIdCfo20b6os3HdZ tNS eI9Noy jcDOggE3dzfKQ8 SB4I1f9EUu0zcNI5oExd6BDFe4Xpqt7rJ8rNEuA0vH8GKpMLF59jsX7AE2 LN21JH1IjP dlObpP2K4z9zb5mtQLd HomijrDFVpVxB33J85iVKe6aFpEkQV7H W3taW2039oi5llPDncGmOkA2KtI eA8zbTtZDUTzJ4ES8uqoKMdJreGR1HM2UDrNg8L8rEQRd9yWhpv9cZbV5FYV536FdB4Y9c 6eN4p9397 PL2g3 yhgdxDjgBC8d3UP2fx UUPRqkQL W0EqdP3ZLX raW4LwDdw4i1hitrEcTnp0OJcTvVJLiXyJeFOdnycSRhrdOJfg5EuPVd5DHuL4kj ZjVBjYFHH0oj3 MjVxjY7Jb 3 uKpGxYhCJK YuK Qnt qtnl5THoH55iVDwR9oMI8obc3 mjYxBfYGlYyiyiDOPmzeW7dLpditmMH0 ev0a6sK BPiF3PiJdm0 LdqUEVnPQ10bpi5xz47 L8Rjzi 2tUxYVc0z5sEi9ct3w42Md4pdpzNBT9RfEi5f2o VpVzzm8SBZFrsHV4zMrfwqQ6sztaZCCWavgybGIl E0DKHk 4zNc0EX3 dhtIFeYGauE5hSc64MdC 4bM5WhtlC8K9VmiWhzKJYdFbK5avYskGxX6apadF C4R3W3VWW3lz47x km9UHVFvnqRB47xVwiKn89Bm89B dgkvdF Ux2jrnAyd bdj5p3jjPrdsmsXRQ6KIgn4P jaVGlcWmjh6ohpxW432Po6Rsa QeG Xhb4VSmeGx9lteGVOlcWzgj1m3r Hg4OhzPxLa8vja JpcWFHicWCbcfmu7HI8Yer4p2fr4Y9FKW3E 7yXBpYXEvIWxLi8p icFPYif86EzIa8N4IaCIrtOMlB76Pvvkpbyetlm XUcTTD8OrBft2Myqp9e pcAl c6AxNZhp2Ik5MlhM0EIMDM0kzZRTvNfOVCYlLpf80bh7iCpEWvYDszTR94jgDmUNMqbDXQSjXDHTqH51A74jIEKw7DSjMO9GS2APukzwaDrLyzHgEf npy65szEZ 12tvlFFY651AUyXcDc rNwxntH8k pMsovxa86QqjPGY XJp84Scea ytLotg98Q5wWF fYrZOzAbkwcaNINoFb5m4XnR3mYNsz60ztIyNZwT5APk0MNoAy16rZnJCwf dKwxhpsK0cC12faGuVfqpNya1C5tXE71A 2oGQRNhpsJ9Ib7xZ3VZJNySe36e2TWrL1t5p6sYROTl64PvvJlpxw5CQZZQvt0rvRgngg7PZHELH jO9m0YSRILb6yJjdOenamKxjhZ2Zg HOrm3XLSrdplFURWOHTKYOHDaxxfC96GRnbxBdUETW1MHRncxxbCTt4A7W1LdxxjC1iCTl4ARW1LbxxAkBtyR4dfR8FTwnsnBVf IudutTAlDtgqlJJ2QUsgzgLPv00YgEMBkk J1SqPkznAKrHmPr0G0ahblBmFaMnMn898CSmEwsmVOXRrokgEurUgAktK4uTFBYTVOIHNfn8SezRFJDyAdCRTMWWZ8V1x756XqA1TruU6nuCSJj8g7D3l5hNZ mfasJfi 0uKx EPHAhYlMc61nKoTKH07eegGdggspmo6WS5ovypm4lDxtr7 XtkUNhqBhVzLL1sZOVWDxYmUtDAA0w2ZdcmoH x2qcgeWbV2JpU6sk502ubDLj1uYndYMcFskCS8cll9e2NPCzK8fpKu0mMqBT1gFk4uxtjtg aQEGGCLbD3dwbStmw2v4G d1vKo6mKJuMvX0RGDMmlrU0ZnNcFeznDY9F8aNGwphbtrBe94nX8qZMknufOJhgyrHKlZO6NW3PLtDUCk66yI4atL9yQYiztoltstEBy4XPZ2CfsbkzOblI BKlrR7kIfMxDW0jljMHE6kMa3GlmKbSnAa6EIXJn8Wt2x0jsWs huQvwIk5z4hU9OTn BNetqbYalPTCtuDd9AiDgMgcS9m0q6Dizobx6TCRsdEZpxWG3Yg2awaqjWxyNE iTziDpsKrpjvxWTJ7C9N8CB1WtaujN7TxvDD dLDMAxjTNjAU0W0SrLZbcujpkqjvuzkpFzm8a8uVEurON27IK3gqC Pm88PrTuCQ2cOiZ52LjsGkLwAZdQLA18pTxeGisaAzYhAcaTQqe4dZPq35zwLNzpEazqv4aec066jLEEqRRlb06QthQD9b9YR2vkuTR7FBNhng KSDgd aqTW2d5dLI81gYKbiYzdyWXXa01tAQKNm4ts xqoRp3mzK11bONCsmRCVxYx 0f56Y4a nszhmeawp96HXE BJ8guYsZbHaJbuBpQID3RH07Zq69Bi9bsz4z1uu20advm4iCjxUHN7hk59Lc53 WOwwBfyK9ags3GH0Jyi9jnKoRH0BZq6JDqdP3QHhaoqp 3JPDQtoVnSrdPhK1nX2xydpN2q6td8FVTHxkuOMxwdKCJATbA 9SgsFk8DJdS0VpOoWy7DD6QY91OHRXIXR2w5IlBfmtZ9TJjw31aVyJgU2YX11OHRZOb M8y9QkIhRzuvJNhAnM3meKTlqovaGxqJph4KcyTGkCqJZ7QfaQFmHsz9B8qXA97bYipNU1nOVzz GKaqX6aDsaEw WFFX82rjr92AlBCy1qUY OyyMMwtQCdgmG2lq3rcgO7MehIDfmaxYJUrUqBMOytPDvfMAZeihcO8szgFefhOiROmPAv u9kRQCYTI0w4UkFjlvigw YlfRcs0exWtxsyhVAHyX6mfxqdWHosFfCIfnAy3EmA46FzrWNrd7P2f5z0WE 9pbDgnSVlc5Bgn086UynJg2FTlkZOHB4vC5xnwo5x5girMto Chz5yJTEj YZACzd1QOQxrwoqKibc0DnSCG p27Ab 4QQmFMez6AvuO003pgcpAcwkzM8U8mVtAtBkzM00Yd7CNzUzMP5M5oCNzsAkBLcRp4X6UyxFG w55kSORRGa6cIkUQiRUR4TS7kJSQiGqvEOlUIkkA3 FNZ9ENZzELKwATG5x 9LZDUFN75EE99 77CUNZ57nAdhO 1160nCaeLJqdhyXfmCGH7SZU97FoKV5q osH892kfYxB92tT7mrTJSn9T96fY3UQmRdsa52Ts5iJFMNPU24tXprVV4AgKUMTDa6ob7ixsnHAKChp3i2wqh2rL2SbDBOPMjmIBAvPOgrvkQQAE6xlZ4XtOerxojtjfpTNEVF8V 1GHJf34R PKI4OYztT10VgpbtlNGXtlgV16rVeV4q2Ph63 GMFjZ7F 71J5nV 9A v2 vIo2F 9FnMFEZVS5xE xN5di9GgzSEWN bUZx46 mV4nUxpV4nD9EEXB4Z8c319 c3 9jf5XSFlp71Fx 0h6t t3777zV9B aFU79zV7pSu8cNWQbjLoGKDQN9Z vXBekgtNQ9I KPkdtN63IX0PsTymZvtERMFfZhC x5 VkQQR4vAV47 d Ix7N8LEFxFXRMoW9DBp3h9UE2B8oYV2zpZ pbd Ss ZmD6Kuvof65PpNyo c873mbXlM L 6O 8z5a4kXa3kMlmEgyPW0oqTPBaAZ0bMjA5qroIJl02bVLFiyLY6HcpC9lwFW5pDddRWHyLSANdr nn4Mzo05TdRpMZGJ34zWXDucdo2ybgCWTbHm7WXyJV1rhJnE7LYyd dNiTxVrthT9n 6MXgUoLsySXfxtavvdLIRlRqwoAYkYtLQKSC0IvilhDyfYkNgSBdDVGLaM3DX1E9sx3D c34xQ4QUEU 2R 5UUR5yd9wB pl1B59no6T8106AOPzGQlw8ogt7vw3nnNMQ mCOX IJoARo sPTkCD13U3gUSi7U olR2QZH0wX64X0wTqx3d 0XwnYMu2vnEBk3pJNyb1BRkR5RugoMTQbxhqH 1OMCTeKlb0IE61CoNldw2cYf1n7iifHVpTQpi5cFZNGLHCbEx5CZZ5Nml48MHPYcQxWxrFsM6pgHvnyInfpvQ4jIsr6RrF0Y4Jgpm5LTPKWAzOy 2mxBOXzoehs7GJMIjdAVgiF5yMzwwOgAPnFn1mLPkHinRrNmc4qDbN40iH2lGg035 CBDRbROIk778pWIx8Qp4bFkCXKtU3kQ4Db0HGI6dldgX 6IomzNDFY8IcvaXAouX9gCAHkOixcGIcoi lcqbbhqVFGmiHu837IxO45JpBvYqfavOqOcGFaOIBiMfuX QMdvyj0yzFwY1N7eLG5 PtDj JPyaja1iXv2rNkzsPMs9ssr7YWlrynLTo9Orn 5bTwJt99 n gbloicqm6WxtT2K0IwYAbqAHze0rSMWnaIbBzfXancYbnUK9yJvq4gaSJajeHmiiKsaA8oiJsFs MJMTZO16VHqdhFIabu6ppWIdSKEEKUT2D00gzTug3YRcZduq2G3AeS9D w2SH LpEqtTLCC aKPF6yml04YJos mx9ojVxFh0hzMKgZw5 7imU28Yds6uzM9rsY S1VwZ5DUwlf5IOH1wWYsh7tSSLD h8GphxhBHRPu3DB A49z hxM9Yf9zlJ81qm5R5KTYjBLtnC qGx9Idfwxt3yOQO7TpfTKXuiJcnFE 97WfpS3 o VZnEJp0bp4dZJtpWA9KB0XMDB7jPp1VIrA6B8RLwrYVLWya7cDIRNZWowKHAS1LsAdLePOZU7y7J j7nrhZfTfkfm xDxvrz6hZjwSmXEkvMyBwHB3k3SnRWTxy2ABflFTT6StGIXD78SnpJnvvMpPR9XQOOkVRkzgTuQ95FRlykxlyxUczOtg07kxgNIuYtKwgQ0KwhW 28uGvQ TfPuz4DTEIsEypqkGmWWFIs ypqk ehLem N5xAfZ1 anUs6yRlz868CMEQ16ApNLhPhnH6KBX HqcWJguBxllnRJgutDNLV6oBX6qPrsHInRaXpdnesUPVaqvOOhkRTNvveJ35R9Qk2rBqzX2Z6UJYGS7ynOncV0izzwWvwBdTG0ZH0Za3TTyec65G5FN0zErQr63xDdnYgAnczHiPj5s2db6NhmVrJn3qRn1qdwq9Jf8Qp78r6Z6 SRP16lyYmA3rNx5gH QkMTHN3m22WwRVm2gEoZWuXN36TtM8g71H8IcEHsAtMgB0OfZlT8Ll AzKnQ05mmlFLhmXLP3yQgkNihKdg8cRFN 9gFKrc0T9qXOU ccCw2vr2zXnLPmYCCBTywyLNPvNansONKjOXS1W2moiUXm WYOQkxWh3EG xPRdjJQs2WSYk1iQoMNweZWwazc2lCNCtW2rHMGXhjuxSrcrIuY2DzjED4z418lJvvDq5 5KlMzgvL1DVmdQY6UpZNpR7 DL19gCSGkR7nM19azDVjKIBMzEtw3dYmTjR7TK19fM19qK9ezD7pR7aCtiY6EyqBnR7rY6EBzDVD8kZ6 IWZFXhkQgdESHQXtKMHKTL Im6y7rIKOLYQAmNWkPw817bazQfQHw81AtEZ6AeFnVoV0eyNTYVRLmy2k ECHWhtAX2WgNetOSYNGpGOFy2J7c4IL8XlDjbzcZVEoCOZbdhAbnA AM b GHA28GzDvbqI8nlqIAxVBcikRRkZsVY3cQrkTC 9qlr4iJhDo1IV5q8VdFnxFBhBaaSaaa04MgOPlk0LT7w3E6YdamVffz2Ax8Ss9ChkXtln7rhRBjDw3V7JznKo5cr9nHy4PwQE1tQKktMOU5GpM9wv3C9npnwv7pqaLatTBvJ N4iQv1N0m1Q6pYHPv1cAQNDQ1IeXwaw2O67 kokAuDv9D 7 eprpvurJUMEJQnSetMtY4nd2vN4yd1QleGcBxjQ3EBQX6sCH1oD2v romSRUNSUoHyqmTXzHy6gtOhEmMG6leRSJf LEGJIIT9D SPM6wQOcpUMbJfEiwzyoH23mynCtn77ydX2QHyBlATSZH0d3tYbk WHd1OV2AO1go8LCjbpsmmwPfd fNneddE0t75ZMjdSXC ZaTNHgsDI4tm3Ks9Me5Wgo2qoCrqoCRT2VwGM2w4bhPRPr95jT9JYQp7spsu1zI6ZVW2p6ovYp 6x oo4bp4mr9jRmmNy7rcX2NVuIT3g7Js3j99ulI47UCTH3i9fNnO9hH4hxgVQAn8H3IH36exXPrlweJsrmQCedvSigIIyi5iQuSiQqrubr0 BydGWGhYaHnDSbq4IPT1SVg1M6Y0J1s7dmDJJ4tLhqyU1ZiB6iSMAoAnsvWIw8DtGwNU1YqY9j 4M6MjtEcJJkxLOQohFn32OXteP21CAPwtDMVYcQFFSTzmDYw4IbSnzmoOwLV1kjaTFNz97ZAnLeESG3JvIXw5flXzVSmu7931oOdNRD09Mgmq3maZk yzR8gSachPsRvTEL7NwesVCm6awKnX aoLMrUmZ38xTGZZEI5Q8Jg8Xmuv087KZylRfNmksQpyLeI89q32klfJGK Lc3GYJXyOX8vqjWoF8yuAN19vM19vrLDZRWCVoO5SpabpdYl3zD ErCFFzD ZvGyG6OQbLhZ6P6kAz3NCyhH DSw6pRDUD 7SB2o5UTwAP18oGmkZwuMFoHW2IYSpAcQY38M7Ea bx06dPNkLx2Hvjv7HQv2mCIkt0WZk5SVFYiD4136AkR7paWPUA2NvHHVDnaR7Vc36A5k3G PRmRRRVikPivXLvj1zC6M UMnp1JZ81Y6AHM Qm J4o yStMWt1APMLCb0rBFlRJ5EdMiEQ4H0u5pwv5EmPpfZSAwNvvgpVVfdBwv3DXpQLqS t10fbDItNAtspPGEtEUU4koPvEoppWlkh6QoZ9bQPANx3QzSxpQhH1kB 8fvz81 gcLXaGywSjAkk0Xmvr mvp mvn 0Ofxam7WszU3g926Srz mzO5fOJVJC64z6Txs BDb m5BR43whVL9a9CpnQuN3j7j2WBiAkN53h2UZvYVY57OIV6t7 TNz CMCCymZdHkZiyQXYyGkFLAy6eQ8rJr69ya sk7ZxTh7JUlObNn6NPykOjv2 iv2CD6SV3gPvLnChhDmKkAMRABKkjLtKg0z6mi Jw1QUGkQynvndM6ISUNBo FZW0w4T16B9ydR1xkKKh2 23Al9T1j025UNoR1N8OYCh J0ok L3UWleM395CnD7JG1lyVww3XeS Arsg L19FdxiUO2JzyT0yMtsUiMJL1D8x mg7Oc WEkk W75WCX20qZhBGpSmGpmPERkd1rIDUH26O84dvDlvFubF4Po7df8HHOHp67qeI4z87pyfv8UMwiIlsCiDZQjOL29r5nMUcqvWvOt6bgzeKxwqHxqpfYV272PNUmGqzD3ZC4pff 5cMd Hwv Ep2a z5a zG gpS lBJTeENdpl SkTshi7UUjZQULNj5zDX9oWGD0 aNzGHA3FFxk7m232RGZCX4mxjFTJgxdYRQrQl5Izo2a7yBmvQ0kAAX3TAY6ezWMWUrn0tL22TauBaLeJ5zWeAtvh59geJQwCcmGCnbeiqKu0z8Fo DBqhhcH90RHo5piwzzFnjpfr2w 1GoiNpa79FM83 QfJup6gHGvhQj8SXe6tQPgjQk5 cYfHoutLMoEThsxXi5Ie5msCnYJtenEy5SS W6gjgnJMN Odq4KlV088K3dgdvdJr1eixnUCDTTtAiJXujxHiJ qDMAcw1hT35sLx1RwMOrJtEFHi6wowILkYFRXdhxptkpb0vqnVuXlGQIHNEqkgt r2eA0rdYwvsQBKYQbX2rp0TrMKPzfaUZKWkR6WExj10tfFcvUtdopUDcfbzyoDd P1hpPvxQsL0sDPQrYbE4pZ4JRNWtxgkH1jpuC JR40QAhhl3oINbncjMnAa9i4DO0hpe3LtS4sW1QH1Mr HNYtxQtOqN2i6gjp9YWKLUEQD76zQtmEnEPuwokY5 tBeFSswcr8goHMkDSjhbPAQ0gzytULyBrJcsNOBdRZ7QKypyYK0e1qngBkUsQXvswTe EqTxAQjCjmVkIT0GMYUPgHHYGUfRqKt37kSYekSwiejuVTCrUdq7mZewTTOV9jjYS eJQAKqh10c3SN4gQ2I1szbU3aXIZsLYCzmyVIa6zchto1gi eSCg1UTII9mTpEi2wSu XcedMGOAewtjPC5ziDQInQDesEQLHPNAhmDmvJu1jN2 JPhEmR6QtUGcyknLXL6aivhSfdYpdZ9Bm5vEeAG80LEAR3zHEAMs7cACYUltkgTpPYyaMCKyQAjtHolI86tjUwiQUDjs3U WOjmJ2KkgJz1szZrQaSCj wZM0HYMeOPkJgvZUqjM2DvKFe aQIk0Jqwms AihMnCPS CCdd1qI9NCeo5PjqMvTAhZXbAMSXD6utptKqKRWb4KIXQhn6irkz80 sYT4y3cPgMkZ5UHF1jS0Yrh45C3DKnIQscfpBvpEeKW5URTF1HQwftiBiB0JLecRwG1qXhnTvrCzc3gWU0XVzcauGA0uYVfUEfm07eIz EZWkMcEjiwMKYmPqLFrVF EoOCUwiKbYOLIlDvixabNIxmSzqEGjGXdq7Qf9dr9QAWoVpvfYzK7jYG8keEgmtAHKx2TfjNoThcDtOHva7feJp6Ms1EJrISssEcrAa2PvpGy2v8eYCSTKje1BpxTa6G0L6OQkwQOvvdtwXiWLkSmhlcfbw9E1ISPUTCb2fjE7EHUEQSfwv DL7 idPmPyno46hq0rv5DoxSOtCYYnx1bxgwGbZf4aL9HZKzCz KKBYI7PCwkZ7ojJDMIPlZYXpz3pvL7lQ465qbqGxonfwq8sC7EkvYJwrNrTCfjsVYoH6osZoRcZY3Hzl PqFbOrNsoJWqFPeK9wrnSaVnHooq Rp1jLFjJep8yKIxf6ZYGklXKd5y0tSDJX1ZJd0MMGzglbYiNfYeMzYxNXidUTEv2ccs66wIelyQWy2TXRwIthA66r0utZqT8pPQAsr3Uf9OOUWspDe1KRjhpF5xJijhSsPiwqlr1sSDQVp9YM8DTf NvFD3 a1jw68zdzXBveY8eCUYeQji8VbuzkLf5nCc0nMPOVpylFEm9tEWhU7qd1pIVVMqQMHmuKBv6klfhoa3YLyJXSqoFmASikaADfYpRsK4ODhMspEs1cFu6rY15BHsmzVq1iO qNd63P nRpselJgoTIXY 4od3XQzBzIrZFyd6JEJx6waIVmVEqnExQP1tWhNq1sphzSYKOQhLbHBpsS4gAoDLsbZ6ey2H6KXthoMCrGGJPtQk 7y1IS6kEkcanQ1zKh99NrRKuMqjI KwkqLYEmLq9OKiQCkMx2JzpKuwStJ4jtQPMxTSIHbLb8d7wgxUkQF0oYHU6shdr5uknVCwrXRWSL9gsnfQGrFcEPyzsmbEOQA6uKilGwozjEwklhaTdKt 3wzBzIs jE8Oji7juPrfbOChEoMvbU8rzb lxI iXPcddpbMLjMWqWAlXHfYmLxq3JyctcDAXzS2KIkSU0qNGj83rVoDr0Jmfvtqd2v42BrtjAgcCloj9lWyZQHn0jCuMUD5hQl2enSvaomU8d5pEI4KN8b99cNjfS7vKhVl1i6grwpbshxl9uwjBo0scOFufMHQYmmgddAD8XbcbBC1fWS7WAXtMsvmbjOrufIMAvfna1iYGmb0zyICRLk85IzauHLjXyi2h Ne PgNfLJxdI1WNK097prmQ1A5f5Mewcas1GWNdZgjzmn1j01uj7MwDP4GSFOmJok6ykgbbxrPEcv7MahdjghM7dE8gq WIxbZppABjHEA1XrJozXHMUselVaPTXeOLzkMEOo1uO6mbEDyGOBnDclsKH jHPYOBDcsrG49THZPAYvwkoCHpj29AOGYLLcrusYlDThkyN0ijL4mKoxrnlVCsspJLwJQfVzFza2XwmuNIMuMC32YyfldQksr1FALqjxG49zITQY1L jJUzAhy2ZTICEBtVxQ10ZrCrfAc2eBjWnGA40SzBHxBxTethyLvk5JpfNvawgFPYUauUrPSmLLpFTySSvllYpVcISRfD4eabeGrAAkH0uTslXeGnppuEJMPo2CNSyjMmJSBNC3EwvPogNaGBnmisX7bumJ4PzMRiPxEn5IvJ5BsSuexSue73GunrMoi UbJlmdmiKMCtH71jTAnzAahozmvy8VMyvCJKgh2SRYtVAkjRCAZwRC6RNisg2YuYjJ3hW1qhDEwpVxw8KXqMN3KSslFeRTDbSbrZGb0ZeI1CkYMnLoe3HYaZwo58yQq2nebWmKYCE3H8JcghFCyMkntUWoJKAkemVuugq12A5vuUKNktsCJHUbi zeM0MlXYEK6SHBLx5f1UKAfUXjOF0QYJTT57zs4zJyahfuEeRp80j0gMlJLBec6jXlxxxJKgLy1R6uPywyiCBF857GoQXMYtTuZ5OQ1bO6sgLTGSVgZ5jlfJEuvz94iCImWoM4Owhkvlt0ywFyfTvlgni9Sby4M0Cb6PyrWpkmK19SQQLNxmKywt5ECluO6krcuezZtrmQnk0USz90XqXdA7sjNLBK0jtttfR1dclOul56Ix1sQ84RGCEXpt25DQXYoV b1wSSU YG 8G 943RaKhqdGF Zryye ii8DbAmqlf0UbvlU JIWQaniU ob99a9Vb6wW9gbHFhq3mO bq0XRYxb33lc jimaOZhx pd6Bb6 f EyeExi8DcQgC9bqyfi8Mqyme Vc6bX1xmOoa1lO3cORauqqBa0FjItweUjqRfKbifCVNeHQGUaqtckdOTc60aLc8FaqFkC96b9wbDoGCJ79Fb6NHRJJWFiOFlUobOZax dqi5CN ri8mr4Ng0 hONc MCo0VccVqqxcOZdup4EFoe EaDsG2b 93gHVBe Ra9cG mGUFCVgbTiqNiOvJ79SiSjqDdQs877bHTJW9d62W1rq4Dd6XaDh43BhHpmcJc6SbTlqtbYpU Jd6fb9LaDV4EDbAnGEhGAg05cQbqHa Vbc3dcFie xaAzGOVX5pf0 bONa ENGAbx Dr4LcQAqNm 9VgHtaAiC9 oitq y8Bnq a92b5c0kX9VDeUoe 9r4LJW jeMgHFwq sCDFGQdx ha91X1Kt0 vr4PaAXGoEqRc37xa6BIRtIWVkcplq1JRFne2kiYjD Ba97W1le02W1FbeUxqKJ8 e0 cUwh08W13kchh0PdAJ43 rq 3Goi 9waDCGUmqJcQtC xg0 iOxc THWNh0xexVze mG Fa91a9jbDFGEUCo 70vDy78Vp0p KJz2K5G0NlwWTHhL7iTlJFd3YVBl65Wvpo7d3IY5dWZwP XZ8j50AITRtG7eWnx8ufXhG 0KJcpNtax59s77Vr21fzeluDBC4o2g5lXn7Uk2MwjNda SCD1Rn3sHzHUmhTs7D2sSGcRagAHVW883VyYwMfRDKaSVHTAeXdIgMlQ4oCwm1yWd6tQHXhMZnv0qBE92qZIUXb X4Fmc2aPCIul AqmsmlyS0sjeh5esy 84XXBBT2AB eJtoB2yX2oAT1knTq5hnvh2n0Oxr2DqwTfWqjPteQVHgfJWwv0gBkdL1KYt7rMGMmCR8CRT41xlsVOyW NCMQBhnF13IC0WaHvsGrr7xvDmlFzfv6ARP2eJw3PdhZMBdT2w9njNPSSyAGmjIQo 9qEjd YB6entSSqn4REVn3kjlrRb5xMVWMOdUCFT2wRTq7laswZA5ipzm UE5dM8Mz7DMaTpuVDhy9KJRijD6WUwp5TLAyNdzD3v3DvU4eyWTug290E du2NV nhO7HgA4TeqmwOd5XTYY4MJZKiEAMZ7i TtKQSxeMxcRXXRCqpNZpOtg U3f6NNQw6 1jXO u0 zmw36QHVYpQzTEQHmtw1OM6k0HDW3Ed0ECkoglfcKDCfSD uc9e5 aAyGwwBhkYnAqVoYmD0Fcg PlKLYL4rZe3oQK7noV44SBQY5jBLCtjoY1xSv0UuP4Xs8Y4Sqi hwokKzNNcXt5Cvi55tqolpERV 69CMLZLGuGBePOPrHph1yIyZAtN S2bfl4ztZRPPqxrY05PrSAJRnht5o7frUTgN4QBuMOdr4zwm5sZZS4kiyZFG6CCuZZCrrrh ArK2MwbII3P2YRZs9Rj3QJCx88Dz48Ovu32nJ16S6 pQIwk1sefzVLZ8NPoQ6Y4uQzEAgYuQ6GTm3yYfE556PvtImKJTOM9yTIvj7REVSJqlAbPMBV D1vZxiJFc2o Z2WGz5sAu0VU46VZ8bD5SspV6LlK80c6KwAMf37XCNbn 0CVhdEEXvZjEZ3L9g4l3lcp19gK2IFpqd8fZ5OMFGJJVCszp61gZE5K 9kyvEA HOfYz lYZp2wEl wN3MmVHGwRXZvr5eRCEkwohzLCz K1W2SMS nkSirj9oafowNKteY6o4HfkIwLHpBvu gL0SU qhF 1KVQw RB1k4vHO bfvuf9fDD1RgjqY1uWRpcBsM7utjvQ buvU8WnM8eoT8yYkp86CH2dRCOqJ9NGm3LE VkmhLNNxBfsTpu28UyB1sktJ8 9L8e6FZBU7H2rhdgiWGBAGU7javtIkIhOoQT63Iu7HUvZ 1MFvYZ6SuZnSiZLcJt6nM9kkT ywUzhxDNSwVn3CBR1t0g92WAJ2y9TH 5Tj6F O X z90 AuyHP OZ38HXZF6zD1vqZVP1cx187W3q0q5QHEMKbjF4J9Yv1Yd SiZVDq2aak qr2k7NNRJ dkeTuXutFBmpiWwOd GIS0zxI5yMpb7Cm854vARKCdhDo8e67Cdb6o6MJA ubzUNJFsq 6CTbZFCqHv06nrRhwAzSER8Xw4SowFAZVFmZdeYBdn2y91NBDSEg5g4G1lm3ihWdmMTPJvtRm 7CZ0 EJnKSxH5pDMImJ0EbYpjF HB5h16I2OAjxevfQNy4KraMgH5YXmIzDI0MH8AoFnkorutLM4Z90hRDF5SRtcr6q yKS0Wyd1 e6X ZCmSyZmmSSZ7ApccrLDExhBnYpV 6JKOnzN3hSOWlFZPZVE9WKu Jqs9pc3rEDaMtOKGfSqF6nO1ChqycKGmgkoxw2 SFD2SLnyDlLDDqmXyirBj4tWaaniVJeZBMe b6aqaauqp3ZotovpQDVjnFrIsqbSPdGuSFGLu1n QwabRC ZyGGLrab MhLFOiej4vzl4c7kwB259rzqae4absGrpEfOLKjL7ShAQJD6uKDEFyRDllFGMxo7yOGTDyPDn78 ibTHxosl N19hQuc S31xST1vz8gNUArKa1Lv62a1vRovTyAALJJTJTuRgXoo2jDPjXXRjnmPANe34M4fBh70K0r5b4bQMwfJxkpuXeuk7pLXGvGTXMeB0xq8aIOk00LHqGQqBszpT3aPknoFFNZ9FhFT996 KL6CuBLAsUxNBCo D 9D 9ja 7qa Fid8dqy5GksOb6VFa6BiOxLUOYqFP98dSio NWHJhl565xD4P8hp6WC4yBDMY6449XVvoICz4uWauGmfu2l3DhiG95stn8KUOt6os6V9rGQ09SxnHzGQKGIKS IAIkI8rs2 HwcXoZeKiPsk2YU5IAtl2Yg2Y2Rf2YMUUSg2Z NYH1dhWzMtMoCX85NQs7z7uxv 1HZLPDS sQpkiW Jr4Rmah4aykRGBytJkhW2E9gWhsnaCaupMYkfhqY pwGpf4Zgo SE3vroXxiVAzOjbYvkK86ay8 qZ1oiGjZhlwWLCU2zYp8maMKT5kdwtim iQuBMEdyhOBYaOoAa1q0ZiydTwg0TyCl7kmG1VCfBOzWGgoZ3VnmgAW4zy MCg42UAciEBAK1GTZi7fnW5uFsBynaHn7bBPr1fzQ4w2x bvpnXFR2Xds8lU39EWp ZFhuSi8ydZoWWtNOZ1zoiL8zgUP4IJE2r7IJJ2oCIUfj5isYPUtYjDPKXYLbTHKUJop2WuHc9Z4cfkzqp9n 4dU4axyPQK2tBMG09Hrsr2qYcIUbcaciDXeGMSZErFXJWLYiEH4KCsPtrhHKeYejhOnxB3dvbPMp2Xow5uxwRcA9hl HmsHkELQjuxKxXs1SL6aM 7ZLV 0PkRQ93fkDQzxmgOJux0xu D74qh ehsvVRYOZ1z7M9xCwZgQBqcPqoQQKZQVaAeETl8Epk7LYjt1udUNsLBfUrdUC4Tv4lzOaBQsZPxIpJ1HUQArRxp6YFnLrsAdVwnV5i Fn cRIa0gFtvvG5l5O utmtHuvEH9OJ1OwxBEDy5 LTLvKTkYTAczBTQISO02GSSKW1GOYy9zZjmINW0ntduY1MFbtlkILyhESeVL sLcrjOngIoObpqdUqn8HQg0qbn9xi0OIFAAZf0Wn4cAiZyEhXtVZVWcd6IlOHt8 YwVwPhem7AinTB s56Sy2SzkE CNozMvsDkSTjAht86UYE8jcKNlrU9Io9TMRobOYbjHmB3rUWky0iKS4WQRiNfh1 UV95svC20XP3EA5iq0670ycJq8lK0eX05UQzRaRozPRwbyFEkdW 1XauD79kNJsA4a27f9O2PbcwIw1caXTEP62EMK9BA6fePPCq85ZOKldl1fLKv0wrjcrLHSqL309gwNVCEnnoONIv4Pyq7pUodA3VodAxLM7AwS8W Co6T1UqWtp8i9p7ejpjh92x943W60BpgmGd kVVQ3dRyxXKKKwnnLOHLjew jc dW7aKwRO0 hYdLG7IoHVwclM4Uk1FciirBrEUe1D9Ncu16pjz 61Evt rIiDe9fI5kIDh1Ict GqCa3 TgRwYlZXm4E85NE21Zms8fhNJz4l9xw26mdhqEUxOwuAe1IHUiphs0qlGXGahGh6e0Gwu6eOGsbC5oqavMHNyJU0snaLIM1EQa1Gj77JV4ygUvSLMhAqYAyWxiOi2XQZvDrrCGsnTytrr8kCuefcN170rDfE8ZnhFWdp0BSLrrSUrueE9eWDRdfrqc qbQfLkOieuenDkcRnJNilovAufRtQ8QCKk5g70UoF 45xDXK1jPYJrZibK3M1n5inIkXcK5HmBHxqwBMQ4kNuyi JplnsFqjxyzMy 1sWCVDyqDPqSaejqlNIZYmfvXR6tH0Eoo5XqY BA8JW61 JWgFV23grTPnF CDEKdlxmNdFytsXzLe8f8CcEUMqOqVghRv N7ItvmyUmZEQbBeHczkwlx96YvtKYyBv4qpC55mMixktCWblFrKDBUIitwRkZQG8pcEfbVsJNx5MC8ktUSNfwjA0jCpJNgbVYbzubq9txQdGAaq9QInHLC9YuAbOxwy91ctyjDfJbOG8hWhHCaiAu4wqzOqUfSpbNdhDDmDLq2PT3ul7vh1YaJnHjcg5DfmhcMJ6G5GxVOyplmdt24E3sZjNMo6Er9G2hOhKNJnt2LNl1YQJSxyGl9NF0h9BURD98rFY7EN78V31UECyG3Weqs5gMfgr6neUQFbhx1GNAKZ1vSit7Cs BLXa1ZoYyGEOJktu5Mqun0sZoGjguFcUkbx12HOCEDoMJ8xmWj7DOXglCKTUSN86Tx3wJ5iDlbZXTMit2vl4i1sPE21fegccJ7lim LCegrIUsH1gpAi3bwyRJWWJhJl3sWGJgpAlnNXTjXxKmNG0qqAC ePXBWrAmNN1C5EaNP8nOVkrACKsinoxUJQCgDC2bR78vpBmHhg81QchJKgwkQ9Oz45NM9ZH65NMGNT kTlBEcs2F1ekEHeCWQjSii Xg9i3XCaBdHMHn GXcW4ZhHuQkhMAT6T4EGSfBf0xPTOp5n23 iCF xTdW37R7kTdD4zTog fKXWVUfWWr01 9nMEFfrKuSlbeJwnwUONEHNoYsxmueNvgO R0buP mgIojBq j2yiZA94NOnia8H04qa4XIPvWIeexPqr3uVlbGIaQeXI951NhRGfLS xBYqrYsjJrIW6mguKSePIqryTd5UGVFkqX3RneksqCC7J cTPhcsGBLabjWsJHGxtdeK8KBsei9hzZJbH3WGF8P KCtkeu xiMDxukKL8kzTnrPpRLEeyQY2OYBEgjHoiim9R9Wbdpqhg3c62 dBCpj7VVAzeLS bvOMHs2 JgUVfbAtGNmFdKHP5aZ Aam VAN55bdERNU3EZ6EKb9u4PgvJzhGW7gk2UY6EanFEMh5b 45y2Aaxq qS8uKTs49Dlz7VRCGAWG71o8 znJggCpkAIMEt zqnpGP pk6bWP089gH6Q30 2IUkBhEVP YLBg7l7iQWy4V0Z1pYb5w7S3VchjVNRg7qS3KxNHmNA6cNCgpbN4c8NJ 9CpRyPrwYefdLjvYL1BitwYfK841xF1HliD9tTYIPCeF5uPkiaR7Wt7 QLjEHsaQu3qkOK9 MHq0Tmhq6w8oGuuN5JDXPI5nBUInehqc9u43 dWkPbBCR6eHA80iS2ENf82ifx4aGTk hmSRUAdO3moaSgU575f4zbEUHGNlDl30w84H B42Q1x2KzNOq81IoFFLyzo9z96S3j7qMRn36vOdNjauE7qPtw0XiPkui58PWOELJlFZ12D4Tn2KJpF CPZMCm9iBr4tocFYzBSp5alSBL9jhY aFXXRH IZmzluOhJ9eh8lH9XOt5e nzNm2icqWUulEacsT9inIXIsRCecLp98eAIsG5T4Mp a7FxlgFWVLl G0NF1v7FQad5juYkLkUzJp4b3iZm963JB39YhOWtiz7dPUrQTpcnscW 8Wa3PcIj12KUDtESLdyO57Cv5fHTVdVMzwoS99e0KHTRrX7Cac1SWZFhh801fMCOfdLTNGEcWR49rEMAZDxEG7 nqagjzzLJoGCw1a4dXIxXgV7UW22u1yQy vf 6PAWlsT9PCommTm8nilErugDbLW9Puc1AXZFlokHadVWK5NoIVqPxKJs4R1BnyRl7x2p7 K XINPULnski7kd0MwbgjpYZZlT2joshWA1oS6Darsdn66eLDYdcdL wDRxjHig7Mc2nHL2o38rz 2Cy2Z7TRZ4D8Z0ZCfCZHzd D twsWlR4RdaYO217YVp 7PE07c 2xWXyzZMrSRQaz2qqMBz6D9o86Dn8uUYMj2mRo2apvf17YeJHNsAJuZci7RhV9igdpcG4ojG4VNhiME U6AOLJVWqJ6THi88xFJm20mMDWZq vtOwNyinyiv2vtthXoxjx3yfMJ3FSiHnrrN m884ZIEEteH7kyK4dokeZm7C2V5A7 svqCmErauNe6LQPa9X6qwCoPBA9VmqIIZ2n9pC6Sq KSXwFvvbZKZJD PGp8Sd3KUpHY3tCYbq4m19vxkBr r2EPACWFbgjdwfLatvAKYCbLr7RsLzsJWxCsBrv5ZXrDejvr4t0LZs 2uf1WyVgLEFW78KRgx51HLul7sDunbE4h8bvvyS6yO R5J5wpyWk5QU8dVatlVgU0XYIQvr9YmLUB1GOAyWhAS5JsISOmLQRZYxQo2gb0cXrRxhUzqNR3MsX6MMWnoLnSCDtSXMLREDjeb0iQx3wEo SHLP1Rx37YL7FQyJeo0arD5pmFZUo4Ll0W7ahj7 JTV9DJNa95x03nI UPLRVBGf2T 9DJK9 5xy9qZx4ZzQ p7D2fBBDvhaFjvKefgIU2SowlDZQIwf321fBh fWwjjoooBH0gNK2MQjzFVUx7RB8x1Fe7gQgAaceqLavgFO2hanXdqftiYqti1X3JVguiRafpEa2EGx4b4fKy8Wu58 CU3RV3JV2 ET9TjP58GRQBp J71NF 8 gDn0WcEHPRHMBzWO7xWK1qZnOts fl6TSRqjNDw6O7VJ38lAEn8peKnU 1RQzM8JtIoKpoqJpsppjTCsyizirrsmNDIPasTPciFD4tYwTbDjJxz86M2QI3Qi2xHIy9fyC UtFYYJhoyDv92lAEIDV4X3nIA B3aFawiHUhtC4Wh m4eOxFr0aKkPa4WSBPbvYPoMELDkxoR6ybaTJB59uRYzAM34rvv me XdD8NFOHSDF33NLnGPzW Mf4 lys4 hsb Lot4mNE2jDhFoN1DKyC2gL8Ad77ek9aMOTOhW4ZRxVwsbVQTr9 yMVPBZAu BdIH2NgWXgyAnMlFAvnZcJnWe10TkAspQOwVSO9l2MFeTY9G2luO0QNnyr3A57ETe4Uvn4j3UC5WYHBRVt0Wy5TjSChu2B yksC1LhB GZN6cNUyIoMe3dyF9lwg4rz3zPAYtytQboezhTCFBK0FNGzwkPovbhIrGPSR1LBV22ypMVs0dD8C7CNilPpI5WAdur6mSE7WmwV594Tx3CZ346QkDJJ4 eALTCvYfE0GItSk95lFhF9B2Mu 3T6Mgk3 DfkV VkJYzuU1r59 1mjL tPRF9LxDYtvkCLloLdAxM39sMNYFFYXl7DoKmPJXxkdSpQACVSOgCN60NOm0rnEuU3a59 y8oJ3TsT6MTRX8ZO49jNX YtXNoEWeHO1Q nEW2uR l9kBZ0QZOCgLFxGY3Z6 BJxCA C6yHVO3WfM8eoova6dtA 9FvZd67domPkQ0DoiMYbGfqB5tUcsBDTuHv78LAxf5JCVwwbV46 Zk74T3qRoEOKXT4kKpqlRxTALCKof7oPmK87WRtxnPy3LwBtR3DL1Wk597rk59A7k0zRz9fNM8gGbBDfP1IcnRLfRwRUtj1TNgx95TjdxAbeVvu4ds4p9p6n0RIuemNJ4LHrpC6wEy IYc7hZ0HTgH97vPSBiKO9WZhWFrxJncTdcx9hWSByRNDdEy2nWSBOb7k2AA3q1 FhMAFMAFMA oqT VGsT 8X8n7fcUCOjgFo6WfSRxRZRUgtp 0C4DuhJ4JvDnmWCRiy2IQMRNqh VhjYh o3573nRwUFjfshB8ZU0jUGReNFo oUgDgsZzwbBS ZevD09bagOxSX6Gz pViyPUDYSydpLvC5XVRQ9 LfX6Tlh1GNxfd22rACnRVgE6SCY8jtZXAxvSPIQbNkQKAxht MAWO2whILFff5wS4it uzTNC WtwxP38 6EL66HmEDS7EepUSeLXpKgLXVmBagZ0iFVJg5FnDo5YuAHZCIPfxnx4Uv9RFlWz7tK5JYvmMOqH2 GQ435OJ4XYbCvhufNh8G2cXvOOaJ4BeXV2btd Qor2szkYq UIxYMfG n0D8LvSF fN8SJnJqOU bqX8tyC o9mk7aOybDHz1AdyAM3q1md6GFHcL4pM7JLiv2YDdTYVgfuZhBLYHxyStTy8YneQpsDqD8EnypGfTDhN RyIQH9Q9 CrZ9GLkiG1CFsH8qK8JedK1JCQ 9cngI1Fym6JD2EFrXsfksgOk9UZD9KkIhGUe1ERoW7z9lHRLoKJ1 31Fw8mG4ZrSdaeqSJg2hzpGY 4WhesHG8Eh0Ui43B0ENC2kkuPye8FepBEKsPiWcJKxGlv6iqc5dNryfH8s7MA0Y545ZlSssUIcIO9z5E 6VkYFFC4UHchq0Sd7EZ9O19Q4IhxiwrHZHU3LkB 1NrgdY9bA8sR9nV6a7wMsaPmaiMB 69nlR8tq3tEPO6CnQ8MtEE1b2dwWxUHB8kRDhjsAUkpawiAU9P 9fPRUMDvP6gSolJ63a8NiZ1OT1cGXsl7uNJqsX3 GiPOP9AN37ftovWzwT5IF TAe IewllZfzz0DVPoF3W6lKttmRV6 TDdM1x3RhxKFhLfyUM3rWkH87B6loBB5wf f0ydWQX646pkQ6gSP KoidaqDiHh44F6WnA3NvZ6x5ibsNOi3YczSyKhDlqFIh2olH8ODInf5HTOOxf85PG83ON4aExpZ zExArEev3yKhjpO1ip6sWqe 1igoAcMhUYH7dKlPRdiV8hCRiEarL 8XYiAlM AQyE0LiG83Mg5XngVA5SqQX4Pt2kQLNJUo9JuLkd7VfPl lX8uW nbyhH5KU9FW1oZkv8o30HtDYjWPL8ne3CH6ED52NFlh1FbFqbsLp7QBpyDAt6zionI YTiXKY09urTAT6gLR 9uCTd alKY5fd4Npqd4ADcIs1 VjnC3fnr3RtV 90QbPJfBTTQT0gZw7wbJ6yRn kqL12OFF Mdn0JEN782iQmAx2t7wILOlsYTu0vIjyYRPCvQNX2h4GXGpzPvOfnLm VEJTVSmv DmHX2tzLEApi4SmcsdU2sl5BUmUsfp3cjAoN9h0XF5geVXo39zdmKR6 F0MtDo3UD4RdgYvRrM6CNozMZvGvhpSEqPjhoeNFJDAxTo90UYURzdK3U ytn9cD4yNl3OMyYrIXhQrh5JqM exkQSHZ9 Crj5GqwzlUjuVUdCIhaKUw7Tfs9WHXKoKVcs6gQxMq1 W6juaEP6iqqtYkA4mI5k4lL71NpSmPbBgKgySMMUHfg860cdIFHvoZzt6yHYvI3vg9D4gn 3ZZw6gn NajlB7aEZ640314B6ccYuZCzy7OIItZp8OUfD15Nn458DIkzNzA6gV1e79z1nwy4pMg02Tq8EFV77xQDBH4COx1dqVux1bqEGVLBp9LfL4EeRJzEK2xqoHeZJKgiF 6GYnOUMtFlumpE208DBYvJB0giMwKVpTf5E5EWb2Q5r1YudUDnIDuCEb99MgFvD3bUuUrpTYhAX68HMqUsH1stOnUCtDGMoWwWhENhwRUf5U b1Vu 0WHi8Uvu4FKIcDleFIWUN7PQe6PvP4fdJqtJ B3u PnQsodR2E3kr86Ov77jx cIv77jq0Ss56JO0 dUYFivZRsiRD63Npz12GD3ohehESVLy3no2xT aMXpgFHS557YbnB WolfAx33Bvq 1Hn91cbC x kJNmXSLsZRussA20cipExRyUJuxvYi3g9JWNmrKm43dP RCpWykfpd5nVaSH gYsoqmeZM UJoFn7WwlxkK6zXSPoh6NZU52mrvBZUIY8tW872QoG3MUnlA1b8vDa 0Bwmqp9gjl0k7P3 jkzHHfMAMLxDKIlgmWMGIB2Rtp CA9LxZahID Fzvvt3Ge3qQKM mfdhy4slLyH0fklCMoMHFeCF5 ZJruiOGFLOKjPp2FGYehgWUhG8yaggwWWWxYzdQPHvPbXHpOVpzpYEBipF8Hbo5P7fplSW T03NIeLH2PBEOAevB2U6rpMCkyDjwihztNAt6ytgdQg J1Mby42uIcwirk CEeN 81fGHD8g3VMKY3JXoydIu59dI6aVlFJ6ZPl8jCaI5acIFtz194OrOe5HQ9EBLjJNS Q9 xXzb4Ja6 LLgSlxkT7BqcdD8gVRNa nc utc83g7grNNa wOxptt0RRkP9Yf5 5Np PQeAzHrIpCZHX sZkYDvDG32or9TSwAA6rsQByEUppJSDNmJEpHguO0A2aitJnZNnMwuDq1yJnTlplZp5INQyZr1QBkopkpS8GZ4wlIvKQAkZsuQkRmfG2AHXxot8YMvREkZkzeoyBDPbbUfGzlEiZwPNVKsJ5uU7oEuB5DCuBjqp4rp4qpY7e9C0dE0doNKMwiP7dYLjrkO7dQ8ITDyBJIEKMRO4XYL0UThZ2Ws DBpUnnV0mcLpP8lc9gqwwHKzgg EA32rS5W J70 hsUDlYvkpxeHmRCmsVmeAsIA NowmUcfjTMYNW2bGNCaTD8qPUvyEhO37Hv XFyBdN3j6RAEh2v1oktO36saRlHM1XR364H1XRzDC6TooyDCO1PUNjYM7VnZJN p7DUfBJuEfhFJdyhLxafTaLvO2uHeeVA7kpJDMlTv72DIZfND3CXXjojALQVU25IJDITZfJauxqQqfsfXdTvfveQpWdO 0k1YdasiecP2xAAEOYBP0UDHwAxO72TSTLhzoFZfp Z2JNX2tSRZ3R76eBhn oNV NZ5 HufwR5kpc0m0UTOn 71WUSty CnBIf3B1z2uVTDGv2EJuCm0nr7nw0XNqevGqyMtuH gT92vJ b8gzkWyBMjOafvSZntlPrMq7xOdUGYIY 6iadaVQybHqNkTJMfmKiyw9eHmQcJTIMyAUCPZaPoqcFsG7wdCxrtmXAIMys5XXMOrM aSqV7UCWZ7Ef3RTM jQydc6Vbwya7VUnKaGOAnGUhKTFKK0GikgjSyqBqlmz z6b2IiNu7wejsA4chvBZHa pOkp8CHKBuIBIIO7QOJpdQqHCfjU5U3Zt2w3Taw8EfiwD3Ras QsKFfLx64Z7ZBT29aXkB7ca qvzzeedz78CumUO1zzea W6UjcKEbK1qqxYAiO0Zdv5j56 4h6fuXdl1AhgGE0PSxzTxIbgyrgyOLzqT BU2 ExLfCakxgixfXevS SHQ8Eenq5qXwlIjRSCO2QtXqdv77T88DZ52yM28Kun0wDUnvTnpnuwk rCtEZpBH5N21QzNIUI1A0BzIru2IlRcVhgAoszviLvXQMy5Ry jwwPIkJpFYzYV3KCxoBe9zVp0sROs7gr544POjikbr7aru bJ7EYzJMkDIlQpI N6smHpoooggRCtIuoi ZinlmrkVwgaxr2UoEftwgS mwwUB7Kz3yt4ktu2X6aAHFowopUjWjbopAcNxhdolm 5MBl3lhENmB2jaNC8CEiWjbopAcN fm4KFjpzVy8OcU3zKMgo 6k9Y3TZGQKL8c5bvVk5PVxzVwR2WEObUhM f5vLTYO2elKmvFkOgvqLWuXyOIvOQzb8T8AWvt5UnA3 z1lf3KKLIlzwAAlhky6LI7nWouy1P5YmPWgj9SXm4L2EaMgu04S4trZJaNu8ECsJqJYm2PyFTiJ1cyBuzVJfcX7kZutPnLpmgEQhPahZOoEDYNCGohcnCDxkKP0UYEoY80qy1ddTRcmuHmJ4pk1WXtPpL r6oBn9 tlOR8vnA8o7pL24D 8bgqmE3GnH2xFv1KeEqLC5lfZhWCe dZD7VROTd00yPOXOnF AhcG7QSoGGwFd6montx7LkDic6aaifcRR33k SJXiX3jj9BHX3FztgF JXIVV3fsUSpbHKXMblP VYd8Tptln 7 4K47K47hPJVVm9sbcSoZOopFA9P0 QUX3VVF5h0wJVBMddQywsAf62XqXrzgexa8nBcKtcix5O32Z4jXAulri3MgehM428gDIBdN7I5N6ccCZWVS a FRXl SHfJUjBteK338XpjIwFIqLlih4 ni0fs7UKb9LVFLP ccQx i6wrWBzD7qzJdgFJs87QB3fHQfBFvVXJK Yx2yMTz3cItAjZ2hJ7EKZ7pLFQfhl6A6Z4FIUbXpH7CTgN10uu pepASQq9F0RjluFO6kuVQi92UrsnZOI05ays1JiBKfzbz2xsTEXe7bp0sAK8nFk7Vib1j5SB5O UU5hADF7K1j9S6J6 DDr9skL38MOBp6RR2uatFCKPuXYWtvYmD3 BGVl4uvmlZXNdgjLbHXHAW5YhnA84XIhB tw0Eci2TYDSDjvCnWCQG8WrbP Ef75fpm0r7kZLo8ADlr4jQP1bq76mMfMYs8HhmvMKuJQb3 atdUXwosWaPgFXTJou5AWTfLBEkwo6oOHZGINLL4gDe116iIu3kwA7Eek4COOdNj4AZOp1EZ2D IpLYAU4EV1BPwVmLlIRmt4amt3VIU5Oy4W9WvlnMuPYKdQNVFPy6kRmdztH4WkQUaSprir0jr0n5Pc7v i6mf75 tkgdSpQg0Ih uSy0SPwuiR3HMNUaUfp444VIP6pony891vGXfTVjbIjqS1bX2Z0Gq8NHBn8XdC5jdp6Cz4hgVz26pV5BY1ha17kFlfxFwzj1Shcavu wz7rPylt3bJCPl1GkqI5hoEYur7RqCAnIzmzs7BFpdEt6KMSNu N47fUz3i6e xDdQe QWoLwQmm1iQHDcP jh0T60VeYpx86BuVq0weNc3XvKBFxXl1 DJfpBUPcNkEsamHqWN0MiV sO 5mr9fR4988GG4WoJu182vC299FIKIe9bQ AGxVuvZCi 8O8sOZOrLRW z714 pBMRwf7NnBUWBL7udgl 65c30MnRQhXUv8eIVnNELOP34Box8CEJNnCoz bQMMpB A8eg0Nkm3zTxtL7H rbPA66t OtOEFhzZ WulxA0ZJnH1YDl8uw8ej68ln9qlKFPYplmcLBgxS8M QhhF 1TzO6caNUjveGqshUHujuQia2kndDd7XZH6Tn67rj32GK Y t8 40oY28VGA2lpiK9cc8LW9ZJLTJvz7s3gNG3W10bC63JGGqfDy7R15OSphVl3Ujk7tu2VDg TMk7hjt6vYMEJYu3V3jo7RIM5YjQT6 7k4m3wXgA0a01zB18V6QnSjEgwkgIHnrmSPMEv10kG7snyZOtVD6uliKWnvu9XZ70XGN9njEl09qGmGVZVOAA5ljIaYPLpLQjTJ0OxvjFPEMvsoqzFzhwn9YscAgKTpFK4XK4XneYkqpmJhJcTExKovICS8mc1PR4 36pLQx7rAr95Ur1R RbIWdS6Vb1AQ5rlEvnUmbaCsHqrDPQ8EN6AOPnv YXAu1hrRiBMqQ5yVHYgDvYakUKKFwahgan1Ub12VcH QYgDTEcaQ4lfs6Pr2RRKZnASZnmyHbgpjYnJFAlE3U 5yxREHc gzScJnPtE8cYp4j0FWvTz08UhOKmzfTFgWdB0mRlkPFJ2eKmSCuIH0dPnlC5DdnCQHyw47yUgC3XBtILt2HRnw631afRsK8HO9o1oH9V40jDPp8D3d70VBegfjLnGO2Kb0htlGY4Q1cO kcGsauRgqxqWZUlgloNZGb1hq pwnGkxPDXpv4SPA61I3docfhLloE5kUx02JJOsB 2qvqM3jPCZVOwQjK7V8HrO4z49WkFHy0yKcfxhvhkGBfZG3aPrx Vt6mrPxz6TIyjJtQg6id88z1XB4Hxj w6Zudr4Va8l45QbI 9V3gNWl7uk1M3v8JpoMR5VuW3O3errgNXHVoseU43OxxKWl7W3xa KFVlNYH3lGjLFVLouq8n3Ki lRbXf MyK0ILk1KXYggH6YX6g7QRTLWwEgzKKdpllG2aHld7yu TwuEvgIfgtu06rsV 6XJMtZCSadT2BiFSHyKW2 UwNHNNLiGnra7oSa07gl4M5wDZuB7Rog W RJraBT eBldp94EyxMdp7gCd5jo7V HF4VZXo2pyIZEJdo46h7ypkPRoomlI8t3OJh2jQsPIZmJ j5cm62bsuqFu l608Ufh KQwsQB97B7LKYXpMCl5 FGU s27Q532iTm gNM4uZcBBOuyfcPblvuSuAy8NXZHkWjo2jDtAItJRDmSkXXEITtMy7ypoXjOEco0lRqSNqu 4iI awUvTIdDrliHccjKfioXf0UvvO9WUBlfkVFmZ4XVZy7zMK44K 22nBI97mVtgp 9 8 8XVNZJO7MlcV1g5MnJFUijDhSZi3N1zzv7FU8PMISCY7Z5GCSUP CMcw7ts5dddwDxmZ9sfEN7u3FTodwF fJ ZLh3 95LL2LrOzhD KiM H9SUmY HiUTZWvozOc 4IU5mTTlJCg7Tj3CCtojT ws920Pw T8dTUgcVuH9ZpZCzwkzSKDg49I I1pHQi6GJ8VqLbnu6VvKruKbQ5Xk TeqeP7GpaNRuq4Atf hNOv4CS6hoGCHu3s wLWgQJXJeIWF06YAGWXkFR46f5B1pHMVvVINVvFqBcSKmJbr wHOpaufQjoaSm6UUTgpGn8mh9Ig2 ZypGf4Ox2yGxdjlOngaaHGLrXrp54xbu xIL8DvNpHq5aLCv an1ZDfZl7coBy7p24s3gsSCLhJYKtvcwAlPKjzJfRPELIODKkkBOInafbvjBaqf0nyas 7cXUOwUIpc7nSSYjq65dAMmuAgABnWk0mDJIrvQDUHftTZ hbFH uf3A dpKpA f0WxSU55lyaO78ixJbymU6LXyFisxPFd1mvUHYsmeV ajsfGbgijohqUOwuxGIabkdYXeniJAKjBC5qXyeuFicZ7C GLyxklb1yKtlJr0V2QlBgLt9LKkNAyXT84x8GktvIGPyWF13XQwwmlGNW2t LullmgNrKlYCg0gTq HPfp0AMwy 1iRAK98SuMBNm1cBXRPDM jMNWa6DdBp5ANl9UbTAYOguCHXGclwvz2hGSDiwpy89IVAC nouZwpUP55h0rRLYRvRLwxsRLaMi0bPwYnhhRF2trbI8RQcGv Q38gILkjiwOS7BCdMz5tl0vKCzgr 6IRH YMLQStAW3JpfEL 5Ue dWJrYaEuWsOq lKnqrlh7rPTnC9J yJdWTr2pZ3zQKUMkKNKtLJChd S WzoapWK 6ra8gJ8Z7Eelh5xqrSueBpgXoibILnr4zCLPAS4SP6 qSb581RqSLhTLLKdvzNsvOqykeUpecFjYMNqEFBxj2Y5edTxNGWN9P9JB2G00 HM2p9HP9dFTnYv WG ci050M1 qxKfYqie1hvidPq0 ri6BLkBX7gE n8uPZI1G9H7lYmVRHKRHG y9zC1ahAoSshbfQldsTeuFnBjO9KgoeTpGstYDtQWHUdY63GsugopwmLfV2jS4n EZVZ zsvSuKX6skVnGrHQCU9e84dv94DIa9uW67gUs6T5P9UxkM1c6WWVXu9UxtHyhHaXkU7DIeN1BIy3XF NBM8YY5P3h5hVgnYpuVmr1u4El69YeO3T9YfPUO483BSYNeO33eMxhFYFbRJzF 1r6uM0ITs7Ef3N wBDLw0PKIkOwefuw fc2HWzKmlatZddM ggho3wXm7DNsiD2xnw L2KSopHGDqwKi142TAS5ZhzLPjTy k BZJGhfbu94q9AUaiPKzXSnS 3rPaLSKF r1V0mkTnV0ccvftlYxDCg 94lkooZetmbUX1vyt8lXCRBX4e1mbV7sZm 8th6UGVBLqlVItgX88frJTzx2RwXcGtYAJZuTmVGWCHzUf8mQmiWjv5nUfJBopSUdQ8MPlMLjo44NRk8RJbpvtmHtX0vYXl6CCgNLfsFaFfNUuoGwM MxdRnU0 qnDVPb Qvm70mGRe4PD60vflum3Cf3SeAgCPZjiRuN9d2Tarh7nLovUy 2mUzzz15y4RxUa OwPWoW mi6a1z4W7RW3sZfp6wG8NhFp41A04ZfgFr 2Gt4pqZOH PA s7dEtXk14hx3bKjF6I54Z2XXi3sex5GYWlDNIGFfY6u XQ6MJ83Q a1EhPLxq dkDnXsZwilFBVCVpJMxsQflchFlVh42SdYVNxOkFNFYEXLoPEWUDpFI egC8T9dBYNxWhGUpWGERjA9Z42miPNu2mn4wWX7Ei4 d Vgd8onS pqjRPl5Ikb6g7EvF9v8v7eUKUaVP6lEqVIfW0DgVQMtciS fiwQ33Q 2n2Bfjy4ffjwQ YPYB L6 MOASW8vdAZK1g318v329rwpGTxaBW A4I sQCnGjLncFJKgnKfeAKgnALGs3e 5YY1GSbFG7GMJ8EkPJD tRkb1 RtWuukvOqlMOsU6CfojQeuzMcZwDxVIYuqRSlkvXPovHvRSRVizB2DHqlwTyYf3A6zpjQSlPEfMTNokTxHr1UgQPeMyZg51Yo8TAmq41p6d0Yqy8nD8JTipXxIGNiwTvhlycF8HCV5jXMpobsPnpS cs1hskWT nmaVNXcdGz1gs7Vbb2CvpBeVq5BxFNQt3wTH0WbGVTPkdaykTym0MDz8joM8it1bwSJaMt63Fa5YUwp9uMctPzlt vgUn5Peq9kKk0u6yfoCPPNQwMQelI g 27Plwfnn6Ju 6q7SF57ReSDPfsr7KECjPpxE ZTkCc4aRdSGwoQ7kQ4KKqmbfEjiawspwGR6Qn3XxVh2rTptAbyQGHAA9xMzBWNB79xMdh qv4qR07VEKUGWVHdvGWang4k5jAM8auA3GHxLago2HUwrWx8GZNcbdLwpeHgbiXhF9tM2zGrxXfGJTPst1GWy4wNvcdgcgNWguJJq6P270W hJ05KhiN mFeY8y2g2FcCd34Diddn 97qO5jdbPlyW4AtS6xj0YrNzfNoKJRo8yDOoi7ZuVqvXQQcbUPI UobiDTXX LikV4xeFfNTRXGhFQYvRG8qBgvuTHAnBlzoa8GA5fuMty8yde0BRjAqXsIQcal8GFBTryLCa6fe oga MbMTndOnrExBqUvf2DIaRJZXaMBYeLxe cJ5s700LyXlDo0ECW78GrJEYZHPIv 9D0TWlRoCYDpHtwCjZ WSekH8NjEEqNNK76JFOby4G7JE0X0H0VJrR3qr93cH0hTVaSvei3ZbTbXiUN7QD3 9yxZnCQ8JEWaiNWP4DUK2q Ld6 ot6wUnIuOo4gbnpaOUU3cUU rg219L9BIomk zs5gmQfCJ3OeM iiS8CZYgpNV6Xj2bYR x 19C34bM pVXqpRXqFwJH 0gd98ELFcQeTeZcPRSn1zbuNfyANfW1jNfKIu54h6Zp6fhj0QWXBkkGLQvPSaIcqLyZhGCwEgCw2qOqXT5b7436Z2IvGMb0Tz7ZGZwJr 9hNzAvGbuL2VBZ0N2MzYRjMPPeQ11FJEEBMhFMyEIwHNViXePBv9BiM7VmbZDGmtfBHCf23DC3hBN8 MiP9w4Rn4qZr6 ifVgHjXhxDHA0SsjhomslVBQD3qVlj1XmXoFYJdlfl2bnwsAKxXspCeyh0C1wjK7VkT2kPlayHG68yd5RytrqD3sLkQyJNXziUgPi9gnC0BNZ3clgmbHTDd 723sQ6TejXJWUN6pLxDMFHRheJK2xRHjtGrdcsOGQIAvrscjaWzxxTKEddLeSLcKWgIZ5CtFa47sZ8l62hTY2hFSX2 yej Fp8dHLmM12r4xHG1OqqHkU9NTyGqUpjsSdfwJH3NYAyiwdd0SVPgGqjno2Kg2 PwN046eKgtTBIdpm6YX8MCbn1n FZ060IeNbuxq9YfO90gJK1K0gSMI2WJ3j8CUI5yKhqzootzo S3z3RT15DC8iLrZXc2sSxtIDEFURUYnPjNxK0rNshbjL43KSTIFt2zfIEXYwUw2swk2Y1DSXwVT3SB6nR9McdgJrzu2gSyiTJ8hW0kJYPHr3i7IcLcJUB2GwM3ZZZ5fDops6OZrYsws55UGbllP889ZVVNFp E5PdYrwwlLaP4OhLPUrnNfd14DYGRfjYwb3yPfkd185h1dygEbbpTS5y8v1TAbUXTcplzcZqYHrZlhg118POF80wglrSeJeA G sFymHz0TucFLB2HUucj7qiI5G2YDi0JAH7qObWifBr7wye3l5lg7OL1jAEAqsbBKU31xk DHyRMd6kTCIF2DZSP5MZvKux2LJl e8r8S2jGHjGfMTP0S0DJapE2a1 Y5WBJnAUazzYgEUc84z78Ff2QvGpcXGieP7 z67wi6Y2oWxQPFf6PhGuhT8f0kjeYxE1O63NHWNTawTmsigyZZkJyqjQa ukqgdSyGHiM3oATQm tBXkZNuu nyr5hTnVO8nTRahM0sGUs0FOsLNVo1Qjkt ZJRvZ7sywFb43zopLxM 9e lnSm9LIQQujOvkQQuexuuvqWuLkjffuffKvnSkmc7p84wSc rVnnAWEubxkVmLeXOqQkzsbEliLAB YmPuQonQQjfD7e1Q6sXyecBOmZ2SyiMXOa48TvjiO39nDRcG3Ig7ua8Pt67rlZpK 4vnRcgHqA69VEuRrpQMhbn7fI1HrGHWcaHackKzgyWIlGblyZUa30BWNj2IlbHfF6rPB7pV E o7ltNEwBwatOwJydv8lg0HdyExKWeNvPeTugnNkTLhAC6gPpoysf04HOaYZH8grgv1TVrkyhbnEBYW6KLMxGxqxp0ejzS19TDUy5VEGujtS mW6q0P0lQ3RDLbFLFazwDsMvNT7ZQtPZ1DZZQtuBcc7UhGbd0W3z5OOws8xIEOLASHPAEjOSTZhr9MGYNQSr82AZ1GF7kltQhj5MOKTHg1SWfXmlSrAVmrgn3EoD AgAwO2Rnky2j wFM5I1Rn1bOa6iF7MlGUC Z0ohrxGJ3ljIojIE3weD456NIIOlvJdOi6fm19xycijP6hjxt5HUlGQvUc6E8ztDjbxcZEWSPdmCI5ZmTRPDHpME8FQM1Exov5ToBudNaESmmU4PRLL7ZQl9 omrdp9GoIkgs3MDLu5qnVnG JS6MD9C4Ta7jEyXVr8eFyX 3louXVEcW X93V39 jhCWV74WwKu8e jWpnk6F9rQ7e25sNE0L9fmP7fIBS09YDqRTzff qMwHtlqZXzffUMIeKfwKGahYjjsEIKVdqYV3DMCTu7TTNc3L1yj v6aL8MpOonVGxglj5p1lPTUGnvvbte8eb rUG wz8F30Ey9nva3gl4 ED1cEOlEoR5I DikCIcVjRWDOQtNzr bF9SJKX3kRySoDWoBDAQA5f3nJhlPtpXFMOtj60oGiXfcSIVBnso WoDASc 5A1k8SmQxVL0T7iNC7sxT0TNy8ez2m 1JNozsPXozDseIwheboEV1IiCglteA7h4q6Gbr7kY89GpFcRddElfyWVXcWGrBnbTt6CSXwnJRcJT9KglP D3MRD5rOMPiV k 00CGNME27NRU6XwP46yYwjPDBK2LeiVllqEa 7KWxY5ZVv4Aei 57QTQZ6IMwGyvNOW7 u4VEKDdeo Abs6lTXk8HAQkCd7po9sBGUEqfPDFf30rvTCduUCc8c08Qc8crrI7HDGicL04MOstp8TDdoPif6EFBLjSRAZVTp8OCLQ30FbsxR1Ik43O0Xzjva4mpsyxERV9w2mUTv0Ewswzdu0kI7Qu3P61wpWwfK1qvyvQ7uOOg8rc zrvyHQ7uPVR JCkTthszxQb4Kf2pOscBnnah8nZ OV0QOQg8NdQ4FFnqsGSg9NwcIaK2 A1wG7puRwGBCvTCqLAqbIbKRlffL96k7szGGsoqNSMCKpyUDhFPDhj9o 8NfkabYWY1OVauGcKPqfSu XbePazosNIJlqKceZWe4IhROTFb1roJI2g7Nmi prB0BWBQvSBC3QOvEFEPHjCIuQACQtLs0 pfkJhO4 P4pgP3zDiA uWfq15JI093pfq6ls9FrL QM1khuS JqZwy0m55xfv0 j7Jw0oqciVr5oUU6sZ6FuXygdQkieP6v5ii4Wi8uiSfmP6tK67kFQtK6ztb95tDArQZOEMtk2p6qh1zlgs5eTiv WFtzIdKgavmIuxmf4CRi1aGxFiIrK689ksDHIHqWirkWxkNrK6s92gIBQQ bcL5qNYitwujnOaUsf9E1alo1awujsJgASr32ZEBMnrC5IEF9OnmLbe51sRrE0ObD43TdFnYtMFpKcJTnD4 5NXRhVwypYxVtVNHXQz25xvi R5E287o3TRX07XM5rfXzzeC51RgAHDD0cZmqqH9st54cXNzvn8cAzErQMeV4WAk uvAb705ItWfeCrDmlq AZdlO R1jS4fXGfPwyuVXf2LEQjhhdwCoU8wFuFvYyX73HBSO5pUVMy7nY2TpUVi0I2bAHCM9ToT4Qy0IT3kys38lME9i4ta4t pNY60Illn1UGCb04ahoiD0V DHZAd5LAV802bTdlFtBi8qbuAluxksFvk562LL3BzdTljdfWKyQ0R8TBdgWQAPWpKuqOeekrZJs459qZ4V7Ln29tz3IZfq4PZgyUaEnmgdZk5Yhl CoDnC2ddgJQJKR qIJKATfdOjugcLPL5fHIbImmlN4LW4e3oti37LlONThlhEAo6TNgGtCTEiCINQ5eKruCZmJOcwijhm1PLIRJ ylUtCNz9TUNjDfaFDm KxT9zkcq7LXJBRuSu BD9B4yHjvofPYPAGLrspuvCzKCOw1ulSrGHNEkxRYnxnA1WB0QQAkZ5kG9VrUOuYfLsE HMKTaE6wWfLsJzliR GXNj3oczxyj8mn8Z Wg4ji4MK8ee1IuqM1yesrDKPHE 21ItuMjfGy7BaIojtkEkEGOTJbjhaTe4veAuP4xk7tP4tA8BmHbJXezg65wi8D8p9odJZy 9 LgBvkzpchzPRr8IqimhElDTh3cx6ASyKIqtfFVry20BYLXRedkMh2ctsHXD0fJBo3SUiD0frB2u7ORh3OD0vULxlFYy5tQnbfJDZN8XF8 4yh75g7MF6adCZdo4tSv9evqhAIfJTGZ9km 8gRJ56MBBBg3E4z8AL22wK63s6tBIM OqZl2tF1TCpEi j3AZvZURqn6MtxDZ9RnsUZ3jgD ZENB Mgz3XXxWBCIT2hAsJADEAPSvoGQxwUZq662GX3i ceaFDBvaddEigdeMbBSJQQA6fPnQrQAFOp0vQ7l9IuFyRuFivTBFzZdvQp0k 0wQ6FaTk7dRwP2xR6535mZaZammdGTjoNOOeUmahgMFFoFp UC naS0njFbJPH7DtfoQLUsl9M LRJw4RMFXzwlkNqS9MtBlCFdXw4xZiBKXiBA7JlLzKfjM74s7MXLpxgAo9l 0EYiNm9T4fRLFDb2Ac3d p5kvz 0OBtcEP4DjDvtmpGoJpDgFJhbsrlEYC LBCJB kVYJLjdsXKuMh427DwmZQXpoUR7QnpW lG8GJ0ppOUPKDJfCdf7R5k3wMkZYM147PdKvaOOqORpMC1xJrjDClEGYaOmGbedyFNB7BDB5wAUhSP2mGaN2zJGrkftx 842RZ RumD72HVlmfJzNRXQvkDa7f9ASIMVINEjMG abodkZkgvHcyHeRe2ECNzZEyavHc4s PpXWvWTvX8sW86HKygN8DHwxjo4FDiPeaiagyGrbsLIfamaubeAXnmJ5aaYquzbgT0eXGuLvFFZazbI2OjqkYqFUj8OvRUhRLqzZ soZGkDPhtrbl6Myu2NbkZQehMxidxFuOvbv36vDVE 0lSTR83FdvuMX vFLENY8dksPm6j eYq5lu YPlFP O70JSIBPIvuCZyHkV4mh6j oQyfJ8ZoS3zfR9ikRqhRIjrPQfORIHUkMEEvmmm90Qyg7nmsbA 1vNE1bVBYI7LnuC7SQt9Xp 1nmZ4nqmJuyMXNs2wh BC6JBi76dPE8o45cCVDhm CIVoi8aWfhapQfSbWEaSbqAKrmVaOX42unguIrVOX4MgGx8 9OgJ3wfVWwTz8S6D5NptXduNaKtzRVsasfJTbwQs4oyadT12 E4o2 YS5eO5djnJj6N608MAxNMn69m IEKtUT3nG9Gj12wC6HTyCAat2j1FYm6XGnX56x5VtTvP2QNj2QjpMjAX7V IhEuxQm6VXqvLrBEdp0kJMwns08G40kaKbpdQm6VPkP4B6nuIAUYDaZmX1QwK0lR4RIzfqjko63R OpQzwbE6zN1V8 uEFAuMum6pS O OMD7x2uqIl4XH36Mh7z8Lzlu4wOP0zC6ve4aCeaiGaj3ejeOk2fGUhhxKKs7VKYCeCPq U4YFC0hJXsNflOpvF5iW6xXb1fGuosVSG 3YaB7sVzpsrUTkksKelzlEKA2te8qvuKlFT QXPit2zkcXxEebhJzwdiICCdWUnetCeZhsShbFs9r XGIaSedsktIqoZ4cDnkSCIMHiaBPk6xH3L7KNxEz5EJ79MYc5Vy uKzj9k385KUVLPN2cbAzrGJTOux0vOsvwlSfXTnxepId03mSGJ6OQM4BKVywEQlgPePtzIupNyvHurKYVMh 59fk3Xl7EWrKqRb6DUdbn0gUuJvcbGPuDrDJzYAFWGjiGeyQeAvqiafP7BbQAGfob6AWOWMK0aSnGhHL0WnqK0qxBefHmmhxt3NbloAgvqKLcNR261CgsWzo15cXh8FphItr3C6PxVED3gNEMQZo985uNEsD6z 40tg960ZLQYB398pyU4jQQTry4t7auysNMQTBtPLyQ LWLMHx bCjBZIw GstwNKjVjR5GeJX6xjR0e7s74sU96iGOxoLs2jJe1IhTr6c6lFHOMXuZ4bHzCMG58VYgz8fQuxIDXSAzgpH5V4ZoRc1O 6J9QMBBjvorBNXfSybRuCBNHdmEBiVhCyGu6cq6s2GX8ZRz63l7waKbjVqAbw PSWEaBrHPSuTyEb7GVbKCqa Xb3dyDTvJK8EjdfFH5sAf SPlMfF8Y3zn0DMRwEEbEB43hfpreE4FXpHdhwfsxzxwbRtzXA5cPpITIvYbdoFaFd8RuA09owZqY587vAF2ILYLLnTaN1VTLPT8q5dg dpxPdrpi41IPu8ez4sxCx2SONRnpz5kL2TclpFYJkZIocRc53NnlAKP4GYwTsA4ug6aC5WNY3a2ALZtEwCQuGpclFnllWka33MDLhD6QcLwmFw2wgOFh5B1K3xWpMKhym9KYxTPFfU1oL45DrACqFRuyNcLnLriYZa1HgLgt9LBh2H6KPX1nfHR1f9mzDtZM6K CdFNiLoNnwUuxr0hVFcMv2IPYQl2L9ZQ50erpYh8 oRIW8eNuyIl9VJGDZbX7ixZdMfJJNJyYFJ3pkvrJH84e7ZfPQE He JamCyRS8qYoOjIrRd3J09Vq095wQRn0Wg N2OY3XL1UyDV zeZ8g18Ex9FbhEHhiLktEYZTo 4N8PC6Z fZRUbBvMTM9kmhS0w73Y2E9pX4ob9lJtPdXCRgmpbPtGPg23LGnlOpQH9POJkW2NFohwW0d5qQLE17OHAjhoCyVdcLseBULxoCOeOBHMToPvLsBkukt6cyXtnhvgKKa07O8bPm8mEy8W42tdXrizvBWMEVqiHj8ocZXMzV4GihBK3uporsRE7PfLv1I0QgJHgrgcjjti8uGyJWwqJeqTw408c5lUBKI5k1KIoLVbnPmYg 1 MIVv2T8wwC50PZ28AArXe f2MdhoVcpvldEL6dHX6cRfzwYGSvHHDvGyRcf78X0b9IT9yB6KJ0nB b26zefpq7FrDukPSKslT8qQ1wkpaqAmvFB2c8SAFLD0ydQlS701N4FHrV o JT6RxiVG2Ebd PCO834oCqIxPKln6uq Dou8 bb8WC7Wm94O9 K6x xQZ4P2xt2 EXrN6oI2rFPZ5p4MZx27egCo fux MjTXVsE f2CZOQBwEVwIZX58Jm0mNVB3E09C m0P8Pt5 cYu W110P81i9hUQQ0FWuJ0EGKYdJUDQRDKjCYniN1dYz8aKze0eEGkq4CqGGFqnqFOryrvAe8Mlgx8dYT32zuNxdMV0x40UbuO7I4Ytq bcYySTO6NsD9o1c0yBlLF2GDBPuFnhZDDTuT8BhD39DTG2MUrgAkxZR8yjVsoWYyjpb950JH6dUplCPcfyZiyb2OuEPZw Cp9c5EPZyVry1IwXwITkXKXgZSABYENE3KTfjQJ0i7 1qIQ5gcuhXGHHdpW0Yjen5gaCS00NBaoKBi3Z yYJE9cGBQN PUb4fe3aGVW Li3aqsyaSBaazgDj7NvS68XMCaU68MMYvFQgqtMzuvd4b PEnwIgIAlx4NdvuhCA8dhXalna wi 1Rt6fqFYIhHsjdIkygIwEEt6d2dwRmmnpsjaiiquBGkrblPAj4bxk7dwHn4d2XRpqFW OpItfAv7x 6IYkUUPZ7 iNhMa D5Zy9CM dTpYrx6 5Xs3JNPALo05inPXQ3ZD3Kb2msXnc7TlB2slFrUJr6HXVt3gQJrk2rzklKVSlD3rzkBPDDtk87rJ2jfLmqxN 5Ut9ISE zxmzLcpy OAJfw5InwiGF3bnew4QkWclYMwhU2BS1PM9uZ bUzF 0yz3nqmBBFCSKUlQT5 QxKBSSyvb OTnEQ0 lEtWV1kHpoH3s332cb02cp65Anml9j1nlAKDBtZNd2UE863LtjZAWGx0UUfpRrg7WGcn2ms4lImI0wks0rWrTytZKGyG3qr2of81v1BRwBARFC toice0ktBqx53EWXhb2kTZzEVpgCaFcsxMJEfnSxiMziJkdsoquIw9PhcNGqhiKHe9qeDO4lbFRMVOrEtNPehCGH h1E0bWnA lfhXREewZFWJelcJoKa7YKJ y7ihGPU0N VGJmE TMAp4aDoeGkFytCyc5GqPqSJFd 7Jo2Ire3 qWWILK8YhWze98Nc0bXu9FnXR9j ArmaYAMyz3Eh9fdS4rw IGH5 DZhZ5 Zop7o7ujKON uo7lAQx0eWXghOuOjkp LVZW40y9m6X2IkxNoj zv04 S31wOnY5vyStwFJ2AJ92LSlLIWzlyrDx01XsH2a6QT8n5v38L6wnJgn6sS8OXZijFPzBydTDZdTq7win0CPqAluwk5HaPVSpeOuYJTAP5bbTnKannWMsPq7BbkD Nr3gjn3KTSS1 BALxBjl 9DoN VVZ5iekEPJlD4M751 27TJ8HIVDppz7X 3AeSwzSXerR4ApBBZyoTOeA9AAoB7hOq22ngU8lvJWxniubnJKcsfFh1AqZFNGOxI6FbGkDcOIFPaOo7xHu7a1FyvG6Vvda1f7na1vpI5WJ9FC LwTK8a1q9 a1Arv1bo1MzR2oFXPN Xsrq0 si6XgDxkAmAsNlSH8q0Wxa2DitRLeGf2DGIZfyAbCleTAbZId06dbL B A6VjydTggtIKUnNrEiBe9dzJv9YBeVmgHjcBxmMxQoeDVY 6rxshIUN5dI7l5Jzg0i09mBe6AbZ6cAye6MrNJq3Os3maelEycmBjjDTOKwnZ339pHgbngS6WUtZyQ2i6wrgrhze9li9Jo4G9Gy O 36fJGMkJIM3Cd65b8daX6d02H87QxR6K9nnibGohOF0xrNZAUysc0Z6E7DwBPTYBoxyPZAU7 o6BHBawc4x5F quTc6RwHFFuokdEikqWumxbuvdOQg0ZU7jTQRzGXra6cuSPiiP21vYK5P52nCgcwQKRrLmlrCpcJB6D5YxKo5UW RSfzkCR8PtRZsn2u6k HH0wI0hKmx LKovlMzB9nvlg9n60ka4uWNC2m7qJ K71r3XiYrSDXy FKxrTJ5 CglCStl2SNzUdLheUvxD I8X8uZfmQiKTzhVvxcG5RmYZcaNvg5BacS34aOracI5aRgs9q4M d0J7Icc0oJVjpe9TDNNJZqIIPsPIgB5uaR2rNPboiQY mvuODWlAnS6dhkfap2zW69UsRottkwvWyAkGvvSztfFN QI aqd9B6BjyNnHFJEaaR WwnM2duWyAqWTUaRVAdGvrJP9Escb sZ8FNep5u43bVWu43f 7J KtH 9Mc QtH tIFZPW vtI 3Ne pmf JWV6nXFb8i3e NkQKaZ08HtJtMJGJlobDY48vEtMbgfFIQXU5r71JDllU 0WBSBM42lrE0cE4wMqEgJTbmg8Cf8mg88gcmgkOS3PLqfAZ4Sxhu Xh8wsd0ZZLmmKjHNEaqMiUHFaB4yHL4J4Y zeVGTgkOsbXL9ts4kohUoIbJYEyPgAGB5 BuKnnvVAz 0xXS0QNxXpn6JDnqTK5Ig1Y 4faelfG8jVpKg5o8mm8SipjKFHNCHNmHN9rS13hNTRipW4jp4zmg6QL SNO8 7gvPhg7luJEWVjmNHTtwArxllp9WoYspW4udfzegCBKCfOiY1 Qm KN6QFAg5NhpWc4fksVMH2HJmsHI6BUWDXY0Jkub wvgJdw7oOm 2oxEoFn8aEz6BuzCkrppaEeYVdOlKSpSweJRSP3LTSMFZrvOwKVzpWRdAy bkcz9x tXl yMyztzGoFbdb48rtYtpXZIfGEdrrNMZiu5pIISeXPOlDlG4tS1v7O5N z8qqtsLBoJqXhAHp4gLZnCvkbH0r nzevlT2nHcV7ZAB0SH rmb84kAyFH9fpyi3HN3GNv9f9zGkcf3v0qUI7AJnwnGpVn uH8TwWy 8oVbFR7jy01J4lKGKOqYf 0DQTXdNFyGEi85a94Jg9xOzvx9eCn3UIeF1Glg3xmFRhjE019hyv xnFEBRgvGpH4 bVpZ8g4kMWNWp6mztsbe9KIL zZxKLKcO0u1hVoX uvFS CeP5Q pcrKnP4tKK hQW3d4rcFhGh1WxUH 3IYGb0kl 8W 6jQQOu5YeFhc nT1pUNTjzHBEwpJz1uN0wR6PfG8AIpuQAAD6GwZYq2 dDDsASw E4Fd4joqNt2BTo1HWhmNztZBO2HaJnO1DyscA4hgCZnBJhgb7Zo6S5ibsHpRrWRyZVfA3gTJnb3STxUyzLkwjzIvReXnUy7m16xU1DBNPoVA7UZekjrPOCUD6SKiovZNLodpL0jjGL8hqnlbSSQ7QkvI7kz0B6ul8JzE4EZVFvXhQk9bAJVvM4Cc3TroIJddsX2F2CuZnB9uYb5KhKpN82MuUyw57gKnbiFCWlWAUcZtL3GDwMq7WAvSHkv9cQCcvcJvnewMfqP0JpsIJo7z6jNoSomb36POvgaQHQXB SYyVeQOaurb9KIcCDqYIHxVuk KkmRD2vUIcyTHz1YcoS3B9b9LqtmIzct17uhZ2rMyezIXliMyeCWEWZeJm2BhrbC5yBzEdT5JAHVeY4JiJp2Vy8XrETZiJp TKy7OiLwLH2nMTv2ULfU3m zDA7Op SZ2roTy2UzTKv B6LnpdNHfjYH2R9i7h1vz3 AQn7 vIT8MlUmRxl2yKhm T7f2psZ934h FqEwwwpl35zY y3Aa3PY4OW76YPY8KlQU7gh7RPtlhhxsS dcsSvQ1jJfInR0ME4D l6RPhGeiF0phrGOVME4 qhyUCX9zqD8WGopY7jn3KkdfaxLSi15EuGlz8ROtUeCjCj7NkXbRD7LdTYY3wADet7q 6i6LdYOfS706cAAqgn2tJgKrObzK6HLxdalRn2cS7WKcqY2u aT1iSwnBmYbXosEqir ZenI7SRM fWWuE4J10A2Cf68K79v9KHGnnQQ EDg6xUQ6F87P2Q t62DQO TyN0lL8O2e9gJ 1x0MGoTAZIzuBcIUNU reltet740VxF25kkpILSVOnb7rxX cyoCAtBq2pbbzrq2EcK1Lp pxrheUooDy5HSj77GSV4zkKftBziVrXC54meSqgkwUboWlRlpHZKLMcLN85be26Eh49KCbsNbtHQBP6VYtMXQgAEpa Q9TT3t Lby Vy5NhxOh lFDyB5o8DXRm8KnJ7hEQPF2AGFEep8cVVrLX Xawaahr1U93YeqkeRt04y5FKVUo5taVaaUKNOJ7H6gPsErWVQ9Q 6T HgbCh2JFFedY5bdY20UIEOJS o5KTKNbjvF24dgosWE3p7sQ6ypwDQJvTXI1J yVT3Sq nQk gKs7dGkKLPnkKvziV1hjy3tJsKXGPHW Ihp7IXcwAj6Q5M0H tIfJW7owKOR2N7rg7Ibt47 ZqA0Rxt ycjopYKp6zCeEp1poc1YKcjclbnxwqhn0fepzKq8KGkBU98L3wXOUKAoOFYhQpJbpY3PQPLRE1hxxKoAKz R1A6 H JGPl0OVSKkxMV ykcY4uBmfbNirq6h4Kkcq3aBdGDbnOeSpL0YDzoYZJUlteAxBBcLps BcLi77jVJJOnQOOHzLRuMNA4z q5NeqsxTTdvpFDXvB7o4vNbq1cuUvLrFtyysqW02fLu1PD09g3X5conBwnjhO6BVv2Tx2RFzW0p97TQtMbfGFRTVZBupsOQIo CsIoNjuwQ8NyEvxh4S5OZdRZqZT41URbclpnga4wTHFLS THrJfsWVE0yaKfe9wmymiUTqE WlRuAh2Puh5 bdExQ45Dz zgGJgJLLlr4srcHzzt5HJ7jB04p VmC369UEEZ4hOFPz8DIzYKz12m oq9IpQx8Vy0l WZ1dIluOFDVOV6VVRs5ZMejlaswGYl9Y1Zmag2VAEbIDxFTCcumjD92J95KKS kLcSy4GN4om16L S31wSTX1zAnu1YMYPPPkkYk2j2mk IKJVWH3z6M6qjrjkfzrlsADXkUMusJiPff5yYHsKQBSBlcs9JBTXcSilqRtqrrrlqlGbYdBCBAkW28srsNnyrYaNlTXXjzx3TnlwuGcSTOPN ZCHdhoKCOYiffP5HjIXC4zUC p N RLBkTM E3 ZWUi4gN18D7XRxJ18nwZFZ2A aU2ynFJAPJHJ420Uy IkwSNY95lUS1WrwygLjvV5xnTEzCoDiXCC7XTNglk ngEgZG TgBlo9Ox mS7LOl4E NyQcm1M6MEJt6ZQ mCsshESA3GHBWW38XWpPJbk09zkYmCHfQndpZ6odF9SywaMuSXkMfAIHBuml2Yumlw7CL2G h9mpXWt4nBU64kn6pFViC06srj9wpkANYir0TD0dWKjdntsAYdCallgRqGtvFoR5XRGZYVsTcIlp5Ede71yP5nKoc3z8by6l1a 73BDQp9xyZn71TxRoAe2lN5PmiRgHs26lz1BIu9oZzMC Ew jQnzn7AkzD1glORkAPDxAmwq1us8jYOFw1Q BU0zvM3lGLL3PAlJcaysf9Rwe46gecxhUD4Um8hqErxSs4FoZ3pc0UdpC7qa6YtQ5FemInu9HqRaH8O6kcjHmec3o k6fcjQMqiRbr9AfB43Ci5B4pWjUDKiN7t K3JqLn8R2I7AYQ85uyv XIKXXSN2vXelrPlXl7AAnU3TBVUnkapgYNwhRT9xvR5EVTt3B HwmXy5PdvBgiZ6ub4rBTe VgqnBNnfN5RlXprmpfr0oKhQ8rYUPBIzJTk297NWMD0tkec7hIZ0BWE4xVSwDQqRMdUg7rO 1IZ mXEUVyMYk039rxi46DNcn h1XiWXsRQf lAmofC7rL eKSUyyCQI5OYUbPTKIMk0qFjfDwvVXKw3GmP8oxQydP038muz7gU AVDuoO Pc7puxhXInv6 RQ2suRxl6uyevVjDYp5NLAXyjtNmD8kN4l9vFvlQeqQL O6C18GsQ4bvE0JOfm16rpX0zDAjMoRo2PQ4MNQUnOYYcmpOtvD4Q2PQipILYFXkP4aOXa0tQLsWEiYBYZNKnhMZ8laQo8n5i NQy7rhssGALBpsqel8DDvXzwDvpMzq8K4NE1p41qUwUwaT8P6AFCdybxS2hOvfLVw5 kw7d4kiUwj6xuArx 1QrRm3GQFGu1OYA Z Cuq dKEhIQGiO rzpe5Quha0DUS3uc3CG2c3qD9paLpK qLaR HGcl B2fvObR HO2HsQF8IQP bOVG41 7NG 9NsOU2hO3HGMj ezkZXR bVQoifq r4Nh56FOmf5f1crqxJsT8QRcQQuwBVcR bVMcFuNnFToe0cbPmbVIfckGOqZJ8GchiLpe0lzGRpX8XxH0FGe9FMjaTqQWOHiD3LPjpszhKkpinUvBdb0lvsCyRk7ST8bOxzTcF yUdINVYL0kiGaI4pODS3Tk GEktYrTS5z57c57cQHCVSP5fiaLgmKgiT8yqf4GxtNdkulXKm0coZ8WPRhSleyBkhHRLVkwpjCGey3ocpJvV5hqVR85Oohs3HztoFYzphCFMmh8FJcP6hVSnKIpifOUgO1ZZx6eqx Bb w5 KJmdduIUzW3uGulOD trVpP3XcVeefNon5zXVmuSXOOvZCSGx1QuYKCaFv2BUQ2JflZWWASB1ARV9s6G0DT3EeyIdxS7FBS voI2z lnZ 32DhCeFb2Bvkp tznXmJwmw8yzR92Mw6ZOIEO5xuheX0YZUQeXWz gcIx4m ZuxqcN5FoA05m siHgCM fLlWGZ8lBisWqKW6zTk8x2Wceoi7Fjr5YeSpEYLYpdZ8bZwNYy4YBWAyaQktaB8Z2AeF7yRvr5biHfP5TekFmtq60AJvNaEtd7FAWNxd5ZYkraN23edoE8voZ5RJ9plS1J0ARXiT3XG0AqAxxiTxG0JVuJoWkJmvp1M0wCdubdUnbLRL7FI88uVLbxuS93cxdVanRbI5Dk8fAFJtn8WRv3g5NRr 5UHVMCGWPBMZpx0bKpUMVi2hbGJmXveayrBTuNDVVsvnWHiGQgiu9aVc2cGO3qujGthkvGLk5UNjbUyccvrqCf39k4teaSVEoMSXsgpbty4FHx6OujgK 0gaj I QscXeH7csiMu4VtH lC2tk83PkZgH8NQLhXI0p3YHT0n pquaXK90J4yP6joL4jovp9aVcBfxKeBOutULVZM5utoXgGutoXMYAemySrxAVvGdvu3gkUOwrrCrDYLdp 4E8Z07mzvwLFB HB8stYEzt1jXZrJvibQ534YrcF5cxS XhwfcnWKVoFtqb0UH3UGhrU7rFYOqvjxSWLiVWHd1UCCDjN4CrX1MCXY8BZRHLBanuAGnQhOBU3EYZi53Yk2HIKFmn AL5dtv5SRQknWxxb3nwHHZ5Bcshwp5Wv0YTxE1YDtyWDn4UgCx9M7 pQG72ZfgH0B LZpZmPuWD3oX9UowLqQDMwqA1LhhvV99 SQzvpxRfw17bHjyjcNQxazGweQ6hgL Ms9yIxa0aeo GOHpC4bm nezFnECbzqdRxAtSRgoTjQlsjrvrkqUtO7iLOQJuACBx AA12L9nr457E5Wlpt860UeFQn1vZeKKiBgvZedPpRbTmSOY2rEhWoYL6xpVwD6vy6ArnPNSP69cRSGDFdeP2e9lUHmnoJUkO0KK2nAPhgT0JnWr NDVQUoD1sTXgG2VQtTXlP0eEHgpS7qlgym Kvn1gRfPg deVB2bPYw01 eArxe 6NdZ4xobUXbkzxaHgdsu827rBZhlgay5cghaBgGQ0SqW2mOuR7cBraiQ WMFOLNgxGJBKYBKg6mFnihxgprBmCfPGGwy44NG1AoUYSiUcko6HJ vj6jq7mC4Hq4PRACC0vw4mkNMli15Z8Oi4nRNM4leviPdqCrGsQEwh8nuqlD2xsY0 NBMIJ5siPd uwJc OoB z Zcu62jUShotlXzAz2MVj9 NUVGHpAVbdh2Q9ZAjCQKZcz31jJrLkht2mlAbNqx0DpDOrovPZIjt2fZLkMW3AP7iXsVWrdMovXEO3Z0 8XFI2oRLbPYUyY0GNn5hM9k9LooRLtLzz0VD8yZMpV4LPRPJ4BC tG8Sw8xGovMCjxIYsVzHeJG C2Z0rgnyPZjBEASRQ5 YhU0xIU8 trnlt2KGdzX3YvocOBJttNGk2sOtr8wmsWidhw6cmrSCjS4B5yfLUfdeu4w05zTek2CjJLMXseQfPPYXingLDo5tNC2YEe290H7DqP3M9LjlDo4zKkgtcAlL o0nxWfPsBjZ8w6h o FnBBS3lV0yeWAVD28TErotfEdoZVBcoZVh64Trh3gD8BrhFa64drh o0X7GoUf649rh3eD8wjyFam1IZetZLeK3QOaC06t7 zWXknOtMqCBb2Yfugd0ndlFLZ08ytVUAvzKZucehoi7lWzkjbw2mAiJJwIyHN UydmGaXsML2e082057ku SMagoWOCYzIABy hIQ5g9iLQRrcX2rc2hhjiStotlYU7TXzknC9YG 25IceU2d847VP7PPMiUFotz5mLJDxfz3JzfCZT9 CMYJwb6c7e4VCMI90DavOSEoF rSpRZzVaITZ2kR MXRiIi8yogNPJtCzRCe3nsQJWcNenh4eVshDG2h2 bA98JrWoVj2D9yWVIJ4lrWMe9kTY5aCMhFpO30Vo43CPKVeTK6m2S61LBxqu0HMtOBt67oxgAQSXqO9U hEmwRVEVjJprY45eGo86rFfPtUnx4 S1mXWDju0cPFl46mBSCmjeAkahui1KHBcoZ xrHSG w4QYz o4lYv6V3tw2fGZ8 JWiZz pGgvoZ r5JNucgYr8arRo4HVt9k2abXDQ3p sx6An10HKK6HJUaCpiSTce8TGqUwEkCdiukgbA5I42qi04ENwfhXZr8GW6daYB3yTcydSKKZvX7R7Ksg49lwgquWCW7ZlG74iSGMwFOqdKSa1CaoOtoIUO33CjZrNkwXiufyo38ijqXyo3SzYNw8 bU8hew78Pr3qHqd2 tWrCOMbMDq2xcjPMJOheSscmA M8OtHeudToZ7dTiIbjtUs3XJuKEfoIhH Eoa syXxg wsM2YbO5ioYdGExFuE2oXd V3MakL4aSx YhvgmfpGLJkrpuCM FRcSKhrmJDpKojTPavWz 3ldo4O6WbLWR ufd0hRTbsgUcEPIEzcVfvzc zK7a9YLRG2rpmvVd YR83YNKvqLXZZFgdGrtOatcpDIOtyeuiSrabWIsi0BHHdIqiS1dkKFWPOMFMj837uyrtGGir8sjzAepYZhgwZhJPkd2a4u0pB1aC842RlHqbwoWHVyjy pijjbT7 6cC ZvmqFS2gOYeiHXnAU3HxAc9oAR3HloDYppRwJrvDjsDixAKsxp1srvaQ4LtWWtatGbjyAbGsW8cEyccbOgm5TldMiiPzSjds2 iPgX CZvXsAQax8n3C50z4IhB20l4Y wx4YL0D4Y7OgCHY3a JmzKbxPOdTaxov uP0r4c32FZ6avCOrOIkd0hRTb6Z9qLT FQ7qL3vnOIVrjxWEVHd4J2VgGkxVVuuXYQZIwS5XUEjA5t2YkAXCCDhV8BvdVCk 0JrvKjUpYX0aE5 BO0muzbtfOVyghUSzvNWmTAAR7Yr l6oM53PgMXtQXOg1BFRm7ilPbmZvK2 u92IKkBC IkHkUD61r1eV0G7qcnyU841wnhmxAE7wTUTQG7A09T1AlR2bQXaW8Gu1bW1A uLbGf2CLgbWpJG7Dh GCe9hL9vIkGGf3YyczY 2Qzxaa0IXYyKlgmJfVaqBmQUM7y79kdmRy NdC9mbhzOzgMX6KH6gwAS mZfNIh4IyhQGSpgHoj dQ8Bo1jvSm9QBzvXQLRQ7I ULAI c51vMrNifDX8dPfTX7LsF3RpPxp2CPxAy16DTLD1YD8R0D4Y7RgCj8q9mv4rhif98w1aCHZ8FvpSKiPx3EDrwtlj2alTD310sIU0bljQCYsycQMMjg8IEGt8rtmr0MAQnIgZcIeAzLU1klhE7kTCye Xdn0cfg235hOMwo86Nmw2fYTQvNQcyIlNoD7rV0a6lxtfxCTfrXLJR kU8b1wXLjC1xQlmNTKZf0bEbNyjzAa9wwLihNoS70QEHCA3a5GqoE0FKZhwBKZWUSp8Gmlgl7ePaKd2l51g5ifXd7M 7lt SmGx1dZelBolbOsrtL1sKWNM316xj cJezPXycmoBm2tWURapE R10l3ut8b9KOpZP2g7zO3Fcgg6Cp6Xp YXIN5yjZCC8qhCE8aZpUh3PaG8HWyYQ7wDBTkvGwv6wDL1E1k xQOQRYzkbhXH59so4mA2NCN8Ewpb253bgOsDleMenvOdUw3vIYCTotZGpWHAHXkz798dhKYLvF1Xl7G77rtV9IoPm7uW96Th4Xx5o 4Vrtn1e2n7HipM1J5qAsNDNyNRl6PDy9f nZU3oVVRXIDSFWSrcEQr8pqe8eBRq8cC5JGLhqVxvCZktPpJEZmjxRDkf75fA2ELfItz1N4HYfehtjO qbvUy1k MDfizAgYNmDxh7y4pAR0hTx6Jok9rNOpkvEcQOGlNlFkgOQm0LSRJW g hqJYGu D27Rngj546WUzxtkOXeoIuwKIKhjBo0hVzUB39Mp2VfpjUFm7hh3VYo7YduhpTBVZ0Z2n3E6Uz39SFCKh Y0wmsq HirXrousGowo2oeKMw2jrFyzDjoiwooEGIfY8Ja7eYefKJjbWcbbjab259Ev1Dx9hZSFiryyaU4UDNPQx9Qx9QQE1RE1RELrnf7XLam6hXp3Ms7kfUkKVxfenYIURIdg8lLEoa lDHjYepkdeRUIw2iyxmIbiwtvLTuoP1eVShXAAlf9tCBIx2JynTTUjWeoP1LFPpI1MJUIYpgvxqX7s9XhbZkSy0EiDx XZRpzAY3619eR9WTU5 Pl5tcnuCCK3wXXpAMCLmoMnuK2ZSHSbfVUZuxPHSUsM OhaEgiAogN1vIsUtG4OQCwhPNUPMBNHzltxvjvqyUSrTOafGgjh59Ku1AaTBxfBnJTzG2bIIJHiGijaNk26AQxtYW0vYX10kgW acTGDvSpqABtZArGkTaG40ojnhjzPAGEZmaXpjHzea9vOeHMBvtDtBLAu2ojaohA5Dn1QHU3Nogln8kvm45qBzRuxbwFKvhDgReYaIGsMGIHEFAneGjrRIWZHfzL7PiJ9oCUqxQbrwxvzB7dWszzsYWzW79LaQwv2PJGimGY2SjwtY6MDbLDcEAGFhkKq70NKLnoim4gM F3JX4QABUNKm6qIGGbnBjDzyScdQouiEjdyNOSwgS0N6lY4zpaXmDcdNxpPsDxE1hGlOEdJUGrPAo6A7aYeeWya0I0iYouL3vrIX yIZyfWr1e0AZlP ImfeJXo dyocfwOP6ifkxjxZowHjm48Oxxowhat fHxcWHUM3CRqyQpXXPHkJRPbo4pXYeQxudKfE8kPuhRuVXwXlLyVNQwnSAOXfQGiyodHTiO iyzDyWliwbgGkXZkeyb4JdjRtkjqd1lYseALGiMD9yQrGeuk0wwYsgiqq FrucJhJtagq7eBk ixyowCO9hM7n2OZ4xGPrcLDWr9bfj56rKvl7kPRGZ8nnuK2Y0KVib9P4lqjJlLG5HT69S1CDvmcvapSclbVn5V0wMnRQ6jZc yOLzbnOyqGlsPRz4TiIaOsjdFDnD9obDNQGelt6RN2xNJ6MnnWiJ1ZC90bjAOOaR1v3uqrKyZvdI yxjY02O56lMtmrpzyrtxugEe4feulMJFpyeWtm9G gkvjrShkhEg 6ViMOGAnIjhSaF0CewYnAwcJZJq 5eklOVdvO89OhbTBUFQ7oQcSS Yq7cjss0iKWjX2lu0ezZNcNi2PfD4IGM3CfVthBhuXYWBBiXYWFFzjKL gkILL3 N8YGXJEOLO3FQ2PUCMc5BFRV4BdBPTQA2ZEgNZMJmxaAmHIPjdcDUwNNhM2EE4ByCBJI1J3C p03XFlVyCDmsmDKctElVYAZV8svISHdyl0sAxSqFsdbsSd s7CfTKJzH4wVX2Wk6Yvvun1borcbAWUgvdJgTcXWBNw1mhVxiQR3TT ekm4fkUbYEUKMBI12yXkZUXIalzTYRUrLjEIedAGnCs4zsYXfL pqDnJAzba6d6pHgRH5E9bsTDbTaqni2sbxzX9Gm5IVfpKRVrFs7Iw1cxTYjRtUO8F8ND7Ei B3Uf2afzkrymssDmQksCNzxUsL5xzJkZT8LzVKuy0nS8 Ao4t6KHvvOjHPMvmz2R1bKd00etVHzthfAb7C5Kckg9YMeuOlEXG37eEnWXdljbSKaSr4Wr6cXWYip0HsrOrwzIaKgYdDuWBuu2HXiaOTJLUMB2m9dRgHflxoXrai4aJQq rgC0Htyp8jeZiD1C1Xb7tTXSMooAwy8lDGzvC5gJkBUoJaOcPsJcpqoO3xgyBctmmanYeJVKHQVkulKvGSDan566WOKAzmbcrByO56I0QiPngwnD7tYSPl83GjaXSWqItiIm5dR5 yymxYT OwGH0L4kSk QS3gVTSvK3jSD7I tfmEVcBczrgTLLvEyE ktS6AprMCDSGDOE7qu 07hgfLCizns3rsP1TOMAyPFmYhNQLugBSq5jMgFm56OCrEFHoEv Y6VKtuEUNQAxbkeBamzjf2xJ YcgMLraLa YXlNoUafkz KVYZj Hl86h9hXp FPqXApF4BiQ k aWvwqoSu4UgPe7PKsAzn6Yt PBDZ236Fh2Ro5T 52S9T l8oHXqRs1bsy1vvCuCPWxfcoOnsMlErEbAN mTEXwVK3nU eF0fPUVwVbDEf8ZI67m3JUZPo1JEQtnzR5 HTUH3TYn0la2SENp1 w63 7XFp51G9g1tR Duk P0Vzhw pG9ue VODQhyE dLhpRkb L4HE5BR1NUD67Moqj8OOM5Tluk3mx6A1oy P1ATr703B7u5AN6CbhAv67Io3Ij7T17o3JUXPo1d8x9x DBj36NMUyxNF1ZGJ6i3Nr48PE4wvvB9FBt6cG9kd7DgvJ 9x88stkxDNTTXH4Vo8T1D1GvBoW6JaFE6dmKBOvvAwixl zwDTDux7jSl7mlEklF2 t TFu8uwU TgQ4qfkniDgP9uBt2n906EJEQvlEE627DHcfg5hJdRID6sSZDcVlNVxRLipozU6uu TQUSJJFGtPCpXp7Pt8qMqhw1oV8TrNwfEMrJ0lDeEENqvR727aQW e2PnP8HWtCIFcvZ82LT1gDB79M8YlSmDivBvyGsduC0GtP62OJtJDWK4zBHhs1DbJqV0VfWXpf EPE6q9WB 2y O1Y1RdZDO4qWRKfprQ rUdzv9FhCS2feSOop8WH9bO2 VjL7H bu5 BXLoY5ux ok4 8iDZ9 PEDFf8JODFhELpnA2OpjvfAxDKcSs93pOGJbOy6bs4JijSTjuuB6yYDPwJTY2SieEIjdlyeCEAWs9tl7kNnt0XUXuYYXsUk7DhnrSg61XtiJbWYPGvAjQrWXcO6qnE1rahvJoyfh wriQDvkvIEFmLdq1vIRoiZYcc4ajUGVoaNZc0uHHrIAhLdOYWsVpA9wmlzhPTKJDIGy0z3 qjo2TVFsz7yKhvE oxrSXup0 7c0zXbr6KYdijBjEbcrWamjtWq7UQIZHsXLnuACG0meWjs4m8GBOreHirlMqmQwHWlVWhlcN1EVOih78EdGaeJmcELDEMEdYKLVLCnTGu4qgbN8zpxeNJDUSHrAoeRq6OL4yHgEQQDK0CDeQTZhfZI6lXBGDwZigaJxXJ3phHZSm Z5PM65ze55XbL5omSWeDLnFFs9xmxhu1 WuKbF7frnn cw3 QLxy5YBuP8Ato7CLNF25wRDV k9qwROiBjMNV1l0Q0DYedkOsXYGGY3rL5pdWQlNJ0pOogOtyr esO7NBZwFE7csROTVIERpBVT otdQQSUkgyp J3mq9bU6c4LaUfgfTBTzR5XV89e0QyU7S1392zb0RfGfphE1PwyIfxHuzONhdee xyDp d0hp2cZGgfVC7EwSDpl16pdwv8LTgNUfk1brvp5e1QTSHZt6x6NCrqdy5Ne3rutq7ZNSYt3u2wzJzAQRFzvnxfRBVMMtvGfRSHTPl9alKcwA72qfrkkOcY5qhQ5phwCQ9T8 6KVCk64gFy89kAUlD59wAYfxA7NO17 V8oO1ZgguxbuA5LQPSvZTxWCBv07CsLAUp17CrR36uNQ xjRNpAbRVHA7C 7qkXJoEMw 3TEKqCeZqCek8GqFKXBQGQc58KOM1UfhWC7TrBJDtRPd5acMg112MDqeMB9uze29qWUgSLBOmo auKnN3PEtpPr0bz z8Hdey7OB6kUwKh6HffQazL5Qc2peaNSIa3wZfknA cJfyJwys c00jE2dn6yflfk7jLDX9 J7aEIRPa9YQCqhn Wdfg0kQY1m3thpHfpygUWqHRTPTbL7mQtwT9t5b1iJm6WHGcxbzpz9D8K0aewH8rHXeMxd8mqZGvvK1B8P8SHBWsX0265wznUPRIW 4G7V99OO5kWgcTQQvJ27MIH8w60gP2Da N7dfFUfj02Rmqcx33dg8XNLPvtlCPKWo7dZgd1u2hYQqFsvwBI1MFQvwEL1xzJLwFXPNFIfwE2kI2wGVYeoUbkYgZELmvOnfUcI mdmaHSRkkPMblTQfnABENkWUfJ5lrh19 XHHzJpQzYjjEdtQMs1vXPhzjL60QzXAP8DIZCwKc5 mzHCW1mZh6vVOHSR6a11nnTXIlDowboVeOnn y4GFJvPbJ1L11GvvbKoVdjjBbt5Ak1YM5bFmwoRzdBEbTfx6Wt7eNm5HeKlH80iLDgeEycmqUlV1tPaTZDveZkD859dWN2hWKbyqljezfT4WLWYE afX0cHhmeU55dKQP5ookvbr5yxuwkZ KYGdcYe6rXpj7xnVZ8gZBa6gyuferr8NP2ykvzDDfJTvw0SMYaZ7e67tqCh OljrbCjAoOWVs f6l6eKoyImKtgH2eQpST1H6l675FUioDexrmttDxnR4YBhynwoEa6LTaK4GbVY CWqDFBifTtLde2GtyPHeasO33hmYJlizDcdi8MkUSbNCb1MqlkNnKjV4gqUuFsGEXEms6HQkhz36r70dkumBulyW7OMcMGf5alo9AjB0t oaTQsi9iAv7WfMIXmDiVgY7h3Iyi89Hzb1iqyJdmaA3aSndK11KjvplrvjuTqdJfxBWwgIg8VSuGU88 jtFPJM6LrGwkqDIVoubsoZsO6cagYDUc5bnDlMBGbYCMmjILUGAMfpXNvm b0xuAWaIfqQuwyrdojaFnh7Sm8HNEDPOcWHjE2fdWxUUqlgKfkrlTdZ37WKBFdMZo6LqtAWb6xbhfqIHrS9jVys5E 8duOlMxp2KhooViIYowIzbPk8PsJpHLfiQGoVGms9JEalJMZ9twtGZ lVwrBwSITjm40XeewwRgp9vmDCNcK MTmmIDabGrtgpW60JiwuPaEAAsN59WTtDpQGQRp6jt1vMKwA u4Pd zybuxk0xYpm4ZIVS1OHIO4RzQpnSBk6yGrlfrNqmA6OBU T9d6KjJ8n2 cluEPzlVXIlpD5eFG oITGJ1INVFVmi85eoWxunW1RK5yxp nEwrho9BXE hR8tuzTLxcgcSWYRTw5PZe2sC8LPRQB6OSL82oHBStHjpHU Kqoz9x6pSRJDOv8QLNQ8kRnXKluEJLxP8sriaVTQLGvo9u0VBl8g6CBzWJRehSnbqGBPKpJEO5slHcRxa7DyJjNywIhvjPJMKTVJDj4bLUGEqtdxzndqtmGM8 YsnBGBU0jlH7epKtHMNRsdcHDcBZtaWWvFSQVJRdWrgToq491EfwwxLpEQJYSdikCVbWFwasafWm uy0TuzgBZgW4GjmQHnXJavjqzWMOECyhc1YEiR0iOIISbOJeJ5eAYYuYgR7Oa7I3b06agfmvtBByD2fLTYI1uUUIBupegIpuYGPSDqhZkAv3PPvO MJipizv3HLDpskVNqtaAYWQAZj7qerUJwBjs7kepf7q94OhdsjP1Q76 h66rIN KwvDlV3 oM9qUrt4zwIRMiUt53xwIj6OQIh6OQ3V3MA0TmkYx6GVgfsZ1qwAQQRTotbwCdS40pBtDyS60KjzYDK3Yvy3c6K5AquBG8laV97Ovl2g4Izrm0yyHp8RGm8mlw32P3XZ1IcDgHHoloHALr9jTlnMxdb5l6ls4xpsEJKWM0qZRDzzMPZejdkJomW8WCMPeOJChgnm XJeawqOI1vrRC7QeUTEOcJfS yHjB6cJLkm u6WQlDrYBNeX1P0p cfsSpGu J99SDC7A5oX7xlHRTgEis0te4xWsnIdYCu0Tu6qu vkPTfH gbDOvPf8yI Eh0VhyxIb0ksBYEJdRXENcY 6GKOAA0eqVhvzVBfo7WU8d4eVxW9Y lg6qSwi4qS tADPhU1FYMINJ1D267TzR2QEViSL7X6Et4IY6ielIFG7oK WqELa5umKdXWCeMeo1PZJbHy9fYoXS27T8EQEEhbZ ywTUn 8s20MVy1MBXMb vFGzS5sV2S11SU35WNlB8Grbf9a519l9yODjM8PmTnwLosGpEQNzSx13K 9ot70xcl4PtUGqQ1mTBNEHpEQDR2o9Ihg8RcQ1PxNG z eIWF3jd ACK mpIiWFSyaVCTdmiaB Teb8Jorv5A4Xw3V0u2AUst2Sov59MYqeSF2aWa11CveG7PfH7odGBreo7deG0weY7jaYhxX2rM5Arcq3meZsO vanrWgU8oWc4bW7icFUjUwtBfOXJNIq stYV1jRzlB4OZ3oG0vk44EvSW98 5Ch cx0OgVW sh cxY2RW do fEqxvEH sh cpKFH Lfl8hps8iOFPfKXJfLnM51sfAKymyNwZz7MAokeWFL ZHgjgd4P0BYa8x5pZhq67FP aaEPyWx2yByJk0DKvAz7TIuMxdkZ9hRmtOxvLCYdDzNDcxIVCgOe3xSsmP81KLTLM10SMtKc8IqbYeRalHhQ5wGubfALqSnfIW RGs9ajXPQkBxd0u6snnIszVauKhmwrUmAAyOatPTlmhr2is98VoXEVzu0JZSETTtKzblrcD04JpHzJiTIVC2e7swKFbT8hJiwMF5hmrg8Qk1TvCuXJWnkd zr5bXlGj7bPyViO zte9R3ylBGIMu30Sre0 CqbEO4SvzG01n3g9q2kvbLX1XIzZnz6T86bQTV5R6BvPnBDFpv9wNTDpJ3o1B1o9STt9MWcP tuHA4oMOPRPK3UpLW1AA9U9ka8CcalHB5i2zkkvRjnkhqsR39wekQHqrVxszgj0swsWe6iv4T8l2o1BaJ7qx frLRQg6lspDtnB0v6GEbDt5AiMNUDpLV0lbJhPL a9aWdXaF uUXk8dMa Y3Ns H5jbKXTqeKoqmKC5PzRLlXkoqpcwmpCZVHqB408QvIPtMeDnrNATvxOFwt h2o1VXL Iqzt97GLODQF3SFIV037uTt9pTxaPTt9pRxcLl06AT7fuQ4ntjupxkOESlZwxhktiK9hgfrlIlDDzvTQQsmfYXlWjBZua269RHf 3KAYyHEdRFUSxQPJlzmnpgGMH0uqAMn4yWb074TADnGct9Bb44LwALPKWlds7uV9hRDQFBGm 6W gQms6B HBN6BPsy Bde Vhq83GZBQO420IOY5BN63hvq672o1nafXU7wZ X XWbNfCayU700o9JOYJpjbHQMl96boG4neD3VxfDpyY9TS Oj4uD8xiM09t6QD TtpqyhaoWvtpSN6LeJjnl3o9P98Zx07z991xVqJZH90TSngDze9zO29SpiZ4faUWZb fH 8m 3TCVDimZqHvgqRt1XEIRzp 8Y5A 4OZ0ozVShRfw4MjvR7nvuvv7JhFAeFoxwn9j zd UsXD5xpF9L8M7lK6vQ2UNsNJIBrFRtJ 8N27LM7t9 4BJ 8R3l05RqVCJ31JCkSVXskPiyd0DxyiFTZTUVeGPH8 bb5gQJDhym8ZrdKAN653KR8 lGjKkX4Kd6zpc7a4 A9gTt9IqJ66Tt9R7VDdURW1DNMBrrhYFHnXDryLRdTl2V0v2wU6bBq Ec0R9Qq98ZRI9W QwNPRfSHzTnSHfr206SjTJg06lBiV1w2k7QbqHCPBNCgVNV1 HuT pUjB8qR CGf9viwvIpulm04ZD V2U3BPDp 8a8PWqq1 1FIzo4AtFGZ8njhERDQ XahRDQpxk upySMFehkt60wpucvb7p4R2tqpaSkYN16NRBaxmymX93j2Kpz oat9Z36Rs Dx7EEwv2lZ3 kVBlQ 8ByQ n3 VvZt4Te17C T8omIq2bCwfxMT csgHmBUpHOD9yLUZo1Qh2P gymGOxllpWcxUozmrg0HSTMC96A7t QDhmmyn8X4c4piJguiWn5wm9KDrRnhXUhoxjtMKVxsD9kfr1kZFhUOhjCYV9sAvj 0otczLGJ4WtkB0wEUJ3nFc zhLd0v2furgQ8ZlgkROutOz3eqNPHreJyZtoo8uVh581X7qJd9r7Iz1Xs05rHSrkqJToSx5Wr2Rq3Q6bt689ckg ty0CQqXbxdD8bMDM1eTZSuM KYuS0WdMVGuH8Lgk qdjn4qfkm8ysfkn6qJviFMCO3SmEHs4 aSuOFn875hF4bP n W5PFH1IBMOwERRCTt4W5J8i1OdykL WgKKjSfyY0TbjCg8HmDRy J10eaCOFiFU3VUxGshLdTN7Ug xDtphCiQtlCK91JqcJzscc8cUOK3jCbH4EJjtSqKjZuxxhrAWb7IhVFCuYhT4TusyP qsUMZp9vOSUA3Y7kfj3IsyMTpG sxt35l0F0wuSt6x8GGyieXyYhoq0AoWDhsIlHviXC2nYjtL4DCokUd9D0D2cKkDSRVeSQFOQmyT4yCZKfFg4JLXKqgIUZL7qyKI1C8epccKJuDK43I4 o2IF7YeSFmufXFgm dOFH7NPa 8 yyiswSFhf5FOAVCKwp4KRAs3e8ECuj54OoYrxvK0XxdOJHPKJKW PlpXAZTXulV sRTX6Rm9e oTfRBRBrdJ AEi1ajmsjcehNrbrkAhchkhXycZYlo5eRTXclp7gLF8YiNDPzMFH1kVkYCty7q1uCOGqsG84Xe2rvhpLDwB4kEL Qr99H5Dwb2XNIDwpisqughu8aBCvfa8ID05O7y1ZUgJmpQVzJZxGUsRDncGGYGJpm7NO7H6OnC5NadBosdF62WOdZzJPy7qQVG1Im9ZKMAHYRp 6ln ABm98D4WMLo8j78GafELkr74LlTxChhpCzmkfm8lIho9e1EQMoUk1wuZq9eBdSF0wYv3PMTP18gZyBA4QAnqxnEd ozVHRkEOeM61oaF1AVJZqYupdcI0zO1rB9lSjDKZKH7EUwp57LOnucqmlHuT opO1d3vaTSvQQ5OzA0l7mQBN6VB7u 9AN6lQ2o1d8GKLhjduMQ1sF67enV9VgD67VYPB9n7T1RzD NXFfJZiSEWz4Y79EX6dEU6TEakAulesQ3GdBGDAcEn7ToDmOdzsNeYpKr8SEOy5lqLb1yS3j wVb7r0 np9PO3lF xk E gwsXWPhpiaH Ft16F d26BKY6 IbCyBPXzKV9zcM8YzAwpyCAhHb1HkOI kTt9L5Swb w70 sOLDHwGvMKXs4NEzod4 NhdW6 tUuoHAHiZbykiVb5rF9xF PwEPnwfVfZehm9FFaoIn0o9 qOPExmE6RDhs Jl6m17rs9i0Nq1OF11 wplY7 TvfQJqZKL3hge pc x9ITwn93Flt7Ak28zMBYUbUVgsa SPeoda8QVEB bjU5oXOSiYfO5Nc3Ps879jTyL WRsJVTcjOsyjrmDqydT1l BDib8fKgoFsrXGkkvta6 LYYr15GIfYdU9QIhb2cE8vGciGSzYFMAzYnoRz5HUo61HE0sNrlJstaTv8ymrmNsu1pYM5moJFgNy0uFsHWIVBpfDzmXULKxrty12e6wZgg4qXURm0sXou0NNHBkcJJhjLewekdVYle8Xzirxt8gZH7ERjDlEty3WmKNl 9VLRY9svMyL5jTrOpH3ggBxfJSachLUOuxEInMcd1mtHXzAkgW1mG77Ihk5aAgok2UzrQQpPNXGUCM4vioxOvMzW6y6dzW2p94YY9jTvCV3enmaOO1w4FjxDfJjjyWfDRdyTcaib1w0CKuE6MY6DsDczmQU9jb0v4QX2SoIAtOoS3td IxLqiVd3qpFKwKHu4phE4CZ3f9enJHgRNd5nH6U4JyW mWa7YIf94OJDH6syKrKl3mco3B IAVfCKd2qxu2kKgIq6xIUB66E HmHqesZvyxN9I0wulvOuFszUKmuKkk2wkgUzUopiLo0WmiYxeVFUU2MnCmBkK9i8SreurXfp fPknncayuEnqLNUx 9VISq JXYV4GyqGuR51Kn9ZB4888ka5TO52npRJQqrhHrAfhCye1vbnLWqOFvLaa4exHceaY26kUsKTpdaXJ8gMQcdMJH5LTh76YMtZV9 dtrF0NzWrO6CNdSvKRQ6JTcdxIHUrhcZwxEBjq8KIJaadsam0GJaamrZQQAg1mfyxfgEigaLgXNF1W0ueGH5aGuWS2AnSL10fYM1MhAiyRuzwFYvRiK1O8SWswExf7Tqhsri X45pkK 8W6HstBwDhNOmJqHPHXyv3ie R8bkOrLN4Aq3KTCaIhJvXJmmogyAz7DuvKTDfYNvXJmyEi jxDo CkMNncyeAeqeRwha0WC0JSSLuGn0jBdu4zDKwaMakahNwZJWrUIMqECsSiM3IY7rEJJztHrkfZZPYIytDc7Q7jFqqMfye6O4fJnI yfaurgbm6WiqpgWIgxAbmiEearj24NXyObesPuZwwxi bJHH1TScHbgvo2G0iqqox0UVjRpbh0bziBSHX 7jCL1p70YxtwaxdYglwiFEK9pJTcfSAfYFyV8Xo21m1LMwEBXEMbhHdan7FrsqSoiysaIr5qq9Wxw GQ2yauoi X45piMp6vPq36QHazO8CqkLNjdsfyzAoc89Hk4vjizSog5HTnfXqrpmqZQvTH5S 5wIfBrWsJY4M0eWEZaKxIpRPqHrq2i65ChiBhWKknsEX8VSP9oWE6SI2RelILuAeoZVWOOKXfXRZHash2lZI0y Qq7rRQq J4LpwPlJ8DPkAcm sCbFaed5n1sitV20OytEfqowtL6aibqmQxRAuuotfyx ndX0gTGx el1cDXlHCl6LloDBnv8QVfmSP0yEfESbzl6OOi24hROy2Zk3ylTyJYfJUlPJeg lXL3uOinUD1MGgbRaAigufdM3R5n2OuSjEh1M1eYTGx5r2JGRTkasrpqWXTCpewi7sE4hzaRRxtUHWcBdMFqVShlqKwtr6ZBWXyi7DsYf9muf7GhKlveOGbf7GVoENOXWT01tHEqFIwmiQ HOsojK1Ptaub VPDQVw7u ATt9SFmm4i9U4dA2VkZTAYKqbRPuh71RfS3X lQOFzf25Vi3 IfbirhS vw6WCAiyRG33nd5qDFOahQlv7uV1RoTsvTgY6oZ06Aw3L3xqVdCgndnhvYjYqhR7xekdZxet9wV9uQGihY9YC4Uj4TJ3efjkDjVmEZEB3t5XTUhkltd9dKYyRORSRmOpfdiR7LYd6mKmmn6xzFCIrnocEvkVLmnG6 n6zxp3gCvlqEKl1ci A8Uxb oNC7N2Gb6GFQYUmVl8nSol6DKKQOhaRe5OOxVp6s0aGrX5pCxaonCvXid2peDfq4xVjWi F 7H 8TbgjVr rMYSHfO0e61H02faQ4R16SRAJw5djhG Ela DLFQ8dKVaw1Jph20RdB4GccCmpYBBy6RutRAy Tu26Kow4EwnH5ntTY6snXqcCoOdYdpV1lrX9QDL1BDTF06ubL8tbzeVjglll6ZI2IgX6eWJ OEU yfmdgz4XD nYUg64P7PbDc0XXGVzA4Q1be8KY2EHXoC7OqEYOFgMtIkzvGPiJUSkYDLQZg2q76M8vukvynzht0VL3chGqzYciDso4gC2XZZyzwjJh C1QRPu7cUha8xd0jMFCOb27d26VaO7mwdW9HtkoF55jNK KRJEhh7oUrBUPoklI8uZaVC9nvXXXv97p9gh XxHyteoPEzpa8Cm1FtcqIPBnIF4i7cSa02f8dhzxGcczxGfeYUWAY1KNr2OoWMOirg5R9 dLy12kG6nmVnf96036hjqBwuwFDvFxN9gui5JnDXH1yyoRByQN17rGbAkeVf8wwFereXQM30AOJAZQmO3G9KkkwyJYJ2r5icvSsOMNqB4TzvDdznJo1eK5dvyqLCt4hjGgVgWJCuBatAUpG1jWlveLsK2XDVwOApsljKy1x BYKRItehbDwz09VROu1eQgbmGcHCqSaKjUIqiYh ip03rRh574TpCRB2dgExj7U9ZSumpClsc0HKfyShEaRmscXbsxRHMBuazjgb49cDi34xJaGkOIsiqOecdunVfEskI55VQegZ84XXPQjeQNOhhmV8n15MGm X9JXeawG1j81Mgkd7vPWliWZmKwMZSFM KQIzDUIdnuU4peJhRcYTCrqSMxxXildd0rSviSbc9XDM8zgya236GL8O2pc5ZwLd6GxL93c8 wK 3sx9iAAwkes GHzNdxh4IchFRGTx KxekuYwjLxS41sPHZNpxhSvR3N4M9LWX5crxXDnIatYAZe a NfQNsYcY4G5JPrMjQ4tYJ8u j0z7dZB5jyGymNUA45 gM9SfzqjhoUcBX7yg9glzFsjB 7e9tPJNf07WLi dS74hy3XpWUJFGDh a7o b2D8dS74hy3XJLDhrfi6V RCf2MvO7magerhkpSLf GCxcU4gOCX6ZtohTwe 4JSlw2dPiHCjGRE1GNvX3d2f79iExavBTm5QtVazFXF37e zOEP7h29T6JRTVST4f 7vUJw 0957gQ8lVj7TZjuSf5pEZw BcUyN8Bh4pqn8feKElESbXGG6pLiiyVip rcXyF2ZZdlk7lClaBN6LVX0kUWcXoy4eWLD54s6c8XJpj09Wl0nWR 3 NpsyRjsm1KzFRdXL5dSUJymdm8lir2O40pS0gjJlas2Zt74iH9W7tHc7lRIeJ ZExRXgxfNTZUMAupsTcIGEbOQhD4s9gFHDx7eCIIjkj4usuOH6nL02OO417UU7g7yBTRDUusaA5dWPasuv1 p dvvIsvfsvuqF6tvurfQvQ z tdNpjKkQfOTP3G2 hkvOoVZLZzUFMZ61ogWwq869mZ7mZ7zU8EBEEH xr 4mFiKYPyzCBMJOcJXcfbcSdYkU8iGfOhbsRRzzMfeTeaSrTJwHjW0 opNHZibmMtEk82skmogLjkv3APPOF7aaEjQsdrvYIBeKm5nX24GWvVDOjy6OrBc6h1RCsC oyifLjkzrlKOkgKCM NAWdolVd9MHnDNdRqGOqdbUJKMRnZPQin79lOIe GIsyQ2wLWVmcYNFh5ZhyKgrFEOHcUv aMNRlPA1jDqNwL6Ku7kDE7wtXIpLaxgBukqPT OuLbJYIwxb1NhmX mqk6kfu5pNmiBdtyJBLIclL18IfkdzkyW5QSp0pO1 573qNa3Qdk2vyDcP7BK4W 0ZJh8s9llmd99ZCDkp70Z7mhRWQkh70Z7c 6KLpJe Qah 62PGi Qh7ITSibMwuVlFNYoIw PNPOSlM0cQbeNInGchmaU9P4 3QrH0ad J42YD97qZ2msM1ZV5eA4AF36Ot9IzRbwiH89VuZ1Nbhy3kx5hPcv5KLft0bO wQiSBKK4LrsU1QxU1ctPXo6ehgko1Sv5tvRQMV6OqRFmJp4O4goXNFu8401ID0aIudbc3UfYOrGctvdYOAeR8Bm YwzjbhIqiTMYjYQiKwo6nX0AmOOAV9kygB8eMX3nAD4GshvQ9wus7bnRrMEanZoXnt0Q685H Fq8v2vyzUPRqtJPNgFA3snhCHPYHLQuhECIl3oPc6WQf8RTmybTO iBms6S63KbQ a4B6IB3OtvpU0rcdP5WEjzO3PSxQefa34H9IKjNth1APPp98s4OnvwufNO5 jHjE6RYqWLJ SmgICxyIJe0OTBnoIowuz0OC0OTEKqF3HdnfPLLtpL7fR NKA6Kg7Qeq4AE6oSJwXou4DzxGgNQh3FlAuBCOfQnNM0JavrzMox MpnUyrK h36WJkOfsk0KsyAgBRc93EE1DuaTFgzu7l9dYp6okoI9Lwbf2wA6 ChzJZ 14nHv6UcN WoXq lXcdpI8o0gsSyr8 bi0FG0nZvhi8A4N4uaioV2 MrWq093 bcZ KlCd3lBggwUx4oifqqA6aVZhLTBb8k6sNzl76dVGve5t6nFHsKIeBui0y7iZ3C2QJpB5fAvRk2IqfmQpD0EOjzDSydsI7x50PPFCCXOnKuK2U3iAZiA2aLJamDca1itwko9a7cjwWiHkkIl6WRgGHd0Qvd0HZnnjyjRowOAXXbSfUchMeAiXVe6pYGG6aPg6sp8h62GyJiUjLgeTzVh6UU7do HvUT9Ig d3XiPkr1zGMYiwXLOZSem19i4yYScjogXHfMTn85ciq95thXagKyA00P4SsAWlMMqxvxsdWCr1CP3Hy7dheD7RXS16lHba4UTDx6WJprG GsbX kxTkwZR5LuHCTvhBDwSG0K4SUgqx4dKcRd6fey5e1UXc8i9DqWTPd8euRovfDneH K0skMs9gPiRaDiHzwdij6ZkN2Mr 8sQfbgRZSYqQ6 X1N5nbE9JCPA2HAdxOaztBB6yRWUGnIaFhsZb OTRx0AwkdxyZjqMi BbFG67kFdLBv0vvxg8QKGEl8r6NlErknEsYOvJ05rjD3s3UGtIuDf46fxfzev0C2SrAtJtDnpqPrnY1g3qToZvukkkVo2FuRR8jLI1dwV7yQ1mNtzA5LKbjKTiXDOCzfUdS2Ybeqqlyym3Ah89rwPWTVVm95RCrnvgxgCvv6jC1SGAnjLrDqBhjgDNlOXKIMGSRvOp56u Yz9SV05FoDJmsuufWD7hJGQeQjVslxnNDiRKjUuK0sEDddH41vp1YtsuydW127FSMbv3 vlBzOm KahCo0J80bIC8DjpNrzo zBYgwzfdeWDddGKSzwyxaPXSB4snMEq0GocLMDBhDOOK3nUuYOWcmV NhBQjfJISsH y7wy2bRmBCh1eS2SzflyGWChJ9YIVEjwMrTXfTNJzMpZUn4FRGR6sWSSb0vMYLpfmxC7WId8NkK3CYxuP3Dkp3teT2YNHPY6 Dbl83TSYFI6wLUJ6SQlHwcSL8dnAccjzQEBlT6h6U3Mn4RHXC2zdJ5Q3gSFgDINtNMZQ7GZR74ZSD5CBxs1eNVZaOvYrPI7wXGXniaHW2 Dw0Q8QNfn GlqYa uWBfTF0ClUl7EJtvUWv86rH Ax07BRobSRoBXAbVGF 9e81Qz3ApV3nPunI1Xgft1jaroXSF My3314ualMOGCFIjX7zldiliVvzGVXFNXcwoOSvqyOOcnNGNobzjILY3EuP8YzWFJ4UJo5Cxbhqd3KbfUaXsmkScc1VpfSGirFsfTNMHd96kY7PtMVotAyhKn0r5dtVmIPxoXuiROz5f1jJOlYrQz H80dVUQGAwoJqu1207560DgZki aZNYkhl wQQgVSCYvCyZm1KC bpK O10WYycKAybiQcT2z2m GrMLStFxuuswZCTGMFYXrGC xxGe466d3yMy47XcVmcDIuzLcMyWbojVBfJyUwAvvaCXflDSI48rJ2UgJNSpbBoR 4s6d2Ijsh7RLk 0XYls7RRbYjAshnjWAbpAw5 0oNhK5n2Hl5yJOypZNxSvCc2OOhnBBqtXgFJWmpr7hesvUHEprDNttWlF1H9 57b 7WEpApH9EGEWfbsyW9orpT Tzc7D 9S5ZI vP2xkS3d1dQ2H9 Ld88c79lLzZNb9GVA9qVVD92p0K4eY7ZGtAfK8xXYxpoJlu8MRuGLPfUoLSDqLBJlfleL8k4YrbrzfcsAk4gIsDexWdYjcp7nSjDuTqCMPqZtlA9Pdh3JEUWR8XB oz0vQyq6KKveG0Mq8iUiDeh69BmoWi9E646X2m34cgYVtgiXlGZcdHEHOxiExe Nxb N FHg9tZA0HvZci02t3ayK4baQjj9IdEL8jbcGRtZlh0goeZtq9Q3PhO7 E9qFanww 6y9Hrw4610uLJBltDrs O4vJYhfNRh UcHxr6aC5LcOJqzh4UYf8I5gB1AtfpM5hY64tBlUVRKKsK7a4bWYxvy2xpmZ3fweCPWSVzMWS H6Hz JtgM9p9rcZ ViGhq2 m67 lHz RHzFRGrYhM9VJE Fdv23omZJeM9Fmwy2 jQeVrgg5GpltnAtaKIrws1bcbe8TckMpMyqZ9WTFeyr YHzF6WS W2 iV2xGWM wt2 rd92w7hlt7xSaoDEqt8WKmvdn70eM5Zogoz18QzAVVFYchPqL9mDJJu9U IHEpv92ZuNK8NZEo0UcF jb0QfNZTFTxU2M20M y00acghB7p4MRGtyixtgCU25 XaIll34CURLa9T40k E O40h0BsrDHhf9 8nMKK5CG8FC77 a3s7 9 AFt9em cbJnw7 ZuKZhA UBoF73Bh3s739a35UU49a Sm6dFp4NN o31R8jHFUgefVIWmpaQui6DflS5wln1qJIoBTYmh unA1yQzRj8OTl9iHgo073HA0c0gTN7CxVkgsD9ZqBKpb8lVFp5xVU2kxS97TfCL7rQsAl9WrJVx7QUk nTPBkQwOVDswfPF61ft97tCXqVFo6JVyO3pl07fMm07zr lJ MFn6fK3ZGMy H JFzO3Vk07NcGszHqVxk07xe6909GEBWdUTlFpyqkgJvW7qgEn MZuUz55Wjd N9Po0c1AwgJCxCGK5NcW OVo68A 4httF pxDgK0sGA2tRbVyNA wT8NbaFwRqgBCj6aCJfNs Fbkw4EnAWc3weKHLd9FZeJ6VxlK6891a9VOfcHEFhgID69GWUS G6gGTZR 4Pd2ob ICwdm67qf9EpVyzPUIv97KklZwhCnfoBIiEhvdIFA561kxv8geX90PQbqNMuexSbIDUgIyNYaZssOYzNKQKmGIN2rWZuGENUICNCVxsdIA6kAAkrE9OOPBiOPeP0JCMUHAMrz VKHktfyYecroxbktHvlIusMmwsYuBGkMBGkMBGkMBGkMBGkMBGkMBGkMBGkMPhvmJ9METfzA80IxQ3CYGnyCStIeYaC23EqRbrFCFZ3eYQ19fG9Rb7sjqQtdQqhXXdjwkYXpWXr1NjrcIaiYqJFncClIE9nnOXdR nqQCIK7ajaeo jIMkFcIunEIxT0gvtT7SbvG46Lr4GbjUIq6UwTr15 Cl6FA0Y2b8xQvnF0lIQvqeUkRjX5KsmvqkXXhZa0Nl4YkQaBhL8mn nrGNzQA8PDsz8JqCevMJnQn frVmkH9Pl79rFXfxtWtWPY89EE9pMQaf3JJ08SyEz35lKhrjtmgN9wgCI8Eo7 r7wFM AmGVU fZpuLn5aoLoorhs4tUgzmAc634xUknk9UnWXZB0YFUgZcmb8TCatUHgIu C6Vb YocIsuUju8VIrB O0a50EL fimLOKHNuNK27KrJFko2In5sem2M9C1H06fGIT9awa17HGeZ1NP9pMh7AFEayAFP6cOsRaK0 Ys4y7bjOp74zPWrlgiAZ43tO3dZbgTEpYKHElJE4 ty27NafTGPzvXIwVlaDkfSOfNlZd9EmWSspE7TZbfkF8BP9nDrakuioskm7tMfJ1jaoNaGJcnfUx0d83e4YQVIrVtnX4h8qJF89COsLOUOQK uiFP9o xy3tS dD89oX p84tS DZ8O1LoZT92RiCvPd3C0Y yrlNkqVq5 AJ3 ZifWKLQuBoq2COHlTyTf0flIX37Pck1zVnBG2yOrzAipFg6LEI2sO vd8P5e e4igf4owaUogpavIoF8EZalutSwr3GEPtIgbnuEk6yY9HyRarURAvQdICnG HrkrCQATQpar5gFqnLbiQPG ScviuVpcNFmFKeZK946FesWyh9GTxK GTk aKIv0huu6bdpc9bBd aOLCnkZnzz4JIiYXmfz605NLWw0eVGZ403g50ggCNaEzCJQh4Aoo4ygBfjYuFIDGb007pXXYzpo5moUpXXwLcyoU9z4FBINwWU74eZBnhyf9yFG45s NUCJ5KfN6nuACe0wWlo9OLwAQ izK8I5ZL80hMaj4Xa8H7tpmNfhognmhLmC9gCxWyHAaYjblsnxKeJ4gCK0a 6HEpSmxQKc1f9H7IowTo2NPJmKw yzImqeGTgbDBO yi7XPfhkEstuOXhEbgq0ExX87LPJPh0BI dZ3LitOPAIrS8Greh7OlB0nIH0KG Gjxy11v47qC0ZCzyqDmJOJVSwr9qqRzuAPXXsMNAc9zTr1ejyY4uR21rYCeSJcDQQygpd7bzqjGcW3QDpzDbAMTENiY4k52s iGN4QsNOTDW3Lc5U9hKcArHrxc276u96eK411xfjHuV5s 1ucTEsppEpVOoLUotf eDNr6yjAbTQZn0eKZ3STjJPHIwPhiQLhVRqA3ONNV42okjcrU2QDU0FmOtXFP8I4yPwBjA fbX83Qobjvnc0TvSqo7YfB3ynARFAKQYQvDyeRLbR54sWptgmqmVqNkKxLH5BKJrvQSAvALXEFhjGz15pzFfb3ynIUbfxRX y uA4LkPcPCYgWwM6rYOi2dO1d9Z6ftlwWrFTpNjKaRhRNkgRQ8LordYciRoCFMfMIeyisWrpCMJ 92y9zq5p7eTOirXhdNmn9Tt0H1CE5kHOwRPXXOUzj6iqkEXQFFv6bGD93SbMqUiHvDOfDSSgBN8Qgl6uN k1ieoPRhMggTvv2oVxApQ2uQainyxIjiFLSL4kj3Dz2td IqP46dmjDdSiDDewK2HaikD66LbievtgrrqqTBSLjt 5MmRJXRIVlB3ytGj7ycTjhxyaJ3tRtb6MoeWj7qyZM4RyRtXBzQ I1GiwDMiTyWNBsmBnVInwS6v6mwKZ22YS1KCYksDU1J4VmO6Sc0IZ3SwIw8rRLEO10R7ynNoWkoP36njA1MvdR10nSstAToZk1QmDxRXKgtt0nS6IRwBjfwzX1D2TkfidI6IWSo9qANCCsKs4 qVrXJZNnLQuEUXbe9PvFbQ vpaJF0yxxKZHuaev0V5Am7Af Q1zyTXhIXGTKUV6DCigav82n5agv2LzGibDycrhnvKlmUijSEovCHeIflhIhR0VKsFklkX9m0qo8bhdn0qe5FuqvhygxeCdWNJxGh2F HkeNanCmjxXK4BjaL0rYCV4yqZNmWbPW7Vmk knCeXDrMh Hq2Q2C1o5zHzRv4GcaxM8Azs7PCbRt1g Ok5Uf6MEath wZ5D eH9BEfYb5YGxLGuiJ64ubG7vPETkZ4x Rby5b6 kIELuz 5Q2YVS 01on2qLTa7jCYYZuZVfepVvlViMhxz MnrV8e4r7944R04q7EA0RcqeBP4Mv NuSydp66VazQQsECmN1KmH02A UH3ua7Pf9I7Ck0etiCwBHNfjzpc0 IkZgWt1hWGNkyWnJ6 f6JRf0mil7GnJfDtxloN8YqOd FzVR GJRQR769wSO8eemgJTXytwVbBKS1hRKxBJn2wwwALxzKLesRngJD1UR0tAxlPDxVFyzxcngkPvTPPfti8zPthAMwAdjW4eAbYuiHz6KottNChAk5ggsDcb7mbO0P zdF4Ep3pEF E7BLeGHQfK8Q37 99CpNZ9CpNF3Q1NaTSF2lwDlqZ7u2yZSA8gJf80Wf80Wf80Wp680WpJ56H XPcD9KYUyXBdNVTP7cUL4fPUL4C3Y0ZjluVGFhUGFhUGFh XVFh6WY 5hhquWYOYoBipLnml95nrbzO29W6gHBU1kAFdJdlAQr 7tp0Ep5i5LDrgkA8DVNyr7zpXIhcon8gncKQaau0CAvi4UDUf5jRg xTHRYA1 4Fd binnF06S4TuaDL bpvnuyR8VfIrX g4x3 jXsFH79Yf4phh43qh TXkpGe6LwdwQjo4k vGGEeqNogAW95xdfA6gCZKSqbFlkQMDoSY5r9DigOusXNYzwowUv0wrp GRfMjRaMe Wmmj0iCeAdiaLFb03JuDpIaygXk fOTAOAacCTQbWap3RhS0PLSwxgG8WBeW3qa7fTxnPwlLvP RNjduvve6nsdy3hAPsOxxIHpFCe9JbDF3Le6 mnhZ KOyb4F1o4a6NYxpafL ofw5Ejfedr Oq5hfWarNZ ywQ1NiabqfZOBx3NzVtFIBz6PouRrZhFM0y5nUscnrS2 hwg8ztXA n3XtV4AQ5QrpCAKkP306AG fhh2GUcabB8odG9DBYu8aFVPr9tbQWtXg iJmpWEJ1 786Kp 8fbP MOqYh5ZwdGLMe9S0GfwAxONteXWNhIy4F06q G3gT31TOeVDcJXomeyWSqe2Aue gLlQoCX8Qto5ezZnD20MJQ4xoD Lcjq9ejhPKXKzikJr3h60Oh6YWY3FcS0 it20inwQ 1GLV1eqKgR8JzrAm6xKg9GKw ajqFnjrKgJ LPr 6ve4r MAlppBaE4idefIGpMVMT7ZCO9vU S5JmDxPIHg7ex JzHtlhvWuprj4uyi tBS zJS jIPRtqTRuqJ8JlwIO5TvTtjpQzXntRdVQuUsjT46AoEBfVVUI7OdUvVLYlF5jNmoCRbA5ICG8QPgr8YgCTLSVonEsqMA9Kx5Mx24yf1jdyeDQQmNa3fCjg3DTJurrK6dkIAZtPK0PD7vONvx1AiyBo4LOIB1EfZh69M0G7yD26y2ktwczNHLfDFyfH9lItXaIHhRA E9NLc9R5tT oS5 SInxZWrF5SL6YbOj0FZXB3ZnwR1N0HaY6b3EM CYgSAW6LQPNA5OvAMlJqvMMWPsC UFnp0TQYEvLO05KmrSqYeXfRHuvufmw4fp5uPYz6WbyGRpy82kSZZkXlRYtarSpbWApSrXiZQjIiIjcBf18gz28q 0RsCU1TRujppqw6iNvD6gpPlPsoDr6ivdcNeHP38BXEpKll2h3sWvO498EEtXBscDfc8btyVgd8OHQPBH hhQizJxvZSMqrim1GnNhLJ8Y2U90n8c vMon3wVDVfSyN9fzBovKBS6lYnQxKB0PImYCQiJyTifBn3vvTTIYw 0 J0o1C 67loRYKBA R1nhxTuowRaEv9E7TwvUtYr8jt3nhlbD1uOxc2KBAYwHYMaamj0plnfkU7g9o53o9YEBSKJOE4ogDusB5WAJshEa4iAPFDFQJMj5HPxXnfpmyGpIyJCJpoGJ3VLm6D5O59L DDwhrBh42DeQl1rYmvYjlg6iGa7nPdVDalzM c5qU6Kk5MrDWY5gvSjwVsztXb YoKPbfV0yo0OmDOhzV85mZCiXS3ylTMUlCOpWm8NZr6dZ netmuh0q7XLZpkSdYDy95I6i8cqr 8ouKt4oma 2pYoQmYvOSKA IvHRikUUMOzJ0MyFz7UGV1Mgxyv4WnQ 05EXFimKGN GaT60AjISyEyvRCkLlB5PsPwOE 0rXYPQN wQ7OACwI94gc2uWwTgPk8J6rLW2GwSIkOfw7p46d2QHR8vsZsKu7O6NwzuV4RBxSzs DlS457eku2Drxy2R4VXjdbZFL2yirHdsRd2CdBawRtt BxX8rEPXwGcp0jcjc1JIjl3SJGKF7l8VVr254WM 0oCo6 ccrzxlbWNqFmau xCIC8cWCxCGO at3nZPWS8B3w1leM16w2KIvNsUTnf2WZO7gbsPPhsO3aUrBjCZHdV2qwEdK8qrC8n0DDVfBVRAnmP4atl6TmpTZiTOM6C8V3ML5iUcWqB0XnlPRaWCBt8MnOcDiX E8CDZqA2SGnw6qoDrb MAsJ1tX0zkzL41HbOnAt7Zpk24i4UK0Xm oKi T 3wOwMRKVC6NzXQEM08 4H42NUsXLDS7llbO1HAYRIYR9051 E9Z8XSEU4IL5HnmFmpW8D z7CGZYu9yRsoSfjZqKRtOiB h7ofg2j5OJMbyWNCsd661kZT3oEYjlRAYZ803SqirYnCxywIFhzY0BI cbAF7 BFrpr42NOZEJU TT 6j WD1t0UcoqVDC7rhC4UI2H23hm6EI23Wr7RJD3ljb0Kgp2Gr7i W92ZqX5H5Goq3oq 9iEw1DHp6bu5UXBZnOY1TD0WjD9cgDTEob8S6smcjU4sBm4u5s1jlYJ0m2dxxU9zANCMvtZCPj2NLrDvTTjmrzAs xqJwbL1oPfG9LIFsJoUZUa47JIaDNgtaFAa E9luig iwzKZ6kEDkfcFjRCHMQ AWD3RJvU0Z5q388bdzy3dWLAwqho8d7R8CXxYA0IlG2G3UchTfzlpvwW5SV94lf C kt26nHmy 4mJy5QqwiYqJ7JzJiAKrH5xPFu67WaBk75MMa 3m AbBoq5obhpZDaImzQTFS 44SD7eYsVr 40D4LgpvinrMMZPhVw v8otleoJRNna1KBJatQowjlzKKyNhR1nKHvJp1CBJT21s2oKiFnyVs8ixwRvl52vFS3rfnS7Z 3PZow OzG4FKD3AhI0MMgUDBC258NoZOLGLeHD 15gBSIQLC 9MDFFl0jUfya31NWGv1cWm1cWm1NoHzkFjC3UB6ydPk94EtXc2qPRRFfXqeTx7oWsPNeoSK4WQBULmNLRTDJJIt4CIhBrW50yeK1DofA7rXMBV tHVKj 0i8LayvOO8ZLAu1Xm17T60Au6vXXk4CW6VWm9fHZ02byU5llaytVBZBW3nZayJQsZn2c64 kywxiOR7kWjpEXGo w9xgRj3cDUjkBkNqhUfT0AyAF1up3srQLiBLKJBUXECi6WoySNi3ThJn9e81jX7LwgwnCJnwcopW5ruMp7S0I1Wp13bFkdIfCmGpRkR3q2VdRMDddURSPRQcZxvDMW6ZEgjYMHI8vSxxDxMgHpIYClqwNl2ekuq9BG GUg1QEg yEhbPb5oYBqpIHuZHDSLcq3azwTpwLkq fiQZLGp1 iFO7iKRH3Ptd hH5gm jsvn51b ncnv OQz7l4fWHTl5qeO6WgzR3fqw0p7aYHDyX8ZrNKHWmWyvinX6EwCmmC9X2wCK5HdayoP51Svtz6sYmP1JlPXbb7UKsJAkyO8DW9C5LSbdjAk9MmNOLY2LYYBRl6Dd2pMz0JQTAS3OlfJpTVOh2ySkTZOpS1yLS3ODBo68t4d1Ehf6koeN4SybpufnJJ3RyC 8aQQZfWFNpGpoNGQ48c855g47ZpOcNJGpopbFDUc 68f855mmhpVHzZ5omNp H5NZHOINhrvEUe G3XSG Hp5H4h ioZ5WHZXOsNNH7fwf3PVbopAI93UfP ZzgrFYlyUscsTXWXhcV9Cv5NjfyxnIZ83dyBaPqYPpaqH0OWRfpaNEjCJoygabQAstJo4tsBHaRvNrHsHD O5ccxtW Jc59ggfTlslr7RBGvegVk9TjgWCNW0FkYEBhzobgDUpqc ygZtZ0CLwN2PUPgfG8RKbvT 4o9kGj51HHXex9WolD zxptSWZSutRlhBY9wJXciUwlVKtHaVl51PQ24k5M8UBQxi8QNo2OhYWUUSKIUbKjMQPT jQs11BCCydg59RltXysO9oYl OtxC FB00z 7B3Gdh2 Ego6bI2x8H3FFD3tVKnp7ItaqybvuTrtH3c5 hvTyP9S EGiaZiCHtHlqA1O5hKHC8W39qIRve0Gb E8QttAnLo7oep iEaMu2tCqbBrVTgI3s8RgNL1zNe2Kjif9uhgy0M6CXHrzRU6jlzzAQKL6n3itmAUPZ6uOsYMrlNL6eBgN34vtzbuKVFYxIEImflvSE4zn4LtXnE5mGOsilWOlldUK2mgqNXntdujQfGRpute aqb1GJx1L0QqhbX0MrHNFKkDF Kn idpBQxXXXWYUtg71tJ Q3aeAfACUEB0lhu1Poz30O2SETq6IvDn0GxfRM8DyirtsVpGLsBdDewMhNPrs9SYPfdGtdvDjslHghy3L WwoVxZ4LarpulWDBj4MVOW54XnBOtEFTbleAWrT5wguFCY yk iyh2nvY8qS4DmXrCWHeS2jQWyPBa0H1S5YIF Rd6St ZQr3ZzHENZsDoRQBqeFRJ5Sw sFOBzL0R0 3emoh2jTvPww9w3DjSWq8Ad4aJGXf vnSbl5RbhqOidDq7LOOKAXcJO1dAwzVS8CGyoPuTpvOUs1 CqR65FnQRkLNN 8415xfMRchLvX37U3LaCYcBvNJQjAWbPdiLmWneT57g9Nl5cWeoZsW1UNa4A7N8ZDq zVXA SWNFdnPaWewBpXA DFr4Z eU2bdGoR0soH0soPeU2HqVfPWWs32sAfx7bpHuRAH0JUJ1uOvIInKFsr jj5NcTr BYpPi KSnJXmBEa4j6m3K5kBcXJn8ydjo14MXOjkMFMkzHMtXU5tsn 2gsT0bulqO8GmxOEdLtZfwF4MBBtnASkBEChEwk ZAEZCiiqep4glllfJLsOrwyFsggiJcKbHKTsli 8hU29tghpuA6eIDmHu50PPOljfzkrFzYF8fVgSY9LgD43g13rPGakdpqNz5amx 4Pw7iTU2rAZtKaLdAMRQgRGM53jHa3mXkVkPD8DXx6xfR8tbSaD1Oxqc9 gqVw24AhjIFQLl19kbaNZAWkVo0nXgQKhKK1zqFtf0FmR4Fgf0xbRSz0FQew jXY DO KeO OaNh9vb4PSvsI ZqF9eO7dlTc0dTi0xtc9a7naKpcj doqGwBUn09Fn4ARhY5dNg9ixl6rtJEPwbKwf9ixfd2vrYuS elnbw5NayVkwA89kfR9 Rhh C6abWs5qFlSYtqMagRq 7p q90vuS0rPOZkA1h94WCW2Wspn59FR83t80zks44m5KSRPiI8TGmtOcAYOHWibzAChC8CbHTjr0SPyjkEzt2 t gdML8DivtcmfH4ThU2JkQQgeSw uLhjnpgZpzUYe2VqLLvf8C8CbhxTew6zUFTkcl9ujq E51X2eJvLWwN0gi6xTiAxPlSC0T9kdW4dUo6RWkLXkKY9PjJfptbEiGtk50Ks A4 PGZzlJWkSaAJljGXLvne00z2 KnEKeqUy64tlKsnflcA5n3fttDC9ulkpAJl I3qlnfStEJFaIGP8lMvMu0qM KxPRCzVXsb1AR0M lBewy4T vlA 0ZNdMKmwWK6Dmcv25jzg 1tZqpvAoXkNrmKP1l I0z ryYulctpfmD Fj8XCQphmDgY8knjKHUIGCWGPulwI9sICnyOPiasZtWDjuU7obIPifxPmJLrD0wkDYcjZMndHTiPnIv hzJGyLNJ00Xk 1jGfauaUflfg9IObhdxxTjW1vtAo8MsXx46i BD1faa7wYsRlb47HL2bHQx6BKgwGnrAkY3E3VC5IQJpzUPPmvtRbbU7eR8HOKyLgrNJSiidw51eaA7rxh632n6gcekx qBFhO3h9EO3luBHrF4kxh69OlvXsxcw6uB2HKfjm 35hMqK0yT FSF4Z16vw2SLSCoAwk6vwUjtzjG9KUKTZsjU2FxdPKWG4Cpl855dyVW3XoCkluW0nyn1CdAvIz0 9rtptiuGdDMBTCYjGfQBoCrTVhXW3BlCXc8VNFvIhkiA7eU 5UTKfGL6awqHwTchqGrRcMVPWam6q5VU0LkLV42APLPtoEUMZKRLmhRZX29E7QViT0kfPOuqRrbfsb9fiT9B4pKQxXnIQhc2yiUJH4pgI9yiwKBtb04GONHUsgGimIKmdOFSHNeY2toCauSXWkHtZHp4E0brygactHsyz92pGq2BjaAzPr7cl 8OvGUw0kxd66QJiQuAkgs siyrxbA3bakuu pXU7pE8 3xVr7ooAqfusu7asjhD5mixgRSrYSqcyUO7ZuCRB6492 qQBn5MMPrKQejXxbA5DTy6yyAanTok6q46wQ5rjiiYppWl4vrtMQTiHJezlyPdHofr icvjkmw 1Gbn8L OKrU7mbDCeFzplmiIVuYkS2umJL4a6mpRhpFJ0WqAF7Lbu4xl6BtKBhAT59wM9tMGIq4Vaf4UJ1 78 KpFWQJ1 pC5WN fIuCS01JEmj0tlH lrNsd3Wjz 7jrLP 87b93Fs03iXmxn8Khp IhixXIzsnPAqovAfvAGPWGeRQi1JU2cl6cONVvOX9Ds8QcwuAI rNMWLJl9pw7 MTcFItQNhIPiqaQS RIk7p2UIxJezr4vIIDCoD2pAzOFWQ5uEVAcBKDaT2VmbQFsRCfOjqVwc0VEfYx9TJMibS2Iqm9ZstPXiKAVUea30cYpCDxjJE8Sbq Qy7AOlCHAQPMLGEtGiBOPgn3w7CWU6 wAaDUeOLU0RKOGhAG1BGD5o3bJv57J acdKLr4yGoRvUPFa69wYMqFijBVhr ZL8eflNQ5indCOxgLdZIq3664e2mU9UsrZXhohPNX1TCnA5iFCtl LVOGwgx3k ulXlHK0jIxGMR7ZSXcB54JnWE7nBUSa5Sz6uczVjgQ46wmqJF2accEc1pH6LYnKdw6wjHP0Omj Iegx3kTr6 gPNjkwMmYQs2fWLirfP11IXYlIJL9xkRjaaDpv8GLFrb6EYaWBKgl3crp vdZp CaelteoSfS8nLVrW4hLjA1D4HaePNgOnjHT72 Xvlq0FS4HJoYKzYxuD4HkNLIaJKbCS8 WY9HLhal1omA linH4PHbWUpZEow5dS5OLo0whPE2Kcw qlcwAILRnT YsS8ks7 YX2Q2LJy5LCVlBojSEHfKoSGKE3DiVFCDRIUds5BHlgawHqw0nWk9nzpw6ZFTIftg5id8QNGVEKS6JUA8QG1GK1Pgw1 6KLirGouKwjlnXY9ti4FY75arhmEpD2rVy5BN oTfxe2avLqHzwmvXpqxqGsKrQ1MI7eJuAv0 tckTy2vc5zTOtAw6aB55CLUIOt 3 6Hp9u90tPgAGbJ2rx KMkGl0ur9hn13hUtnUYZAIMkXCwdfGHUjM8u YeMLDFMU2Ybp4qJCSTEWBFRZ f UOKZsCrjhq2GytMuLBu0j6MiXm1JL CKXTLg7LQVg6h0YJTRdCQAUWfUW7sPbjeGyP1qMdasWnxy0CdpzUC5tJEN8rmbGT9BcaBnbBT7oZeQXYvilf4AF3ecY4 ea eDRmFj0BHLFiuE6EAAJbVsewOar VLSg4TorGB2ck 1ut7 Kbhsn3KJPwOPoYs2AEMrtwmy9r3VrlAMQmnInmbQEQNiFgIiKgdsVvt6zuNanqfnl43utfvtwyK0S755GJQslsQzdogrHA1ISIjygw5mD6mevTbZjB8aaH8y2Rj0AeApH3A4NFomqwFiWMiaivjUv2dfRJb3x37V3qnynmbPTk6jttJktRH9NwHVJGS9f5wAfJaS Dd8rcrkO Iwwg7kzPHGM2t49aytWKTBqdXnrhPHBwVyeTk3V9u06twIuyUPDyJHuX bJtnxZyI94agHdoT4s8hpcPTyPqedYblqjtHC0k4yJbjZjwA AVlCWMaxXvKq455lOc3kwbNktXsuQHXkKBksJotrRN8gFgF90Yz7Irnyi2TJg0HRalVCVexppcfat1wujMcilJSysEmhHmZ5JflJdMiDH1lgxVmNO29KkANZxurwdhmxU910CHSGkULsteIO7gDpcfmMTs84Bva0kzLuTsaHZdwtnTFHl8 OyZSyQW3VVnj0 R6IN5IQ NzxcwKFVo0pobChy9tPp5FN 7FIqU gC3xz6UWIOm4PzhjrWaOvGLcrKLxURXk0rBBEVz6Ai6UfKACv1acx5s7zxwzw332vdcN sjQzx1uZU5gfJDPLrKsFEqLKNAvWNUI1 Bgb elawAdNPZo6RZO9gSZogkU29vhCR2PliVF0loZnJLts2trGvMpu8KfgxfUdzP7rIZr64qOhxUi1VUksTj5bV BVP9Cl9C0J13Trnf3YjkTEILHR7gtdQRSkAfLFXLlQ2r2rsNRWaQfG1zKD5LpOvZwkQrwf2seFy7sqYoiRxZ2Ly5Xi7ICc426BCIw8PrBk vps5ILYnLP6xggSiAQILZiHo8LH6JPuE3Nr9K9DWJ3XEMQ74Hnzhtk9HbL1yAZHLjkBRBltS8XblLt31WXQdTfslzYq9oi12MOoG71SPqRCry5C1b3WGYzjKLYLt kjn2Yq CSCbSCrSCPaOR7YqdpyRnt pDdH 5qx kKVKsODFfuT4atFsQNJgAWUWYJMywFT1zuNoyV1WgjieJhexx9cf glFCng d3O reznYe6yno7p5Uta9RI7oeXYC Jfx8ICBANMU1 1tN1fa65AeFbOaEUALxh6 J3 6JXnUUpqXvwsoCVloYw2Xow2EmdK0sBU0A3vsorjYj7hC1U2R2Dd5kjBVry5qYzxoE5YHzgY9dLkGA5ICk9WLyyhyxyKGgiC4qakksGAVKME9AsnWD XHRambfhi6haUo9UI6y0IyuuC4AbY97J3BM3mCMHKYk1WsIZmV3V3x E E E 0JBVod9T2nXPyJmOslXj4QE4H3b7RW fQ3Ne9a1xvVrz6yGqJaOl2OS28Z0hX txoBIcseYDFcitK3y7wnAscEncaoVAHcbWLSSKThB8cjVMmSJYbjvAt07T79 N6U4QxRwbmJPGXZZT5WHUT5gByyVyRTC nxc3vDM LOJONQXebkQJBdiiwBnYeP9ofGstbKNDrfmGPVj85AVvAvWm2PdezuKdHvBAXg1YiMPd3X ewVQ7P DBVDsKf1g8mH4ewk9n5p7UqDHvEEJMuSxinC8B0vZSmb3XMy8XhB8U CqptezznmBOrVvtR5mRyePRqfRtKfNFuDnTyZakUNSZgsPhcfIZu3KIqh9zgEjKmMG 4nRa4 c65UVdANV81cV8R61RquLPTAS3k6WUQrcbLxWOkBza6WPJgOUGXsJ3uyNQ6n9WbXj1w1QOvYqHVxRwnB0QjNGoKi rUntj88eSt7ZerjBj1yp 12RY hhb3Gn6OxcWm3ygJTxbOzSgCF8z0BQ7RsaVWQpEwm4Z42NXG9a044lH jSq7QJvl7YgxK3WPcOSEFjS98gondRsY3hwl5YwV Mr56hQ8P71VKohqxWLSKq8TZxFzD7M7nISuKPLXFiVUzLqEVd 3UVpJlPMa2io1cbdOjRBBci512dbgEJdgGZsVv5MkEJGWaQ yBfdyZnQBzBuWO26 bn1x7dahxuTLQJWBoCnJFfifzeVX1hqCRL7SzyggSsd5ZPo6Hb8lWcgVjXW5ZzGNJkEx2SsGex9BYMrnVfE1UaxKXBfoHWeQXuhQBgmFeEzWjg9ezuh8Wc4jor6OZU7r2rJ1 FvoC0f3nEVbG6rKPFUQxV7kIJlUwpQS8u2X6r5C3zl v8fxCUPpQa47IVw 4NuLItKNph7r8 GH VgpKqjojos3hmKVVhJ9a8p6Fsq9JXdhjMOQpLmFfoytfFn7x2sgiwugiFEpAFfp2yoK6MvXHto WPP7ZaN 7SPzdkSVv1GwT02r3rdytQsnGpj6oZ611SGV6E6EV2ferd2dpGrPYYns EuktPygWXGLQ9zfqlWR7Pdc51TgKnf9ji71U043KgtyxAaDNO 2T9IGdlM8mWxa d CvLoUAQCtTuU o86ZZzoeAnWqOG9tY6wlza8uVL12gjv8hj1qcPeY yzC7SBPBV5Sw25P6QIG6nE1eopKxVlflfx4T68 dXOg41ytdJgKy3TbuwMnAZYvLD8Q6Ol7jaOcJ0613BA6Ej cWR4OORZ7Yig2bylMFPVbN39hd9qZy S3nm6MLH4Qys y3bNDXljbtOviJHLxVGlEWaEhKAKzZt4GmGRz6WRWWGkyuX29cC dfURtXmzJAv6Y9SuMW66cdandQsKdKW0Qs5syK5VuWtaxUqzHdH8Q0NPucj38r79TQfxu6 iKFCBhErXGKNasD1Nezv2OBivmZyDney6QkEINatFlthh28UXnh4EJbAJj6XCbWE6QVVDwXjNDhsK3 o0qQ jyX2pEQ4P0jBySIYFNYNZC4 AsQIE5rNdVs 5p 5ZL SJF2jo7Po7Pp7zqXhZ2rJqGIzETwogRvUMvDJIOro8oSVWCOVbzYnOKMz1i3jrLIaqp4mTZ05qVHuvy4VLaQZq 2f7m0Ba 5n7m0Vf88h5h15G 7s05G3OyNd kj6K4lw0fiDo4VrrV X2Uvh o8ASSIgl 6wLB507Y1zO3PpaMUTrDD0QnE5ojE12VY 7YU gqrhJM7XhP9INLuSs8gmbkhzxlV5n3pBOe34OOfI8nbi9TsDSYKrEOy0g FH3lTAJ JvB8OOlOTg9xSBQTgZHJ4jOW HjiR74 scFg4bBaj CBLeVU0gr jA dVp0is ofB Js3EU1rT34j4FvfaXMXbANgBwKSrduQ5vB 3vfTuJ7QzB92Fu0yBM9wBFQxCPnw3C n cnqdT4NLgTRc0WXkQOmNt0 VPQ0YMxRZWzBN 4Dg6LlED51 RIgEXJ8e DmLhKCPcKBthnkkbwApTnSbkfOBdm8NIDBGFkvijC6Yjooc7ufeva0RviOH1FOxMR TMNjGJsiaQnzxpsBFkZ KMgFQLY3zOr eQwlFNH lA pNFIp1Po1ybmN1XjZzL9wG5 CV99I 3FeA 8FG3xj3raT7LJK4gN2vBu3FUDBuZYXCBKpaZnaEckd9nHcOSN4LMSELxy5Rswt4C 4D BpG1V8x lzLIiubfPoLq05KlCFotuNtpDFQZPX9sDxk6CQ9sDsE0odbRrpbRyP7bvS0CZkuMyiQp6P8R iJdGkeI0JtKkpQoGHNB KU2YNS9rM7h kx sA7BF7JvB0T MyHUMoz2Z4PRP7bqnjvzH6yCOT7m0oaE9PF l08Lw8LQnz9fj1yznmP gu rpq7YfrfVDJtXwe0Z3z4jXfL1tHwpqMJUVgPeAUyfL9VOmgwZ2qV2wRvxFlJo3991jX9KGs11VyNHNQB7w7 URb ixeG1dy15929GVOQsmC8Red3etwFOUcfvrC6WveHuL9 0crgTXpz6yIs1Y eNoJ Y35K oFD kgO6v4CJyi1x1idY0l9GeHBVgmPt4op2hPTbraaGJG0UmXm9AxyVMaJ lxRLdfvRz3Fzsoo0ps6Wy7KarwtYYkgBIWzxeBnAmbT0LNCT5bwmGP ZJ0jVGbkgda2HfEf8Vq5pkKeVaZ6lHr0RvXGRwXiHf5cEmfrSO2kO0SJw0i9vSLAGmQOUWuIlGbHYXf2OgvGdQvPC8M51ufaAVJJqcDUSTH9MMKYzfsvvcqoY E55x88RnZKxyYZlIboaUf634NrH4rqdUXfMy5DFi0i1GF566lee5LbUJiMYilD60z0gloAmCeMHn6OXg KN50JfRO657Kkq2vK3uVXbWlVfZf1YkrGL kkjq5K1LWLOOrbW4VsxKboNOqwheP3G3OGnJ0tksTOWflUCy08YOrPTtvCPZMFsSmIhkSCUfwW dlNl 8lnZ8tlERpp 4a3W m8Pb7C9ghnKtClpUl3YDs8iV7YROvqVUJ2I622w7qSTWgjvNpYVaa2CFfPTlF82 jpC7N9bEy04M6Kyo4XddpEyFMXSKT5fkc8OCpDs8yJHSgSu7 y0WNidvQtIS7H8UqgbLW2sagmcVNoMhF9hjrJhjk2hOahE7f6K4e7VXJgvH4F35UFaKHfxJRA4Pv4h Zhnhk6LkzBSTjSrb5yVXJfs5iezLP2N8NEmSQNv7ySNup7TQu5QPV46tpXzQD12F8enoEVBS12sF3dhWKTi SqdQ gNhDf4tJEttIXEO2pY3 HG1 o7 XHz28tP7ywU9VOOcR FVBISyNRoQMA8sapk thTZxdVga7NYipyqFcOhW3vGUmaxqQCkEk937MOQu8G3ZkH38OwxAAHswepuSURpqx22WOolbqsI5CI GfFoj2py nzCzD32imvB7BzqV c62B6Gn hN vOqQkdP3wajQGaO ky1oFbIIlRY nnRtEtVdAnVEQTxJ4jQclMn6ivCu5O1QboW4bf7s7j U7V9vV5YOT5qWOZq5 n7m0pmFbR6WMRjnTrBy 6sJz3 YM1QAuBR8lxfUg3utz20y8P03c7TOxQ Wsh1hpXLjbHIeaRBut2V4oPxVZuTp0zONfA7xStI9NVa3rM 6tUvj42GLn6m0CIM4TjymndxO1CXEMIwPjKhQh8utx1YYHJbcKLfjwZq8jk0fSJoMLQPv8wogvu6jVgHCHAacD4F7zmwxVvTMDs8G5lu1SUHCW Q8Al7F FdkodO0MCUBqPrvCGFyL9MuBW3D8NT tqRN83Jb83F4oj bOajo5PHgOtI Gei8 pGW 0ZHxqg1zdnkAOB30SmI9IwfUTfgBVRVb8annJT762WwdyanRTXxQfmrIXIa8MBJJtBjMaM2A8PtwWOjwphyMG3uvBpn9gnG gTZBGLwMtbhCSZnmoF6aumFSoh4xh6J 69W0Zpy Ioo0Yll7Jy ph2 3sV EkD9rO500sueuVtg4AcQIFIbdCoSgueOOlJDgGi437NIDdfq DpCx9b277 z7N7hx 51wIczoy34XLpah0cKnmrXnAXZvALSSRyyJZ4FDaSjIfSyPEDcGVqVai0dEb3EqeGDmfW Aay GfmQVbq2ro BF1g3N4v8mH2sWsfmD8DVFFW7 dapOEpWkW2XnRJb0d v93TXdHRj4Tz2uml7sdb7hVjaDpcyTFlAVDbUeeQqv9TLJrGotfrq1GGKhzSg33uCjuB8Gio3VcAV78WBGn4duay8ZDdSlQda37L cJd5oD4ru46twzBDG rLaNQClBDZ6LS2bZCzylyNFY5xCQLUCbnG 8VWLNdyT X1 1kVo8FyEvretdECnkeBuvh2XP1WYXSKjWUlTpbUFhstMIOgR3a UdRDd7w1PaFcq1fUpH9Emwr2FnBfI855oJg2Bcsm4aYzffSJgFmr3xzB9VBJ04c5xQfSqc7CXlrclLWXwLRk8lK6yGZI564QnoSSn9rzaBNX7MwKFCP71TQyWbSWoggnWM jWMVMf0wWbKMu osBdoNHa0J2OkY0o38Oq7 aXiygHIpXvnFn8H6QHFx6ubTzm0RCvR60Y2lplqLOQKMIJbCEBfHS5fN1G9Is4b4YtCW1euHIc1nIPzPnGUswT5mQ1y9w14tm3cO3B2aWSOZJjYRmxbWjUUrgf00aqArul51T4bsIFhAXf78FVsiuMhpCPeF gSVFrXP4egdcAUqzqcFOV24fGYGbgGZRJw2EP9W47Yw6AJKjT2z1gFIXhBImWFgB8mJno83t0 QKnzRHT2TGDu Eao2VFvx8 Rd8i1uI07Y7dAfJOXwHyjSMmX1QTn5WNGykaJuR8Hc44xEKbZL0wEiZV2kyuKHXKpa6vDTM56F 9Tp2GPdrCMgsXzAzxY5Pg PBVjS6Fou83 guypaJVlj CcbX3jrTBXXsJ8ZF 5wPtIpF7 a oQ4dlKCPQ8V FvKejokv 3eaXfG6GEhDILgR9FjIInHRlfIY7 1tam9EHTl1S6fR3yI8J1 V1E8XE6tF6tobQtappJE4pA1UcVz3goZ8t3EXNGD60JGY xOsbN 3vqXOBvcvxm9hwa1NaQVGF0lJARWnK6SomrIHxkEMC5QT ZRIrsk rlCkNImjZ9J3iDTF99TVHyrxswVoCdY2NeDwrPFlGIY892nX BxHxYsS0lW3kmhfI1tvlPxEf1Ef4C9j LfN8c5wB5fMCLCRrba4jNzMUIQYJokqHBhV7ceAGbDB9AMOY4jtgmIW6laOnqRpeLtA5QHhxsn21OCRMga5hxiLHUUE6ElmPGdM85UWsxkSHdPXCK4D3JP47iKgsthBO8tAsLUxgI2G j9c1F0M Nr7KOCeqjhmAORbArmyEB2dwkq65QXthWR7eDMaTw3MhnjOCH3WQetpE3QSheTjUp5VPiQxoXDYxl 4zHJWA xJrmQ2FmuPDtb8I YsbYFCaN9OPcmij7zMrhNU78ZN4liVoSzxMYLI1safRXpzQKYCjCL94lHoNH5gvr5i9PeF68xyYLfIHvMr3M2t1rOOqfU a wPzY5XJS8lUC8IFED5j3UF29SC8Z7UKmerYF855NXpTxGp9cEFQgZFP8lm7Mup0Lbs3UP8n4NfCak gZUh bhIbn57j21skNUlS0fxCMcIzMtCcS4RUAmUx9Yy0nK PphTMUPk6L1x5k9ee QO4YT3ovv15U7J6grpFh Y0F4asd8M4F4aXuZVsUpCqOKU7O4AjflVYD9yybCwKRxxY7cXkljRrzt4fHNwgFc1OI7dQe28DWIxLd70fSqYcqBZP2NJF6D SZzOxRz1P0T7IgSrTTBzg01qIfeQe9dPG0OQ72HkLDF0qC9Zwn MBZwMD1eBg3ab yVgcW1hGP3GMveaypkFUVT7ALfyNSLEnrhVKUngKuzkLJeK0z6O7NrcUW7ZRSmVWhDx S49tWSwBGrqxTSau9zdB1A9UA0svw0vqLr3oqnfReHd5l6Inhe vy5ufnODRAiOprwRaE3mV1DHPppSPDbZsfA2Q0iPGFmnDGpkuRrhl7jtLozYrhD1vt 5XBUSLqcKKxTjUNJ8lKgirp tJS deaYpX8qqNr6iCuj6bXVHlR0YHdAMRJ16kxPl6oWRfSeHntgFqerYe0s4ZMe8bjCTKiyCv0c08 56MWMaGR7U OwB3QlpKhDLoG1GS4P j7oNVQm6H3qLK i3o0eNlkaS87n jtzeUnZ508PsnOWZ4dxbwoZovRMnTQJ1FnGTn7MJqdcrgwnd38USCKwf3uTDgpq7qY7rPvONJVrCz7jyJkEMzrr8GlBjINfC9iEApFpX8OKrYlxKJHjcmgiHlPmgMSvtlmkz8HByqmgfFV2FhLW C3Qc9lYLdUdTGB1Ghd3rwyLa7o uDuaMNAi6t0FlYp3qfzzgDo4EmbOnGYdI 4iEmKSb7xRQE4kVxBKPZYItIYXiixeYIoQsR8mAIIjLfKYLlv7lI6sVFzz3ywZAgJWmeVQamxHd1 zKGvO0J0jvOhlmO8T1nR5QAjKHAYsn69FDnbzAnCeRXg2TgAiUelk6sHLvYGiC7yqX xkpqBqb8vYkn6MWlaSjv7sI79H1fb 8vYq9y0oxmf2fSVrqqJdMlDx3u3FLwLb9vc1eHfKH5IeDuKrmzuGLPionqI4cfhMBJXcelKrnsrM6jJ47SffL f5bkM1RSRiIYWbgg6fFw4kKYzcel0 fxxfltijQzLps2uTrfFc5q6w74ebJPh2sfmhy0eDouoexeAtt oddjzTIPxqwEM70VsoWn0HLspMc m tMu80f9pSrAeydH7AIELk U aX6qwZ6og2bhuz0 Yk2E01shk8yVyWhijdVd6jKWVOWMKcATVwiiATScz529CEmDewwPIkPyQm880P TdbQtO JvqJjHNE4bzWPhCm63kE8wyIBZKu1k4oMGtNvD2EfR8BuAe48CybyBpAX450z0k5X0KP0YfbM9SShPQ2tODgkSAjaj8z6DajztVx3aD5c41JmgMPTvk xs9dRd q9Jnv ixI0IIG7SrupTJPDMK82CtCPtKP8fDBS z0GJWBK0fv307yzLI9LYGLmyXgpyXAqfRzGInrifdbyGIDQGfd7yDxSthYqabys5ARGWLCPrz bX4tUA6nCO95N8ZO zm4a X0zdQ2kFLYZ14ClO8DYJ5eiKksPlPpSLmA6023g6L3CepgMtlkvvolp1fwOqHwZcpJwZ26zmWrsAP468gx E8j1OI9ziKGr89PNrzG7J4BCWBuLk9zeLN7X4FJjOFIwpLaGQs56PXWCvM jVNsTLfaXBb1B7gC yzglT7sRQPNkYYvWerwIBlp6wkxOIyHq7l7nXFTq4lXU7mLssMDkf9V6EcMT6BDN289wpAy2Eit8DXMgVfVbFDSu0 q AJ YjgqYyyHhzDFL6S5HUQttdB37RRi jL5fIe7v8mwvXuzea9I1ThqWYPjT4H601m09aOJcNwMXVw6VjnvtlBE8thC dfAtHY2MnAosQLQFrYXgbguv2abhBYKu6En2 kzWL3 h kNTCb4dASoqW5g5KeEuJhlaXQReKPT0cc5uuNGEH3E2qoWxELGWWwPQmiR7bUwxq v0cCrfTaCzoG9L89UNWcQiGOts77IvTW9pEg7Bg6kkSyDslLTp OyNMWm653PWEulb2OOWmllEEoo2KDVm2To8s2l1j6An2Q VB0Qq1QLPeHPaBq2ROcEfF4eQMNRVZgNoRud7s8u2pjfYMJTzgy0yXbli4FnI70njszW0nvvllv 6XhDaKBBdC6AexP1NrlfgoIerMiyzkGI4u8CKZnwMtDYUDUaFPl1uUcfmv2SMbkPcgJI hdJntvfYkgp OLhcZTXv iibgwlh 0O3HkqPj6lT 5fBNkzNe1lY5ugmsqN g3pN4NgVZWxQy7x9aTFT TDg 1R2z4V2 B hO9x qTFJ93z4QWF 3FO2V5HA 6hsA1cd cqPELIDg RWwytNH 6Dg v7mCR7VZWfk2 Tk6l8e wvVX0c3i7j GaX0WgpclebYivAk wP9a Ekz544lFRWorMb6ezpzqRUyQzNCMw13WWQEGqy4z ucDLMc9FPpOeWi1gK5eopCLPMj4iqOxCSVpI3GTQIjlf0zWe 71d regBro24GfXGq0Z40B 5qxjRZWPlEtVMgtYkdGJT5z75N9qUwCHWKmc8MubFdf fB f QOxWP4MBjEShseWchE02 vTZNuXt36K8mSRJrXjXd1zOnKP7IojEseX5ijA Op498F7HeHpXM7NTJB0l1qTuFyouyYbhrrGG0mr71Ypb5zBZERRwWU2eN8529zpJjhsQvrwmB s34ZjSO6JUp0KQBCFLTSQH51l0eTzHAxPiHwGwp l2zyoOwVENeFSQTFpYp3UVojos7 QTF oh8rOyximK efvIOH7U d7fzeg 0p1jp1tn jjDASN2uTQ4amV2VOyenr6NA1GZIs6UN0HJANUhyykCC8GxLwy6cCXtRiFKTLblRqsfWcVPPbDZUScQVDBofmqx1fsK3tjJipqTwDSRWYEu844xjJWqZ4NNaEUK5gZ7vHtDfHhl5Pcfy8VYsZSKMWp35yFqqZoW95s1Gnc4m1AW5qUoZ1QSHKjpQmdPRsPHja3qOe7WzjVwyaYF4EmjjeqnOdT4sROLBM5xrBQClS0LbqM0R21S0ROegg8 GPZuiGxa5RG U8igzWosZJfJEz0pQzRALWiv3NqNouJAjWiCv3fr0K6cD CvUTUcVY4wg9A1yFvMqgdzsaNZHdjKwQMBk irn0RriRs7 t7 0QTF7IQ2JMcjsmC8Ejmw8 tkoET1TlEl0yc3rVggeVl2 txue1E ZuIuVCK9LJ4VCe8vgSE x3PSrDmq3wC8Mg1lEZM7yuV5ERdOcfVoMS22Bl96fVozcHn2Ub8UAg7I22xXBsVJ8U IJJFQZWNUy7 hypWVHYP4f4OrYL 4T38bkXs2ek5B TFYZ0UwcBX QNTi8FKlBFUZPvWq3b2nCknY9i1o35Gs G31 ZJ3z4FFSCK7HFKPbw Hv46BOTSbMsbrB Nl3tQuUpybOegKseNXQvPMCjEhLs8AxaDhIzbwMX J4dux5JdDLvQAOTOrzBRMML1Umgge5EZbE4VPArFSPTSHBKRnNU92gi3Ya1TzqooaadiFouZ p8DuRYJmBnmAkoTnB7PWg8Q2O59dCMSTHnvH3FpFTrHw2iTrTcnrt0DDQ6JQDbut0ZbCLqDRN5GQv5yrufyZoSFAwZDjjaeCC XGX9Y103OfWakUPNhWh0xz eZyvUjShz YB2cjLxOgQAJfqQ3ux F6AQL XNXAlZ66AxYOU1wWUPkOwqkuvhoYrBYTk1YPk1eh9uQv6rPsJbI5c1nBqtr0BAyIjUr7G96hg61xaZ9 OrFVIPn0CfkBIwEQa 59FE1IFKvSnQCcRT1X5dedaqu4PWJvWxxxPClRdmiBIyclRx2CDZYYPpOsLvR0Nm0knF9Y6nGk2uPAB4Bfo0QXZDYQCx 8FGpSiCKAuVU2LmRblz0SeMGFa2G2Pz1GZrpimhnh6jK4C0KykACl2edz8ZFQAzp CyiiyYkVuRk4afwRNSMHXvb4pLvnYDMi76HQ5F 59n6xWYUI20v1rxXOgUjaqUbcZaHkdCaTcW2zWE29b9Szm0wIIHyfOaJg3PE372hAhUF8c61bkTw2BHiEZAB50Se0JO5Zh8FXpsQbMksboh03KMzyuomQkeS9VFKSJED4KI6gqhPrZQoYEW7Ct1cEINZEVDnzpW6sw2WYPfTb4sM8VMZqVsPwnzcnVZFTVz0iQ6Z2cnR866TAQLxSv4U 5 uR5rZrYHawdagJ2gyvdQfL0f2aOfhCO8q4qHI890QmsN3HeskpC h3NVcVK9EnQAJFFuCjZg990cnHDz PLsfo9mgBBb3qeXn3aVr66mrvOQDtMTRPxyPJSJjeZbrkKlW76B6DY6ivFBl2J5GEunu59Os1gvquIW 4P3 Z ia8CTQDIMWqWAtnIDXYnTbwdYBOxgNtV7 OtRw671xPNtVv9GD6pa7Zw2BufDEk43mvWxsfiPM0UC1rl0Qrdm blplaDqDIEOvRAUD0d8Vz 2MVujhd4CcBTx0O2g6iI5SyKVbM5Ncv6lvLN9 QpAHRG71GdgR30eDooGMLUdX3202sQNh6R6dLjFR6gLXJJQqMaJJdHRLxFBDbj6FYB5BTn3wuMGgk f03XzhlGljYM kdBgSW iFGRyh UpJpIK0oEKUlSRePOmQwUMikDn5BG7PD42R34RfqAcQ3RNdYxKtd47vzvI16H LNrBSWcEYHxAFE7mzB NedLnFYVgMROUxoKirPxhPjeZ2YogdXTH3Qeqo4ef17T7X 2XepocgqydQatb HQA5Lyxw1Ue9me8Clo9sxCXvYLFTClgQ5FJ2Ol5xgIiWQLM 2wWz9Bufits3kbDUfGzp0 wAH3TQRl04Ntw H1ch Xuo3HBxXycXmEuHFRyJQyXgzT9MvcepJ5a7Bom5fRefEvIdXHD7oLXAxJ K3UX7xHYhH56PJis0j0HfryXTpeAGjqv3VEObKDZtEvpMQleHHe3IcRurw0zYTWKVa65a9vPNEUjxCXClminGSkxC5zMCLCXvhaMNpNcudtj24wfAAkuTxdy8LRAuEtP6Ur xRmNlIIisqidPSr8zuyLzwlttoZjMblnzSw0VYvVcLjiG KV0rqg zxh1C5uCrdAHfAgDWLqjS4Mc DQK9DWDTpgh9AuWgouphf91iPRrdd8u32HF0idvhIXqxg4Ipjo4jMbLrw foTRJqqLyHq6psXgekaa001XByuVr5t4iebBmTexarOG3RhMnBk lPIAralvUDXRanIhmsh YcswNIQABrrlOBNWiuGEGi4Q71dYO2aP7VOPR6mrqsUQqBGaeFsJcmpNsYyyR9ImEOhcYaCUMqGDJOIyq 8U3ZKVsrrqAwnHbkzyEwLW0VspaDKZZYwmU Xgsl S3q0v3HUZvgvccVUGpjpZ Sb8K1zFdErv6zde2jP9m4VRsZevOTEVcx5bd66cIS9vDCqQWOvCiPitPkV RUDfBPfeZhaqmGHvEpt8ImYZZPUrEqQPyhS3ddsLnuXehKHPKwTYt9x1dE0iIve80Y cqkN51FXYE1ySMytI8N9FfL0de S QO3UsXV6VQPsTW939SMxlz S3A6EhjSI2Yc x1GHEySH5 84 IrJ4yXVB 29L6lUnyiFqRFG7a0ZSKsFY4DOmGV2VAz9EGyCT yfVVY3ckSMV2VQzkFGhJdet6Fy2sfGx9 AYpBFqtU3aje6X9UsHFKDVRxShx3h91 N63aqSmsYCbJ4aQ3qVF2FF7DPLpexC5o8LxUtFbM7OxI5xfxKj4nBRiWqdn7HDNct1x9cJRYbT2DXz5GcaFCx2xvUVVBW0d 8sV5HPxew xTK ctWiCzZfVSkxA5uOirILPJcxC6PZLdJylj7x uMhflE lU8kdsLMDY5xyWl AFHloFUv6 A5Xcfm27D LlIRXkn16baplQe HQuLvbrebZ3YDN5zuKTLWeVC1fGiHqfuk6U5RgLaAmH OQnbQkHsSgt2kue0LYv6UTLPJwkbfR18OcKkc57kJbVJK6yoRDmNuPplhoshrLYxMGeys3Bzp2v Vrgb0XgSpdZWUQ sr3jXNIyYggAsKeBeBxdYyYZCBK0RIizxfc47bi6ZZokhA2yPN0VIIx8GRpF4J35cHhXSQgReKesnIkz8POV ngQ33PsUbnp1PK5XcdXzuf JJgi9dss01oCEtYKSpL4sFKcbd5IfHEHJOHGlFmkKn6zSQNhosX8G767RA7JRNaKPIFwXPNccDqseFxRxDrJk 7GXj6ofHw5oTeVKRuf7evf3xeDD8KWfvJEkMlfzW5BXktY xKwXt3ko R66ucuxNuCKM8Z8wLRkJTPuCLKpiLceR7GgHdAZlePMGWqKs1ymcNMwnAiZGJkNOerLek FD1IInQXwnAUxxNZtKwZOfNfde95G8PF pmKd32XRLoAksIUh TSdWdYAkU8M3Wk27l2 AVpmHXi0OD0PXwBTr xCxDXgfvrH3zwwqMjvjRwOaAYK9HkfvszbmWMq0SuQSvL20cBvAmDPWwNnDT7MUBvytksQVut0eWhaFXeLhmke BXsI fvEWNmM7tMRjOXG2qcPrJgFde9EoN78327UmenHhgO7DN5S37m375Eo35 99H2UbNc4G J5i4LTk6KqTSCToicvXA6FA1wNfuGNc qRKNZBLetc2tH2dqhlWVuiIEXvbr9USBNbRihQHi c7M08EjqGC UdVtvQWrR3low3wdl7M42Zvncv3Q8TJGScTQw0LHpPY7VB1kCB1IC3SA5sRBPEj7TuMcPvlsS0Q5z2APAN yPifBlFlGNvGDng7fzhNR ghr vPWb8nFTx8bsfBaIs0bko cuus9Kjao4Is1sgpG 5LvAmuZaWZimMZxGbkTg8DHXoEt9R0mPl8SOUleQQKBaPknfbsPmQADeZ0hWykI9uPv3RyoJaNyyQP cfwPrHxwckybkmgbJgFFbkbPlneYrO3gYZ4yVgsMye3xIaDg0heXN4LFof30mgZ4OuJCT86iLT FasEilj Ybzjzu0fdndjNoz3DN0HKKD1rrqKg K1duL85bfumi9i7IGpIEK9WB39OHeu9K54X Jz3FMbaOZuLqiksiThc4RAJBGQJy62aXKCuUjjulVj7 s92CTZUox24MBEEGKSRDzKVH2GeJWDrmGoj6HwUcN49xrUJpf99XLMhb AfrlDTOAl8fap7VwSJGQZdit j4AFfiYlFnp6dY1 CBwM2 Erfk1Pa kFEXpRnx7cDCxPFeschCjH IGKfWQnRZaIt5WV52e MlQOatJZQvVN8vq07Jm6oW 4FGUK XRJuQo1xeucdC3gRV7 sddw5Mf4MJpRZyxwTPKqwMQtIBuFm l9P2ubFwm8JqtEmcvHyZrCRWTfQDrSIDArBmlC opp6YfpNBNhDf5GB6 Q4F7ss719wV8WJFGbMAWc9vllBb7l7X1UtxmFLvIKL9aPiLKmk76VMRRr21IY1FOn WyXZ649vukvc38pf n7gRRt216L9pCTiK8Lm6efQoAHdp4MtU9TVd2IJz94e WlUzbIljBrXpL0MoUn9Sdxlo7h5ARHR1bxB15i0BkUQlc1AGFmHTQTRbMs6V096xn7QLphabtXuPYv5CCj 4qVNOZ1a4loHH3lKyOdDWjr5hRYgWb7jgAp9p8OPZ8yz2v 7v6XOHDQT3cJ04w42 ca0W35ki6INrlJR3jOVMxsBU yVUnPBqgCThiUfBnVIoBRS5jyToraa5nFduhpGyzj28MloHJo Y3yojALb821jxRNCSVjhcD5lWnpxoUPyOO57joEFXHyiP5hko iXrDFqLtcnVV1VysLXEPi9JB UIJZsORB36CJVBjG8Ja2hHCWtWojPaviq84AjiK4z7sx9y YERNDxp5ep3 2Xtmz 7NaunchzjLypE5Ncr3zjIIvY7b6uqibWD1oTxBb kKsdfq1cPocPS8b3CcI5oRqYrlnnQpMsgXhrMSdN4dU6ZI W3PRYEXxXjgRcSorfVvC6tI6f7l3ZJ 0VZrJer7oaDP9kDO326rmzl9zxcFY6qS9f5MNw4loULWfn3CbaUIcV2 IWe3BLJhjYLqy9 d2eOP jBemdit3p3DFgqtLHoGtxr8eA4KDVjj8bsAWJvnMhccoa7Nrif0D5vTpI yhQq1PXESiOcaruweNkeG6Q8f8wk4oOe by8Rhoh7QhrxZdZSbXBqaghXHKWsagzZ0SUoS2JtXjvx8wux8wvx82OIdeWEWxNQZ2WNQHEVx4UAIKJxzKej Fw qNXOPRmCW1V3RcyOCr6vMxYnbXdnl5QWLdtdj G50asRWw6u4A KnOOcb7deGs2cUVtBohY5WOVIX H5zOt9ex1kX FgIGKHlc6v srXqAKIrhWKaz56RSC7dDWxWQhr4GLaNjIzua1UztSFtmjFBC97KTy5TfwUvFWsJvQA6Ifkdmko73TrUOqmTf386yGuVLKroarOMh";

    // ##############################
    // HELPER FUNCTIONS
    // ##############################

    fn create_probability_map(data: HashMap<&'static str, f64>) -> ProbabilityMap {
        data.iter()
            .map(|(&k, &v)| (CompactString::new(k), v))
            .collect()
    }

    fn round_to_two_decimal_places(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    // ##############################
    // LANGUAGE MODELS FOR ENGLISH
    // ##############################

    #[fixture]
    fn unigram_language_model_for_english() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "a" => 0.01_f64.ln(),
            "l" => 0.02_f64.ln(),
            "t" => 0.03_f64.ln(),
            "e" => 0.04_f64.ln(),
            "r" => 0.05_f64.ln(),
            // unknown unigrams
            // "w" => 0.0
        ))
    }

    #[fixture]
    fn bigram_language_model_for_english() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "al" => 0.11_f64.ln(),
            "lt" => 0.12_f64.ln(),
            "te" => 0.13_f64.ln(),
            "er" => 0.14_f64.ln(),
            // unknown bigrams
            // "aq" => 0.0,
            // "wx" => 0.0
        ))
    }

    #[fixture]
    fn trigram_language_model_for_english() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "alt" => 0.19_f64.ln(),
            "lte" => 0.2_f64.ln(),
            "ter" => 0.21_f64.ln(),
            // unknown trigrams
            // "aqu" => 0.0,
            // "tez" => 0.0,
            // "wxy" => 0.0
        ))
    }

    #[fixture]
    fn quadrigram_language_model_for_english() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "alte" => 0.25_f64.ln(),
            "lter" => 0.26_f64.ln(),
            // unknown quadrigrams
            // "aqua" => 0.0,
            // "wxyz" => 0.0
        ))
    }

    #[fixture]
    fn fivegram_language_model_for_english() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "alter" => 0.29_f64.ln(),
            // unknown fivegrams
            // "aquas" => 0.0
        ))
    }

    // ##############################
    // LANGUAGE MODELS FOR GERMAN
    // ##############################

    #[fixture]
    fn unigram_language_model_for_german() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "a" => 0.06_f64.ln(),
            "l" => 0.07_f64.ln(),
            "t" => 0.08_f64.ln(),
            "e" => 0.09_f64.ln(),
            "r" => 0.1_f64.ln(),
            // unknown unigrams
            // "w" => 0.0
        ))
    }

    #[fixture]
    fn bigram_language_model_for_german() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "al" => 0.15_f64.ln(),
            "lt" => 0.16_f64.ln(),
            "te" => 0.17_f64.ln(),
            "er" => 0.18_f64.ln(),
            // unknown bigrams
            // "wx" => 0.0
        ))
    }

    #[fixture]
    fn trigram_language_model_for_german() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "alt" => 0.22_f64.ln(),
            "lte" => 0.23_f64.ln(),
            "ter" => 0.24_f64.ln(),
            // unknown trigrams
            // "wxy" => 0.0
        ))
    }

    #[fixture]
    fn quadrigram_language_model_for_german() -> ProbabilityMap {
        create_probability_map(hashmap!(
            "alte" => 0.27_f64.ln(),
            "lter" => 0.28_f64.ln(),
            // unknown quadrigrams
            // "wxyz" => 0.0
        ))
    }

    #[fixture]
    fn fivegram_language_model_for_german() -> ProbabilityMap {
        create_probability_map(hashmap!("alter" => 0.3_f64.ln()))
    }

    // ##############################
    // NGRAM MODELS
    // ##############################

    #[fixture]
    fn unigram_language_models(
        unigram_language_model_for_english: ProbabilityMap,
        unigram_language_model_for_german: ProbabilityMap,
    ) -> &'static LanguageModelMap {
        static UNIGRAM_MODELS_FIXTURE: OnceLock<LanguageModelMap> = OnceLock::new();
        UNIGRAM_MODELS_FIXTURE.get_or_init(|| {
            let map = DashMap::new();
            map.insert(English, unigram_language_model_for_english);
            map.insert(German, unigram_language_model_for_german);
            map
        })
    }

    #[fixture]
    fn bigram_language_models(
        bigram_language_model_for_english: ProbabilityMap,
        bigram_language_model_for_german: ProbabilityMap,
    ) -> &'static LanguageModelMap {
        static BIGRAM_MODELS_FIXTURE: OnceLock<LanguageModelMap> = OnceLock::new();
        BIGRAM_MODELS_FIXTURE.get_or_init(|| {
            let map = DashMap::new();
            map.insert(English, bigram_language_model_for_english);
            map.insert(German, bigram_language_model_for_german);
            map
        })
    }

    #[fixture]
    fn trigram_language_models(
        trigram_language_model_for_english: ProbabilityMap,
        trigram_language_model_for_german: ProbabilityMap,
    ) -> &'static LanguageModelMap {
        static TRIGRAM_MODELS_FIXTURE: OnceLock<LanguageModelMap> = OnceLock::new();
        TRIGRAM_MODELS_FIXTURE.get_or_init(|| {
            let map = DashMap::new();
            map.insert(English, trigram_language_model_for_english);
            map.insert(German, trigram_language_model_for_german);
            map
        })
    }

    #[fixture]
    fn quadrigram_language_models(
        quadrigram_language_model_for_english: ProbabilityMap,
        quadrigram_language_model_for_german: ProbabilityMap,
    ) -> &'static LanguageModelMap {
        static QUADRIGRAM_MODELS_FIXTURE: OnceLock<LanguageModelMap> = OnceLock::new();
        QUADRIGRAM_MODELS_FIXTURE.get_or_init(|| {
            let map = DashMap::new();
            map.insert(English, quadrigram_language_model_for_english);
            map.insert(German, quadrigram_language_model_for_german);
            map
        })
    }

    #[fixture]
    fn fivegram_language_models(
        fivegram_language_model_for_english: ProbabilityMap,
        fivegram_language_model_for_german: ProbabilityMap,
    ) -> &'static LanguageModelMap {
        static FIVEGRAM_MODELS_FIXTURE: OnceLock<LanguageModelMap> = OnceLock::new();
        FIVEGRAM_MODELS_FIXTURE.get_or_init(|| {
            let map = DashMap::new();
            map.insert(English, fivegram_language_model_for_english);
            map.insert(German, fivegram_language_model_for_german);
            map
        })
    }

    #[fixture]
    fn unique_most_common_language_models() -> &'static CountModelMap {
        static UNIQUE_MOST_COMMON_MODELS_FIXTURE: OnceLock<CountModelMap> = OnceLock::new();
        UNIQUE_MOST_COMMON_MODELS_FIXTURE.get_or_init(|| DashMap::new())
    }

    // ##############################
    // TEST DATA MODELS
    // ##############################

    #[fixture(strs=vec![])]
    fn ngram_model(strs: Vec<Vec<&'static str>>) -> Vec<Vec<NgramRef<'static>>> {
        strs.iter()
            .map(|ngram_strs| {
                ngram_strs
                    .iter()
                    .map(|&it| NgramRef::new(it))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    // ##############################
    // DETECTORS
    // ##############################

    #[fixture]
    fn detector_for_english_and_german(
        unigram_language_models: &'static LanguageModelMap,
        bigram_language_models: &'static LanguageModelMap,
        trigram_language_models: &'static LanguageModelMap,
        quadrigram_language_models: &'static LanguageModelMap,
        fivegram_language_models: &'static LanguageModelMap,
        unique_most_common_language_models: &'static CountModelMap,
    ) -> LanguageDetector {
        let languages = hashset!(English, German);
        let languages_with_unique_characters = collect_languages_with_unique_characters(&languages);
        let single_language_alphabets = collect_single_language_alphabets(&languages);

        LanguageDetector {
            languages,
            minimum_relative_distance: 0.0,
            is_low_accuracy_mode_enabled: false,
            is_built_from_one_language: false,
            languages_with_unique_characters,
            single_language_alphabets,
            unigram_language_models,
            bigram_language_models,
            trigram_language_models,
            quadrigram_language_models,
            fivegram_language_models,
            unique_unigram_language_models: unique_most_common_language_models,
            unique_bigram_language_models: unique_most_common_language_models,
            unique_trigram_language_models: unique_most_common_language_models,
            unique_quadrigram_language_models: unique_most_common_language_models,
            unique_fivegram_language_models: unique_most_common_language_models,
            most_common_unigram_language_models: unique_most_common_language_models,
            most_common_bigram_language_models: unique_most_common_language_models,
            most_common_trigram_language_models: unique_most_common_language_models,
            most_common_quadrigram_language_models: unique_most_common_language_models,
            most_common_fivegram_language_models: unique_most_common_language_models,
        }
    }

    #[fixture]
    fn detector_for_all_languages() -> LanguageDetector {
        LanguageDetector::from(Language::all(), 0.0, true, false)
    }

    // ##############################
    // TESTS
    // ##############################

    #[rstest(
        text,
        expected_words,
        case("this is a sentence", vec!["this", "is", "a", "sentence"]),
        case("sentence", vec!["sentence"]),
        case(
            "上海大学是一个好大学 this is a sentence",
            vec![
                "上", "海", "大", "学", "是", "一", "个", "好", "大", "学",
                "this", "is", "a", "sentence"
            ]
        ),
        case::latin_alphabet(
            "Weltweit    gibt es ungefähr 6.000 Sprachen.",
            vec!["weltweit", "gibt", "es", "ungefähr", "sprachen"]
        ),
        case::arabic_alphabet(
            "تعمل بمحركات بنزين و كهرباء حسب الطلب.",
            vec!["تعمل", "بمحركات", "بنزين", "و", "كهرباء", "حسب", "الطلب"]
        ),
        case::armenian_alphabet(
            "Ահա թե ինչպիսին է Մարիամ Մելիքյանի մայրիկը.",
            vec!["ահա", "թե", "ինչպիսին", "է", "մարիամ", "մելիքյանի", "մայրիկը"]
        ),
        case::bengali_alphabet(
            "আপনার নির্দেশে লগি-বৈঠা দিয়ে মানুষ হত্যা করা হয়েছিল।",
            vec!["আপনার", "নির্দেশে", "লগি", "বৈঠা", "দিয়ে", "মানুষ", "হত্যা", "করা", "হয়েছিল"]
        ),
        case::cyrillic_alphabet(
            "Розташоване воно на правому боці річки Мерля.",
            vec!["розташоване", "воно", "на", "правому", "боці", "річки", "мерля"]
        ),
        case::devanagari_alphabet(
            "बनू शकलात तर कृतज्ञ बना, कृतघ्न नको.",
            vec!["बनू", "शकलात", "तर", "कृतज्ञ", "बना", "कृतघ्न", "नको"]
        ),
        case::georgian_alphabet(
            "ადმინისტრაციული ცენტრია ქალაქი პინიასი.",
            vec!["ადმინისტრაციული", "ცენტრია", "ქალაქი", "პინიასი"]
        ),
        case::greek_alphabet(
            "Αγαπημένη ομάδων των Κοσταρικανών, ήταν η Γιουβέντους.",
            vec!["αγαπημένη", "ομάδων", "των", "κοσταρικανών", "ήταν", "η", "γιουβέντους"]
        ),
        case::gujarati_alphabet(
            "અમુક પેટ્રોલિયમ તથા પ્રાકૃતિક ગૈસ પણ નિકળે છે.",
            vec!["અમુક", "પેટ્રોલિયમ", "તથા", "પ્રાકૃતિક", "ગૈસ", "પણ", "નિકળે", "છે"]
        ),
        case::gurmukhi_alphabet(
            "ਉਹ ਬੱਸ ਗਏ ਤੇ ਹੀਰੋਸ਼ੀਮਾ ਨਾਗਾਸਾਕੀ ਤੇ ਬੰਬ ਸੁੱਟ ਦਿੱਤੇ।",
            vec!["ਉਹ", "ਬੱਸ", "ਗਏ", "ਤੇ", "ਹੀਰੋਸ਼ੀਮਾ", "ਨਾਗਾਸਾਕੀ", "ਤੇ", "ਬੰਬ", "ਸੁੱਟ", "ਦਿੱਤੇ"]
        ),
        case::han_alphabet(
            "五、同业间良性的普遍推广与各团体的互助。",
            vec!["五", "同", "业", "间", "良", "性", "的", "普", "遍", "推", "广", "与", "各", "团", "体", "的", "互", "助"]
        ),
        case::hangul_alphabet(
            "그러나 아름다움은 또한 아주 가까이 있다.",
            vec!["그러나", "아름다움은", "또한", "아주", "가까이", "있다"]
        ),
        case::hebrew_alphabet(
            "אחיו שלמה שניור עזב את ספרד.",
            vec!["אחיו", "שלמה", "שניור", "עזב", "את", "ספרד"]
        ),
        case::hiragana_and_katakana_alphabet(
            "京橋創生館で1日とめても安い駐車場！",
            vec!["京", "橋", "創", "生", "館", "で", "日", "と", "め", "て", "も", "安", "い", "駐", "車", "場"]
        ),
        case::tamil_alphabet(
            "அது மார்பில் செந்நிறம் பூசியது போல் ஆயிற்று.",
            vec!["அது", "மார்பில்", "செந்நிறம்", "பூசியது", "போல்", "ஆயிற்று"]
        ),
        case::telugu_alphabet(
            "అనుకరణంలో అంతా చెప్పి చివరికి అని అనేదాన్ని వాడతాం.",
            vec!["అనుకరణంలో", "అంతా", "చెప్పి", "చివరికి", "అని", "అనేదాన్ని", "వాడతాం"]
        ),
        case::thai_alphabet(
            "สตูล จัดพิธีมอบบ้านนักเรียน คืนความสุข สู่ลูก สพฐ.",
            vec!["สตูล", "จัดพิธีมอบบ้านนักเรียน", "คืนความสุข", "สู่ลูก", "สพฐ"]
        )
    )]
    fn test_split_text_into_words(text: &str, expected_words: Vec<&str>) {
        assert_eq!(split_text_into_words(text), expected_words);
    }

    #[rstest(
        language,
        ngram,
        expected_probability,
        case(English, "a", Some(0.01_f64.ln())),
        case(English, "lt", Some(0.12_f64.ln())),
        case(English, "ter", Some(0.21_f64.ln())),
        case(English, "alte", Some(0.25_f64.ln())),
        case(English, "alter", Some(0.29_f64.ln())),
        case(German, "t", Some(0.08_f64.ln())),
        case(German, "er", Some(0.18_f64.ln())),
        case(German, "alt", Some(0.22_f64.ln())),
        case(German, "lter", Some(0.28_f64.ln())),
        case(German, "alter", Some(0.3_f64.ln())),
        // unknown ngrams
        case(German, "xyz", None),
        case(English, "ab", None)
    )]
    fn assert_ngram_probability_lookup_works_correctly(
        detector_for_english_and_german: LanguageDetector,
        language: Language,
        ngram: &str,
        expected_probability: Option<f64>,
    ) {
        let ngram_ref = NgramRef::new(ngram);
        let probability =
            detector_for_english_and_german.look_up_ngram_probability(language, &ngram_ref);

        assert_eq!(
            probability, expected_probability,
            "expected probability {:?} for language '{:?}' and ngram '{}', got {:?}",
            expected_probability, language, ngram, probability
        );
    }

    #[rstest(
        ngram_model,
        expected_sum_of_probabilities,
        case(
            ngram_model(vec![vec!["a"], vec!["l"], vec!["t"], vec!["e"], vec!["r"]]),
            0.01_f64.ln() + 0.02_f64.ln() + 0.03_f64.ln() + 0.04_f64.ln() + 0.05_f64.ln()
        ),
        case(
            // back off unknown Trigram("tez") to known Bigram("te")
            ngram_model(vec![vec!["alt", "al", "a"], vec!["lte", "lt", "l"], vec!["tez", "te", "t"]]),
            0.19_f64.ln() + 0.2_f64.ln() + 0.13_f64.ln()
        ),
        case(
            // back off unknown Fivegram("aquas") to known Unigram("a")
            ngram_model(vec![vec!["aquas", "aqua", "aqu", "aq", "a"]]),
            0.01_f64.ln()
        )
    )]
    fn assert_summation_of_ngram_probabilities_works_correctly(
        detector_for_english_and_german: LanguageDetector,
        ngram_model: Vec<Vec<NgramRef>>,
        expected_sum_of_probabilities: f64,
    ) {
        let sum_of_probabilities = detector_for_english_and_german
            .compute_sum_of_ngram_probabilities(English, &ngram_model);

        assert!(
            approx_eq!(
                f64,
                sum_of_probabilities,
                expected_sum_of_probabilities,
                ulps = 1
            ),
            "expected sum {} for language '{:?}' and ngrams {:?}, got {}",
            expected_sum_of_probabilities,
            English,
            ngram_model,
            sum_of_probabilities
        );
    }

    #[rstest(
        ngram_model,
        expected_probabilities,
        case::unigram_model(
            ngram_model(vec![vec!["a"], vec!["l"], vec!["t"], vec!["e"], vec!["r"]]),
            hashmap!(
                English => 0.01_f64.ln() + 0.02_f64.ln() + 0.03_f64.ln() + 0.04_f64.ln() + 0.05_f64.ln(),
                German => 0.06_f64.ln() + 0.07_f64.ln() + 0.08_f64.ln() + 0.09_f64.ln() + 0.1_f64.ln()
            )
        ),
        case::trigram_model(
            ngram_model(vec![vec!["alt", "al", "a"], vec!["lte", "lt", "l"], vec!["ter", "te", "t"], vec!["wxy", "wx", "w"]]),
            hashmap!(
                English => 0.19_f64.ln() + 0.2_f64.ln() + 0.21_f64.ln(),
                German => 0.22_f64.ln() + 0.23_f64.ln() + 0.24_f64.ln()
            )
        ),
        case::quadrigram_model(
            ngram_model(vec![vec!["alte", "alt", "al", "a"], vec!["lter", "lte", "lt", "l"], vec!["wxyz", "wxy", "wx", "w"]]),
            hashmap!(
                English => 0.25_f64.ln() + 0.26_f64.ln(),
                German => 0.27_f64.ln() + 0.28_f64.ln()
            )
        )
    )]
    fn assert_computation_of_language_probabilities_works_correctly(
        detector_for_english_and_german: LanguageDetector,
        ngram_model: Vec<Vec<NgramRef>>,
        expected_probabilities: HashMap<Language, f64>,
    ) {
        let languages = hashset!(English, German);
        let probabilities = detector_for_english_and_german
            .compute_language_probabilities(&ngram_model, &languages);

        for (language, probability) in probabilities {
            let expected_probability = expected_probabilities[&language];

            assert!(
                approx_eq!(f64, probability, expected_probability, ulps = 1),
                "expected probability {} for language '{:?}', got {}",
                expected_probability,
                language,
                probability
            );
        }
    }

    #[rstest(
        text,
        expected_confidence_values,
        case::language_detected_by_rules("groß", vec![(German, 1.0), (English, 0.0)]),
        case::known_ngrams("Alter", vec![(German, 0.81), (English, 0.19)]),
        case::unknown_ngrams("проарплап", vec![(English, 0.0), (German, 0.0)]),
    )]
    fn test_compute_language_confidence_values(
        detector_for_english_and_german: LanguageDetector,
        text: &str,
        expected_confidence_values: Vec<(Language, f64)>,
    ) {
        let confidence_values = detector_for_english_and_german
            .compute_language_confidence_values(text)
            .iter()
            .map(|(language, value)| (language.clone(), round_to_two_decimal_places(*value)))
            .collect::<Vec<(Language, f64)>>();

        assert_eq!(confidence_values, expected_confidence_values);
    }

    #[rstest]
    fn test_compute_language_confidence_values_for_very_large_input_text() {
        let detector = LanguageDetector::from(hashset!(English, German), 0.0, true, false);
        let confidence_values = detector.compute_language_confidence_values(VERY_LARGE_INPUT_TEXT);
        let expected_confidence_values = vec![(German, 1.0), (English, 0.0)];
        assert_eq!(confidence_values, expected_confidence_values);
    }

    #[rstest(
        text,
        language,
        expected_confidence,
        case::german_detected_by_rules("groß", German, 1.0),
        case::english_detected_by_rules("groß", English, 0.0),
        case::german_known_ngrams("Alter", German, 0.81),
        case::english_known_ngrams("Alter", English, 0.19),
        case::german_unknown_ngrams("проарплап", German, 0.0),
        case::english_unknown_ngrams("проарплап", English, 0.0),
        case::unknown_language("groß", French, 0.0)
    )]
    fn test_compute_language_confidence(
        detector_for_english_and_german: LanguageDetector,
        text: &str,
        language: Language,
        expected_confidence: f64,
    ) {
        let confidence =
            detector_for_english_and_german.compute_language_confidence(text, language);

        assert_eq!(round_to_two_decimal_places(confidence), expected_confidence);
    }

    #[rstest(
        word,
        expected_language,
        case("Alter", Some(German)),
        case("проарплап", None)
    )]
    fn test_detect_language(
        detector_for_english_and_german: LanguageDetector,
        word: &str,
        expected_language: Option<Language>,
    ) {
        let detected_language = detector_for_english_and_german.detect_language_of(word);
        assert_eq!(detected_language, expected_language);
    }

    #[rstest]
    fn test_detect_multiple_languages_for_empty_string(
        detector_for_all_languages: LanguageDetector,
    ) {
        assert!(detector_for_all_languages
            .detect_multiple_languages_of("")
            .is_empty());
    }

    #[rstest(
        sentence,
        expected_word_count,
        expected_language,
        case::english_1(
            "I'm really not sure whether multi-language detection is a good idea.",
            11,
            English
        ),
        case::english_2("I'm frightened! 🙈", 3, English),
        case::kazakh("V төзімділік спорт", 3, Kazakh)
    )]
    fn test_detect_multiple_languages_with_one_language(
        detector_for_all_languages: LanguageDetector,
        sentence: &str,
        expected_word_count: usize,
        expected_language: Language,
    ) {
        let results = detector_for_all_languages.detect_multiple_languages_of(sentence);
        assert_eq!(results.len(), 1);

        let result = &results[0];
        let substring = &sentence[result.start_index()..result.end_index()];
        assert_eq!(substring, sentence);
        assert_eq!(result.word_count, expected_word_count);
        assert_eq!(result.language(), expected_language);
    }

    #[rstest(
        sentence,
        expected_first_substring,
        expected_first_word_count,
        expected_first_language,
        expected_second_substring,
        expected_second_word_count,
        expected_second_language,
        case::english_german(
            "  He   turned around and asked: \"Entschuldigen Sie, sprechen Sie Deutsch?\"",
            "  He   turned around and asked: ",
            5,
            English,
            "\"Entschuldigen Sie, sprechen Sie Deutsch?\"",
            5,
            German
        ),
        case::chinese_english(
            "上海大学是一个好大学. It is such a great university.",
            "上海大学是一个好大学. ",
            10,
            Chinese,
            "It is such a great university.",
            6,
            English
        ),
        case::english_russian(
            "English German French - Английский язык",
            "English German French - ",
            4,
            English,
            "Английский язык",
            2,
            Russian
        )
    )]
    fn test_detect_multiple_languages_with_two_languages(
        detector_for_all_languages: LanguageDetector,
        sentence: &str,
        expected_first_substring: &str,
        expected_first_word_count: usize,
        expected_first_language: Language,
        expected_second_substring: &str,
        expected_second_word_count: usize,
        expected_second_language: Language,
    ) {
        let results = detector_for_all_languages.detect_multiple_languages_of(sentence);
        assert_eq!(results.len(), 2);

        let first_result = &results[0];
        let first_substring = &sentence[first_result.start_index()..first_result.end_index()];
        assert_eq!(first_substring, expected_first_substring);
        assert_eq!(first_result.word_count, expected_first_word_count);
        assert_eq!(first_result.language(), expected_first_language);

        let second_result = &results[1];
        let second_substring = &sentence[second_result.start_index()..second_result.end_index()];
        assert_eq!(second_substring, expected_second_substring);
        assert_eq!(second_result.word_count, expected_second_word_count);
        assert_eq!(second_result.language(), expected_second_language);
    }

    #[rstest(
        sentence,
        expected_first_substring,
        expected_first_word_count,
        expected_first_language,
        expected_second_substring,
        expected_second_word_count,
        expected_second_language,
        expected_third_substring,
        expected_third_word_count,
        expected_third_language,
        case::french_german_english(
            "Parlez-vous français? Ich spreche Französisch nur ein bisschen. A little bit is better than nothing.",
            "Parlez-vous français? ",
            2,
            French,
            "Ich spreche Französisch nur ein bisschen. ",
            6,
            German,
            "A little bit is better than nothing.",
            7,
            English
        ),
        case::polish_german_english(
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe der blaue himmel über berlin 中文 the quick brown fox jumps over the lazy dog",
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe ",
            4,
            Polish,
            "der blaue himmel über berlin 中文 ",
            7,
            German,
            "the quick brown fox jumps over the lazy dog",
            9,
            English
        )
    )]
    fn test_detect_multiple_languages_with_three_languages(
        detector_for_all_languages: LanguageDetector,
        sentence: &str,
        expected_first_substring: &str,
        expected_first_word_count: usize,
        expected_first_language: Language,
        expected_second_substring: &str,
        expected_second_word_count: usize,
        expected_second_language: Language,
        expected_third_substring: &str,
        expected_third_word_count: usize,
        expected_third_language: Language,
    ) {
        let results = detector_for_all_languages.detect_multiple_languages_of(sentence);
        assert_eq!(results.len(), 3);

        let first_result = &results[0];
        let first_substring = &sentence[first_result.start_index()..first_result.end_index()];
        assert_eq!(first_substring, expected_first_substring);
        assert_eq!(first_result.word_count, expected_first_word_count);
        assert_eq!(first_result.language(), expected_first_language);

        let second_result = &results[1];
        let second_substring = &sentence[second_result.start_index()..second_result.end_index()];
        assert_eq!(second_substring, expected_second_substring);
        assert_eq!(second_result.word_count, expected_second_word_count);
        assert_eq!(second_result.language(), expected_second_language);

        let third_result = &results[2];
        let third_substring = &sentence[third_result.start_index()..third_result.end_index()];
        assert_eq!(third_substring, expected_third_substring);
        assert_eq!(third_result.word_count, expected_third_word_count);
        assert_eq!(third_result.language(), expected_third_language);
    }

    #[rstest(
        builder_languages,
        text,
        expected_language,
        case(vec![English, Kazakh], "нормаланбайды", Some(Kazakh)),
        case(vec![English, Kazakh], "нормаланбайды I", Some(Kazakh)),
        case(vec![Kazakh, Mongolian], "Балаларды жүзуге үй-рету бассейнінің үй-жайы", Some(Kazakh)),
        case(vec![English, Russian], "III не нормируется I, II", Some(Russian))
    )]
    fn test_specific_language_detection_problems(
        builder_languages: Vec<Language>,
        text: &str,
        expected_language: Option<Language>,
    ) {
        let detector = LanguageDetectorBuilder::from_languages(&builder_languages)
            .with_preloaded_language_models()
            .build();

        let language = detector.detect_language_of(text);
        assert_eq!(language, expected_language);
    }

    #[rstest(
        word,
        expected_language,
        // words with unique characters
        case("məhərrəm", Some(Azerbaijani)),
        case("substituïts", Some(Catalan)),
        case("rozdělit", Some(Czech)),
        case("tvořen", Some(Czech)),
        case("subjektů", Some(Czech)),
        case("nesufiĉecon", Some(Esperanto)),
        case("intermiksiĝis", Some(Esperanto)),
        case("monaĥinoj", Some(Esperanto)),
        case("kreitaĵoj", Some(Esperanto)),
        case("ŝpinante", Some(Esperanto)),
        case("apenaŭ", Some(Esperanto)),
        case("groß", Some(German)),
        case("σχέδια", Some(Greek)),
        case("fekvő", Some(Hungarian)),
        case("meggyűrűzni", Some(Hungarian)),
        case("ヴェダイヤモンド", Some(Japanese)),
        case("әлем", Some(Kazakh)),
        case("шаруашылығы", Some(Kazakh)),
        case("ақын", Some(Kazakh)),
        case("оның", Some(Kazakh)),
        case("шұрайлы", Some(Kazakh)),
        case("teoloģiska", Some(Latvian)),
        case("blaķene", Some(Latvian)),
        case("ceļojumiem", Some(Latvian)),
        case("numuriņu", Some(Latvian)),
        case("mergelės", Some(Lithuanian)),
        case("įrengus", Some(Lithuanian)),
        case("slegiamų", Some(Lithuanian)),
        case("припаѓа", Some(Macedonian)),
        case("ѕидови", Some(Macedonian)),
        case("ќерка", Some(Macedonian)),
        case("џамиите", Some(Macedonian)),
        case("मिळते", Some(Marathi)),
        case("zmieniły", Some(Polish)),
        case("państwowych", Some(Polish)),
        case("mniejszości", Some(Polish)),
        case("groźne", Some(Polish)),
        case("ialomiţa", Some(Romanian)),
        case("наслеђивања", Some(Serbian)),
        case("неисквареношћу", Some(Serbian)),
        case("podĺa", Some(Slovak)),
        case("pohľade", Some(Slovak)),
        case("mŕtvych", Some(Slovak)),
        case("ґрунтовому", Some(Ukrainian)),
        case("пропонує", Some(Ukrainian)),
        case("пристрої", Some(Ukrainian)),
        case("cằm", Some(Vietnamese)),
        case("thần", Some(Vietnamese)),
        case("chẳng", Some(Vietnamese)),
        case("quẩy", Some(Vietnamese)),
        case("sẵn", Some(Vietnamese)),
        case("nhẫn", Some(Vietnamese)),
        case("dắt", Some(Vietnamese)),
        case("chất", Some(Vietnamese)),
        case("đạp", Some(Vietnamese)),
        case("mặn", Some(Vietnamese)),
        case("hậu", Some(Vietnamese)),
        case("hiền", Some(Vietnamese)),
        case("lẻn", Some(Vietnamese)),
        case("biểu", Some(Vietnamese)),
        case("kẽm", Some(Vietnamese)),
        case("diễm", Some(Vietnamese)),
        case("phế", Some(Vietnamese)),
        case("việc", Some(Vietnamese)),
        case("chỉnh", Some(Vietnamese)),
        case("trĩ", Some(Vietnamese)),
        case("ravị", Some(Vietnamese)),
        case("thơ", Some(Vietnamese)),
        case("nguồn", Some(Vietnamese)),
        case("thờ", Some(Vietnamese)),
        case("sỏi", Some(Vietnamese)),
        case("tổng", Some(Vietnamese)),
        case("nhở", Some(Vietnamese)),
        case("mỗi", Some(Vietnamese)),
        case("bỡi", Some(Vietnamese)),
        case("tốt", Some(Vietnamese)),
        case("giới", Some(Vietnamese)),
        case("một", Some(Vietnamese)),
        case("hợp", Some(Vietnamese)),
        case("hưng", Some(Vietnamese)),
        case("từng", Some(Vietnamese)),
        case("của", Some(Vietnamese)),
        case("sử", Some(Vietnamese)),
        case("cũng", Some(Vietnamese)),
        case("những", Some(Vietnamese)),
        case("chức", Some(Vietnamese)),
        case("dụng", Some(Vietnamese)),
        case("thực", Some(Vietnamese)),
        case("kỳ", Some(Vietnamese)),
        case("kỷ", Some(Vietnamese)),
        case("mỹ", Some(Vietnamese)),
        case("mỵ", Some(Vietnamese)),
        case("aṣiwèrè", Some(Yoruba)),
        case("ṣaaju", Some(Yoruba)),
        case("والموضوع", None),
        case("сопротивление", None),
        case("house", None),

        // words with unique alphabet
        case("ունենա", Some(Armenian)),
        case("জানাতে", Some(Bengali)),
        case("გარეუბან", Some(Georgian)),
        case("σταμάτησε", Some(Greek)),
        case("ઉપકરણોની", Some(Gujarati)),
        case("בתחרויות", Some(Hebrew)),
        case("びさ", Some(Japanese)),
        case("대결구도가", Some(Korean)),
        case("ਮੋਟਰਸਾਈਕਲਾਂ", Some(Punjabi)),
        case("துன்பங்களை", Some(Tamil)),
        case("కృష్ణదేవరాయలు", Some(Telugu)),
        case("ในทางหลวงหมายเลข", Some(Thai)),

        // words with both chinese and japanese characters
        case("人参はβ−カロテン含有量が高く栄養豊富", Some(Japanese)),
    )]
    fn assert_language_detection_with_rules_works_correctly(
        detector_for_all_languages: LanguageDetector,
        word: &str,
        expected_language: Option<Language>,
    ) {
        let detected_language = detector_for_all_languages.detect_language_with_rules(
            &split_text_into_words(word),
            &detector_for_all_languages.languages,
        );
        assert_eq!(
            detected_language, expected_language,
            "expected {:?} for word '{}', got {:?}",
            expected_language, word, detected_language
        );
    }

    #[rstest(word, expected_languages,
        case("والموضوع", hashset!(Arabic, Persian, Urdu)),
        case(
            "сопротивление",
            hashset!(
                Belarusian, Bulgarian, Kazakh, Macedonian, Mongolian, Russian, Serbian, Ukrainian
            )
        ),
        case("раскрывае", hashset!(Belarusian, Kazakh, Mongolian, Russian)),
        case("этот", hashset!(Belarusian, Kazakh, Mongolian, Russian)),
        case("огнём", hashset!(Belarusian, Kazakh, Mongolian, Russian)),
        case("плаваща", hashset!(Bulgarian, Kazakh, Mongolian, Russian)),
        case("довършат", hashset!(Bulgarian, Kazakh, Mongolian, Russian)),
        case("павінен", hashset!(Belarusian, Kazakh, Ukrainian)),
        case("үндсэн", hashset!(Belarusian, Kazakh, Mongolian, Russian)),
        case("дөхөж", hashset!(Kazakh, Mongolian)),
        case("затоплување", hashset!(Macedonian, Serbian)),
        case("ректасцензија", hashset!(Macedonian, Serbian)),
        case("набљудувач", hashset!(Macedonian, Serbian)),
        case("aizklātā", hashset!(Latvian, Maori, Yoruba)),
        case("sistēmas", hashset!(Latvian, Maori, Yoruba)),
        case("palīdzi", hashset!(Latvian, Maori, Yoruba)),
        case("nhẹn", hashset!(Vietnamese, Yoruba)),
        case("chọn", hashset!(Vietnamese, Yoruba)),
        case("prihvaćanju", hashset!(Bosnian, Croatian, Polish)),
        case("nađete", hashset!(Bosnian, Croatian, Vietnamese)),
        case("visão", hashset!(Portuguese, Vietnamese)),
        case("wystąpią", hashset!(Lithuanian, Polish)),
        case("budowę", hashset!(Lithuanian, Polish)),
        case("nebūsime", hashset!(Latvian, Lithuanian, Maori, Yoruba)),
        case("afişate", hashset!(Azerbaijani, Romanian, Turkish)),
        case("kradzieżami", hashset!(Polish, Romanian)),
        case("înviat", hashset!(French, Romanian)),
        case("venerdì", hashset!(Italian, Vietnamese, Yoruba)),
        case("años", hashset!(Basque, Spanish)),
        case("rozohňuje", hashset!(Czech, Slovak)),
        case("rtuť", hashset!(Czech, Slovak)),
        case("pregătire", hashset!(Romanian, Vietnamese)),
        case("jeďte", hashset!(Czech, Romanian, Slovak)),
        case("minjaverðir", hashset!(Icelandic, Turkish)),
        case("þagnarskyldu", hashset!(Icelandic, Turkish)),
        case("nebûtu", hashset!(French, Hungarian)),
        case("hashemidëve", hashset!(Afrikaans, Albanian, Dutch, French)),
        case("forêt", hashset!(Afrikaans, French, Portuguese, Vietnamese)),
        case("succèdent", hashset!(French, Italian, Vietnamese, Yoruba)),
        case("où", hashset!(French, Italian, Vietnamese, Yoruba)),
        case("tõeliseks", hashset!(Estonian, Hungarian, Portuguese, Vietnamese)),
        case("viòiem", hashset!(Catalan, Italian, Vietnamese, Yoruba)),
        case("contrôle", hashset!(French, Portuguese, Slovak, Vietnamese)),
        case("direktør", hashset!(Bokmal, Danish, Nynorsk)),
        case("vývoj", hashset!(Czech, Icelandic, Slovak, Turkish, Vietnamese)),
        case("päralt", hashset!(Estonian, Finnish, German, Slovak, Swedish)),
        case("labâk", hashset!(French, Portuguese, Romanian, Turkish, Vietnamese)),
        case("pràctiques", hashset!(Catalan, French, Italian, Portuguese, Vietnamese)),
        case(
            "überrascht",
            hashset!(Azerbaijani, Catalan, Estonian, German, Hungarian, Spanish, Turkish)
        ),
        case("indebærer", hashset!(Bokmal, Danish, Icelandic, Nynorsk)),
        case("måned", hashset!(Bokmal, Danish, Nynorsk, Swedish)),
        case("zaručen", hashset!(Bosnian, Czech, Croatian, Latvian, Lithuanian, Slovak, Slovene)),
        case("zkouškou", hashset!(Bosnian, Czech, Croatian, Latvian, Lithuanian, Slovak, Slovene)),
        case("navržen", hashset!(Bosnian, Czech, Croatian, Latvian, Lithuanian, Slovak, Slovene)),
        case(
            "façonnage",
            hashset!(Albanian, Azerbaijani, Basque, Catalan, French, Portuguese, Turkish)
        ),
        case(
            "höher",
            hashset!(Azerbaijani, Estonian, Finnish, German, Hungarian, Icelandic, Swedish, Turkish)
        ),
        case(
            "catedráticos",
            hashset!(
                Catalan, Czech, Icelandic, Irish, Hungarian, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            )
        ),
        case(
            "política",
            hashset!(
                Catalan, Czech, Icelandic, Irish, Hungarian, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            )
        ),
        case(
            "música",
            hashset!(
                Catalan, Czech, Icelandic, Irish, Hungarian, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            )
        ),
        case(
            "contradicció",
            hashset!(
                Catalan, Hungarian, Icelandic, Irish, Polish, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            )
        ),
        case(
            "només",
            hashset!(
                Catalan, Czech, French, Hungarian, Icelandic, Irish, Italian, Portuguese, Slovak,
                Spanish, Vietnamese, Yoruba
            )
        ),
        case(
            "house",
            hashset!(
                Afrikaans, Albanian, Azerbaijani, Basque, Bokmal, Bosnian, Catalan, Croatian, Czech,
                Danish, Dutch, English, Esperanto, Estonian, Finnish, French, Ganda, German, Hungarian,
                Icelandic, Indonesian, Irish, Italian, Latin, Latvian, Lithuanian, Malay, Maori, Nynorsk,
                Polish, Portuguese, Romanian, Shona, Slovak, Slovene, Somali, Sotho, Spanish, Swahili,
                Swedish, Tagalog, Tsonga, Tswana, Turkish, Vietnamese, Welsh, Xhosa, Yoruba, Zulu
            )
        ),
    )]
    fn assert_language_filtering_with_rules_works_correctly(
        detector_for_all_languages: LanguageDetector,
        word: &str,
        expected_languages: HashSet<Language>,
    ) {
        let filtered_languages = detector_for_all_languages.filter_languages_by_rules(
            &vec![word.to_string()],
            &detector_for_all_languages.languages,
        );
        assert_eq!(
            filtered_languages, expected_languages,
            "expected {:?} for word '{}', got {:?}",
            expected_languages, word, filtered_languages
        );
    }

    #[rstest(invalid_str, case(""), case(" \n  \t;"), case("3<856%)§"))]
    fn assert_strings_without_letters_return_no_language(
        detector_for_all_languages: LanguageDetector,
        invalid_str: &str,
    ) {
        assert_eq!(
            detector_for_all_languages.detect_language_of(invalid_str),
            None
        );
    }

    #[rstest(text, languages,
        case(
            "ام وی با نیکی میناج تیزر داشت؟؟؟؟؟؟ i vote for bts ( _ ) as the _ via ( _ )",
            vec!(English, Urdu)
        ),
        case(
            "Az elmúlt hétvégén 12-re emelkedett az elhunyt koronavírus-fertőzöttek száma Szlovákiában. Mindegyik szociális otthon dolgozóját letesztelik, Matovič szerint az ingázóknak még várniuk kellene a teszteléssel",
            vec!(Hungarian, Slovak)
        )
    )]
    fn assert_language_detection_is_deterministic(text: &str, languages: Vec<Language>) {
        let detector =
            LanguageDetector::from(languages.iter().cloned().collect(), 0.0, true, false);
        let mut detected_languages = hashset!();
        for _ in 0..100 {
            let language = detector.detect_language_of(text);
            detected_languages.insert(language.unwrap());
        }
        assert_eq!(
            detected_languages.len(),
            1,
            "language detector is non-deterministic for languages {:?}",
            languages
        );
    }

    #[rstest]
    fn assert_low_accuracy_mode_returns_no_language_for_unigrams_and_bigrams() {
        let detector = LanguageDetector::from(hashset!(English, German), 0.0, true, true);

        assert_ne!(detector.detect_language_of("bed"), None);
        assert_eq!(detector.detect_language_of("be"), None);
        assert_eq!(detector.detect_language_of("b"), None);
        assert_eq!(detector.detect_language_of(""), None);
    }
}
