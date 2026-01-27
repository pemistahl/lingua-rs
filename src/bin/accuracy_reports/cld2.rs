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
use cld2::{Format, Lang as CLD2Language, detect_language as cld2_detect_language};
use lingua::Language;
use strum::IntoEnumIterator;

pub(crate) struct CLD2Detector {
    languages: Vec<Language>,
}

impl CLD2Detector {
    pub(crate) fn new(languages: &[Language]) -> Self {
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
