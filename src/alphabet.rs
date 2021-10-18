/*
 * Copyright © 2020-today Peter M. Stahl pemistahl@gmail.com
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
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Eq, PartialEq, Hash)]
pub(crate) enum Alphabet {
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
            if language.alphabets().contains(self) {
                languages.push(language);
            }
        }
        languages
    }
}

static ARABIC: Lazy<Regex> = Lazy::new(|| create_regex("Arabic"));
static ARMENIAN: Lazy<Regex> = Lazy::new(|| create_regex("Armenian"));
static BENGALI: Lazy<Regex> = Lazy::new(|| create_regex("Bengali"));
static CYRILLIC: Lazy<Regex> = Lazy::new(|| create_regex("Cyrillic"));
static DEVANAGARI: Lazy<Regex> = Lazy::new(|| create_regex("Devanagari"));
static GEORGIAN: Lazy<Regex> = Lazy::new(|| create_regex("Georgian"));
static GREEK: Lazy<Regex> = Lazy::new(|| create_regex("Greek"));
static GUJARATI: Lazy<Regex> = Lazy::new(|| create_regex("Gujarati"));
static GURMUKHI: Lazy<Regex> = Lazy::new(|| create_regex("Gurmukhi"));
static HAN: Lazy<Regex> = Lazy::new(|| create_regex("Han"));
static HANGUL: Lazy<Regex> = Lazy::new(|| create_regex("Hangul"));
static HEBREW: Lazy<Regex> = Lazy::new(|| create_regex("Hebrew"));
static HIRAGANA: Lazy<Regex> = Lazy::new(|| create_regex("Hiragana"));
static KATAKANA: Lazy<Regex> = Lazy::new(|| create_regex("Katakana"));
static LATIN: Lazy<Regex> = Lazy::new(|| create_regex("Latin"));
static TAMIL: Lazy<Regex> = Lazy::new(|| create_regex("Tamil"));
static TELUGU: Lazy<Regex> = Lazy::new(|| create_regex("Telugu"));
static THAI: Lazy<Regex> = Lazy::new(|| create_regex("Thai"));

fn create_regex(char_class: &str) -> Regex {
    Regex::new(&format!("^\\p{{{}}}+$", char_class)).unwrap()
}
