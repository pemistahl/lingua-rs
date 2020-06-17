/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
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

use crate::language::Language;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Eq, PartialEq, Hash)]
pub enum Alphabet {
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Han,
    Hangul,
    Hebrew,
    Hiragana,
    Katakana,
    Latin,
    Tamil,
    Telugu,
    Thai,
}

impl Alphabet {
    pub fn matches(&self, text: &str) -> bool {
        lazy_static! {
            static ref ARABIC: Regex = Regex::new("^\\p{Arabic}+$").unwrap();
            static ref ARMENIAN: Regex = Regex::new("^\\p{Armenian}+$").unwrap();
            static ref BENGALI: Regex = Regex::new("^\\p{Bengali}+$").unwrap();
            static ref CYRILLIC: Regex = Regex::new("^\\p{Cyrillic}+$").unwrap();
            static ref DEVANAGARI: Regex = Regex::new("^\\p{Devanagari}+$").unwrap();
            static ref GEORGIAN: Regex = Regex::new("^\\p{Georgian}+$").unwrap();
            static ref GREEK: Regex = Regex::new("^\\p{Greek}+$").unwrap();
            static ref GUJARATI: Regex = Regex::new("^\\p{Gujarati}+$").unwrap();
            static ref GURMUKHI: Regex = Regex::new("^\\p{Gurmukhi}+$").unwrap();
            static ref HAN: Regex = Regex::new("^\\p{Han}+$").unwrap();
            static ref HANGUL: Regex = Regex::new("^\\p{Hangul}+$").unwrap();
            static ref HEBREW: Regex = Regex::new("^\\p{Hebrew}+$").unwrap();
            static ref HIRAGANA: Regex = Regex::new("^\\p{Hiragana}+$").unwrap();
            static ref KATAKANA: Regex = Regex::new("^\\p{Katakana}+$").unwrap();
            static ref LATIN: Regex = Regex::new("^\\p{Latin}+$").unwrap();
            static ref TAMIL: Regex = Regex::new("^\\p{Tamil}+$").unwrap();
            static ref TELUGU: Regex = Regex::new("^\\p{Telugu}+$").unwrap();
            static ref THAI: Regex = Regex::new("^\\p{THAI}+$").unwrap();
        }

        match self {
            Alphabet::Arabic => ARABIC.is_match(text),
            Alphabet::Armenian => ARMENIAN.is_match(text),
            Alphabet::Bengali => BENGALI.is_match(text),
            Alphabet::Cyrillic => CYRILLIC.is_match(text),
            Alphabet::Devanagari => DEVANAGARI.is_match(text),
            Alphabet::Georgian => GEORGIAN.is_match(text),
            Alphabet::Greek => GREEK.is_match(text),
            Alphabet::Gujarati => GUJARATI.is_match(text),
            Alphabet::Gurmukhi => GURMUKHI.is_match(text),
            Alphabet::Han => HAN.is_match(text),
            Alphabet::Hangul => HANGUL.is_match(text),
            Alphabet::Hebrew => HEBREW.is_match(text),
            Alphabet::Hiragana => HIRAGANA.is_match(text),
            Alphabet::Katakana => KATAKANA.is_match(text),
            Alphabet::Latin => LATIN.is_match(text),
            Alphabet::Tamil => TAMIL.is_match(text),
            Alphabet::Telugu => TELUGU.is_match(text),
            Alphabet::Thai => THAI.is_match(text),
        }
    }

    pub fn all_supporting_single_language() -> HashMap<Alphabet, Language> {
        let mut alphabets = HashMap::new();
        for alphabet in Alphabet::iter() {
            let supported_languages = alphabet.supported_languages();
            if supported_languages.len() == 1 {
                alphabets.insert(alphabet, supported_languages[0].clone());
            }
        }
        alphabets
    }

    fn supported_languages(&self) -> Vec<Language> {
        let mut languages = vec![];
        for language in Language::iter() {
            if language.alphabets().contains(&self) {
                languages.push(language);
            }
        }
        languages
    }
}
