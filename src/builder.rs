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

use std::collections::HashSet;

use crate::detector::LanguageDetector;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use crate::language::Language;

pub(crate) const MISSING_LANGUAGE_MESSAGE: &str =
    "LanguageDetector needs at least 2 languages to choose from";

pub(crate) const MINIMUM_RELATIVE_DISTANCE_MESSAGE: &str =
    "Minimum relative distance must lie in between 0.0 and 0.99";

/// This struct configures and creates an instance of [LanguageDetector].
#[derive(Clone)]
pub struct LanguageDetectorBuilder {
    languages: HashSet<Language>,
    minimum_relative_distance: f64,
    is_every_language_model_preloaded: bool,
    is_low_accuracy_mode_enabled: bool,
}

impl LanguageDetectorBuilder {
    /// Creates and returns an instance of `LanguageDetectorBuilder` with all built-in languages.
    pub fn from_all_languages() -> Self {
        Self::from(Language::all())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in spoken languages.
    pub fn from_all_spoken_languages() -> Self {
        Self::from(Language::all_spoken_ones())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Arabic script.
    pub fn from_all_languages_with_arabic_script() -> Self {
        Self::from(Language::all_with_arabic_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Cyrillic script.
    pub fn from_all_languages_with_cyrillic_script() -> Self {
        Self::from(Language::all_with_cyrillic_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Devanagari script.
    pub fn from_all_languages_with_devanagari_script() -> Self {
        Self::from(Language::all_with_devanagari_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Latin script.
    pub fn from_all_languages_with_latin_script() -> Self {
        Self::from(Language::all_with_latin_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages except those specified in `languages`.
    ///
    /// ⚠ Panics if less than two `languages` are used to build the
    /// `LanguageDetector`.
    pub fn from_all_languages_without(languages: &[Language]) -> Self {
        let mut languages_to_load = Language::all();
        languages_to_load.retain(|it| !languages.contains(it));
        if languages_to_load.len() < 2 {
            panic!("{}", MISSING_LANGUAGE_MESSAGE);
        }
        Self::from(languages_to_load)
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the specified `languages`.
    ///
    /// ⚠ Panics if less than two `languages` are specified.
    pub fn from_languages(languages: &[Language]) -> Self {
        if languages.len() < 2 {
            panic!("{}", MISSING_LANGUAGE_MESSAGE);
        }
        Self::from(languages.iter().cloned().collect())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the languages specified by the respective ISO 639-1 codes.
    ///
    /// ⚠ Panics if less than two `iso_codes` are specified.
    pub fn from_iso_codes_639_1(iso_codes: &[IsoCode639_1]) -> Self {
        if iso_codes.len() < 2 {
            panic!("{}", MISSING_LANGUAGE_MESSAGE);
        }
        let languages = iso_codes
            .iter()
            .map(Language::from_iso_code_639_1)
            .collect::<HashSet<_>>();
        Self::from(languages)
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the languages specified by the respective ISO 639-3 codes.
    ///
    /// ⚠ Panics if less than two `iso_codes` are specified.
    pub fn from_iso_codes_639_3(iso_codes: &[IsoCode639_3]) -> Self {
        if iso_codes.len() < 2 {
            panic!("{}", MISSING_LANGUAGE_MESSAGE);
        }
        let languages = iso_codes
            .iter()
            .map(Language::from_iso_code_639_3)
            .collect::<HashSet<_>>();
        Self::from(languages)
    }

    /// Sets the desired value for the minimum relative distance measure.
    ///
    /// By default, *Lingua* returns the most likely language for a given
    /// input text. However, there are certain words that are spelled the
    /// same in more than one language. The word *prologue*, for instance,
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
    /// returned as [`None`] which is the return value for cases
    /// where language detection is not reliably possible.
    ///
    /// ⚠ Panics if `distance` is smaller than 0.0 or greater than 0.99.
    pub fn with_minimum_relative_distance(&mut self, distance: f64) -> &mut Self {
        if !(0.0..=0.99).contains(&distance) {
            panic!("{}", MINIMUM_RELATIVE_DISTANCE_MESSAGE);
        }
        self.minimum_relative_distance = distance;
        self
    }

    /// Configures `LanguageDetectorBuilder` to preload all language models when creating
    /// the instance of [LanguageDetector].
    ///
    /// By default, *Lingua* uses lazy-loading to load only those language models
    /// on demand which are considered relevant by the rule-based filter engine.
    /// For web services, for instance, it is rather beneficial to preload all language
    /// models into memory to avoid unexpected latency while waiting for the
    /// service response. This method allows to switch between these two loading modes.
    pub fn with_preloaded_language_models(&mut self) -> &mut Self {
        self.is_every_language_model_preloaded = true;
        self
    }

    /// Disables the high accuracy mode in order to save memory and increase performance.
    ///
    /// By default, *Lingua's* high detection accuracy comes at the cost of loading large
    /// language models into memory which might not be feasible for systems running low on
    /// resources.
    ///
    /// This method disables the high accuracy mode so that only a small subset of language
    /// models is loaded into memory. The downside of this approach is that detection accuracy
    /// for short texts consisting of less than 120 characters will drop significantly. However,
    /// detection accuracy for texts which are longer than 120 characters will remain mostly
    /// unaffected.
    pub fn with_low_accuracy_mode(&mut self) -> &mut Self {
        self.is_low_accuracy_mode_enabled = true;
        self
    }

    /// Creates and returns the configured instance of [LanguageDetector].
    pub fn build(&mut self) -> LanguageDetector {
        LanguageDetector::from(
            self.languages.clone(),
            self.minimum_relative_distance,
            self.is_every_language_model_preloaded,
            self.is_low_accuracy_mode_enabled,
        )
    }

    fn from(languages: HashSet<Language>) -> Self {
        Self {
            languages,
            minimum_relative_distance: 0.0,
            is_every_language_model_preloaded: false,
            is_low_accuracy_mode_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_detector_can_be_built_from_all_languages() {
        let mut builder = LanguageDetectorBuilder::from_all_languages();
        assert_eq!(builder.languages, Language::all());
        assert_eq!(builder.minimum_relative_distance, 0.0);

        builder.with_minimum_relative_distance(0.2);
        assert_eq!(builder.minimum_relative_distance, 0.2);
    }

    #[test]
    fn assert_detector_can_be_built_from_spoken_languages() {
        let mut builder = LanguageDetectorBuilder::from_all_spoken_languages();
        assert_eq!(builder.languages, Language::all_spoken_ones());
        assert_eq!(builder.minimum_relative_distance, 0.0);

        builder.with_minimum_relative_distance(0.2);
        assert_eq!(builder.minimum_relative_distance, 0.2);
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_arabic_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_arabic_script();
        assert_eq!(builder.languages, Language::all_with_arabic_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_cyrillic_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_cyrillic_script();
        assert_eq!(builder.languages, Language::all_with_cyrillic_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_devanagari_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_devanagari_script();
        assert_eq!(builder.languages, Language::all_with_devanagari_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_latin_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_latin_script();
        assert_eq!(builder.languages, Language::all_with_latin_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_blacklist() {
        let builder = LanguageDetectorBuilder::from_all_languages_without(&[
            Language::Turkish,
            Language::Romanian,
        ]);
        let expected_languages = Language::all()
            .difference(&hashset!(Language::Turkish, Language::Romanian))
            .cloned()
            .collect::<HashSet<Language>>();

        assert_eq!(builder.languages, expected_languages);
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_long_blacklist() {
        let languages = Language::all()
            .difference(&hashset!(Language::German))
            .cloned()
            .collect::<Vec<_>>();

        LanguageDetectorBuilder::from_all_languages_without(&languages);
    }

    #[test]
    fn assert_detector_can_be_built_from_whitelist() {
        let builder =
            LanguageDetectorBuilder::from_languages(&[Language::German, Language::English]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::English)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_short_whitelist() {
        LanguageDetectorBuilder::from_languages(&[Language::German]);
    }

    #[test]
    fn assert_detector_can_be_built_from_iso_639_1_codes() {
        let builder =
            LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::DE, IsoCode639_1::ZU]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::Zulu)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_few_iso_639_1_codes() {
        LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::DE]);
    }

    #[test]
    fn assert_detector_can_be_built_from_iso_639_3_codes() {
        let builder =
            LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::DEU, IsoCode639_3::ZUL]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::Zulu)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_few_iso_639_3_codes() {
        LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::DEU]);
    }

    #[test]
    #[should_panic(expected = "Minimum relative distance must lie in between 0.0 and 0.99")]
    fn assert_detector_cannot_be_built_from_too_small_minimum_relative_distance() {
        LanguageDetectorBuilder::from_all_languages().with_minimum_relative_distance(-2.3);
    }

    #[test]
    #[should_panic(expected = "Minimum relative distance must lie in between 0.0 and 0.99")]
    fn assert_detector_cannot_be_built_from_too_large_minimum_relative_distance() {
        LanguageDetectorBuilder::from_all_languages().with_minimum_relative_distance(1.7);
    }
}
