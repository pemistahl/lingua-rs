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
use whatlang::{Detector, Lang as WhatlangLanguage};

pub(crate) struct WhatlangDetector {
    languages: Vec<Language>,
    detector: Detector,
}

impl WhatlangDetector {
    pub(crate) fn new(languages: &[Language]) -> Self {
        Self {
            languages: languages.to_vec(),
            detector: Detector::new(),
        }
    }

    fn map_language_to_lingua(&self, language: Option<WhatlangLanguage>) -> Option<Language> {
        match language {
            Some(WhatlangLanguage::Afr) => Some(Language::Afrikaans),
            Some(WhatlangLanguage::Ara) => Some(Language::Arabic),
            Some(WhatlangLanguage::Aze) => Some(Language::Azerbaijani),
            Some(WhatlangLanguage::Bel) => Some(Language::Belarusian),
            Some(WhatlangLanguage::Ben) => Some(Language::Bengali),
            Some(WhatlangLanguage::Bul) => Some(Language::Bulgarian),
            Some(WhatlangLanguage::Cat) => Some(Language::Catalan),
            Some(WhatlangLanguage::Ces) => Some(Language::Czech),
            Some(WhatlangLanguage::Cmn) => Some(Language::Chinese),
            Some(WhatlangLanguage::Dan) => Some(Language::Danish),
            Some(WhatlangLanguage::Deu) => Some(Language::German),
            Some(WhatlangLanguage::Ell) => Some(Language::Greek),
            Some(WhatlangLanguage::Eng) => Some(Language::English),
            Some(WhatlangLanguage::Epo) => Some(Language::Esperanto),
            Some(WhatlangLanguage::Est) => Some(Language::Estonian),
            Some(WhatlangLanguage::Fin) => Some(Language::Finnish),
            Some(WhatlangLanguage::Fra) => Some(Language::French),
            Some(WhatlangLanguage::Guj) => Some(Language::Gujarati),
            Some(WhatlangLanguage::Heb) => Some(Language::Hebrew),
            Some(WhatlangLanguage::Hin) => Some(Language::Hindi),
            Some(WhatlangLanguage::Hrv) => Some(Language::Croatian),
            Some(WhatlangLanguage::Hun) => Some(Language::Hungarian),
            Some(WhatlangLanguage::Ind) => Some(Language::Indonesian),
            Some(WhatlangLanguage::Ita) => Some(Language::Italian),
            Some(WhatlangLanguage::Jpn) => Some(Language::Japanese),
            Some(WhatlangLanguage::Kat) => Some(Language::Georgian),
            Some(WhatlangLanguage::Kor) => Some(Language::Korean),
            Some(WhatlangLanguage::Lat) => Some(Language::Latin),
            Some(WhatlangLanguage::Lav) => Some(Language::Latvian),
            Some(WhatlangLanguage::Lit) => Some(Language::Lithuanian),
            Some(WhatlangLanguage::Mar) => Some(Language::Marathi),
            Some(WhatlangLanguage::Mkd) => Some(Language::Macedonian),
            Some(WhatlangLanguage::Nld) => Some(Language::Dutch),
            Some(WhatlangLanguage::Nob) => Some(Language::Bokmal),
            Some(WhatlangLanguage::Pan) => Some(Language::Punjabi),
            Some(WhatlangLanguage::Pes) => Some(Language::Persian),
            Some(WhatlangLanguage::Pol) => Some(Language::Polish),
            Some(WhatlangLanguage::Por) => Some(Language::Portuguese),
            Some(WhatlangLanguage::Ron) => Some(Language::Romanian),
            Some(WhatlangLanguage::Rus) => Some(Language::Russian),
            Some(WhatlangLanguage::Slk) => Some(Language::Slovak),
            Some(WhatlangLanguage::Slv) => Some(Language::Slovene),
            Some(WhatlangLanguage::Sna) => Some(Language::Shona),
            Some(WhatlangLanguage::Spa) => Some(Language::Spanish),
            Some(WhatlangLanguage::Srp) => Some(Language::Serbian),
            Some(WhatlangLanguage::Swe) => Some(Language::Swedish),
            Some(WhatlangLanguage::Tam) => Some(Language::Tamil),
            Some(WhatlangLanguage::Tel) => Some(Language::Telugu),
            Some(WhatlangLanguage::Tha) => Some(Language::Thai),
            Some(WhatlangLanguage::Tur) => Some(Language::Turkish),
            Some(WhatlangLanguage::Ukr) => Some(Language::Ukrainian),
            Some(WhatlangLanguage::Urd) => Some(Language::Urdu),
            Some(WhatlangLanguage::Vie) => Some(Language::Vietnamese),
            Some(WhatlangLanguage::Zul) => Some(Language::Zulu),
            _ => None,
        }
    }
}

impl LanguageDetection for WhatlangDetector {
    fn detector_name(&self) -> String {
        "whatlang".to_string()
    }

    fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    fn detect(&self, texts: &[&str]) -> Vec<Option<Language>> {
        texts
            .iter()
            .map(|text| self.map_language_to_lingua(self.detector.detect_lang(text)))
            .collect()
    }
}
