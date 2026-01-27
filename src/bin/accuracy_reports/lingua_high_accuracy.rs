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

use crate::language_detection::LanguageDetection;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub(crate) struct LinguaHighAccuracyDetector {
    languages: Vec<Language>,
    detector: LanguageDetector,
}

impl LinguaHighAccuracyDetector {
    pub(crate) fn new(languages: &[Language]) -> Self {
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
