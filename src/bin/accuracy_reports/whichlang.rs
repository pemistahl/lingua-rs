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
use lingua::Language;
use whichlang::{Lang as WhichlangLanguage, detect_language as whichlang_detect_language};

pub(crate) struct WhichlangDetector {
    languages: Vec<Language>,
}

impl WhichlangDetector {
    pub(crate) fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
        }
    }

    fn map_language_to_lingua(&self, language: WhichlangLanguage) -> Option<Language> {
        match language {
            WhichlangLanguage::Ara => Some(Language::Arabic),
            WhichlangLanguage::Cmn => Some(Language::Chinese),
            WhichlangLanguage::Deu => Some(Language::German),
            WhichlangLanguage::Eng => Some(Language::English),
            WhichlangLanguage::Fra => Some(Language::French),
            WhichlangLanguage::Hin => Some(Language::Hindi),
            WhichlangLanguage::Ita => Some(Language::Italian),
            WhichlangLanguage::Jpn => Some(Language::Japanese),
            WhichlangLanguage::Kor => Some(Language::Korean),
            WhichlangLanguage::Nld => Some(Language::Dutch),
            WhichlangLanguage::Por => Some(Language::Portuguese),
            WhichlangLanguage::Rus => Some(Language::Russian),
            WhichlangLanguage::Spa => Some(Language::Spanish),
            WhichlangLanguage::Swe => Some(Language::Swedish),
            WhichlangLanguage::Tur => Some(Language::Turkish),
            WhichlangLanguage::Vie => Some(Language::Vietnamese),
        }
    }
}

impl LanguageDetection for WhichlangDetector {
    fn detector_name(&self) -> String {
        "whichlang".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        texts
            .iter()
            .map(|text| self.map_language_to_lingua(whichlang_detect_language(text)))
            .collect()
    }
}
