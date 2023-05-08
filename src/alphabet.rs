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
use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use once_cell::sync::Lazy;
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

    pub fn matches_char(&self, ch: char) -> bool {
        match self {
            Alphabet::Arabic => ARABIC.is_match_char(ch),
            Alphabet::Armenian => ARMENIAN.is_match_char(ch),
            Alphabet::Bengali => BENGALI.is_match_char(ch),
            Alphabet::Cyrillic => CYRILLIC.is_match_char(ch),
            Alphabet::Devanagari => DEVANAGARI.is_match_char(ch),
            Alphabet::Georgian => GEORGIAN.is_match_char(ch),
            Alphabet::Greek => GREEK.is_match_char(ch),
            Alphabet::Gujarati => GUJARATI.is_match_char(ch),
            Alphabet::Gurmukhi => GURMUKHI.is_match_char(ch),
            Alphabet::Han => HAN.is_match_char(ch),
            Alphabet::Hangul => HANGUL.is_match_char(ch),
            Alphabet::Hebrew => HEBREW.is_match_char(ch),
            Alphabet::Hiragana => HIRAGANA.is_match_char(ch),
            Alphabet::Katakana => KATAKANA.is_match_char(ch),
            Alphabet::Latin => LATIN.is_match_char(ch),
            Alphabet::Tamil => TAMIL.is_match_char(ch),
            Alphabet::Telugu => TELUGU.is_match_char(ch),
            Alphabet::Thai => THAI.is_match_char(ch),
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

pub(crate) struct CharSet(HashSet<char>);

impl CharSet {
    pub fn from_classes(char_classes: &[&str]) -> Self {
        let mut set = HashSet::new();
        for char_class in char_classes {
            let table = crate::script::BY_NAME
                .iter()
                .find(|(name, _)| *name == *char_class)
                .unwrap()
                .1;
            for &(start, end) in table {
                for codepoint in start as u32..=end as u32 {
                    let ch = char::from_u32(codepoint).unwrap();
                    set.insert(ch);
                }
            }
        }

        CharSet(set)
    }

    pub fn from_class(char_class: &str) -> Self {
        Self::from_classes(&[char_class])
    }

    pub fn is_match(&self, text: &str) -> bool {
        text.chars().all(|ch| self.0.contains(&ch))
    }

    pub fn is_match_char(&self, ch: char) -> bool {
        self.0.contains(&ch)
    }
}

static ARABIC: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Arabic"));
static ARMENIAN: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Armenian"));
static BENGALI: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Bengali"));
static CYRILLIC: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Cyrillic"));
static DEVANAGARI: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Devanagari"));
static GEORGIAN: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Georgian"));
static GREEK: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Greek"));
static GUJARATI: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Gujarati"));
static GURMUKHI: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Gurmukhi"));
static HAN: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Han"));
static HANGUL: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Hangul"));
static HEBREW: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Hebrew"));
static HIRAGANA: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Hiragana"));
static KATAKANA: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Katakana"));
static LATIN: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Latin"));
static TAMIL: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Tamil"));
static TELUGU: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Telugu"));
static THAI: Lazy<CharSet> = Lazy::new(|| CharSet::from_class("Thai"));
