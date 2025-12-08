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

use std::collections::HashMap;
use std::sync::LazyLock;

use ahash::AHashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::language::Language;

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
        self.char_set().is_match(text)
    }

    pub fn matches_char(&self, ch: char) -> bool {
        self.char_set().is_char_match(ch)
    }

    pub fn all_supporting_single_language() -> HashMap<Alphabet, Language> {
        let mut alphabets = HashMap::new();
        for alphabet in Alphabet::iter() {
            let supported_languages = alphabet.supported_languages();
            if supported_languages.len() == 1 {
                alphabets.insert(alphabet, supported_languages[0]);
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

    fn char_set(&self) -> &LazyLock<CharSet> {
        match self {
            Alphabet::Arabic => &ARABIC,
            Alphabet::Armenian => &ARMENIAN,
            Alphabet::Bengali => &BENGALI,
            Alphabet::Cyrillic => &CYRILLIC,
            Alphabet::Devanagari => &DEVANAGARI,
            Alphabet::Georgian => &GEORGIAN,
            Alphabet::Greek => &GREEK,
            Alphabet::Gujarati => &GUJARATI,
            Alphabet::Gurmukhi => &GURMUKHI,
            Alphabet::Han => &HAN,
            Alphabet::Hangul => &HANGUL,
            Alphabet::Hebrew => &HEBREW,
            Alphabet::Hiragana => &HIRAGANA,
            Alphabet::Katakana => &KATAKANA,
            Alphabet::Latin => &LATIN,
            Alphabet::Tamil => &TAMIL,
            Alphabet::Telugu => &TELUGU,
            Alphabet::Thai => &THAI,
        }
    }
}

pub(crate) struct CharSet {
    characters: AHashSet<char>,
}

impl CharSet {
    pub fn from_char_classes(char_classes: &[&str]) -> Self {
        let mut characters = AHashSet::new();

        for char_class in char_classes {
            let table = crate::script::BY_NAME
                .iter()
                .find(|(name, _)| *name == *char_class)
                .unwrap()
                .1;

            for &(start, end) in table {
                for codepoint in start..=end {
                    characters.insert(codepoint);
                }
            }
        }

        CharSet { characters }
    }

    pub fn from_char_class(char_class: &str) -> Self {
        Self::from_char_classes(&[char_class])
    }

    pub fn is_match(&self, text: &str) -> bool {
        text.chars().all(|ch| self.is_char_match(ch))
    }

    pub fn is_char_match(&self, ch: char) -> bool {
        self.characters.contains(&ch)
    }
}

static ARABIC: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Arabic"));
static ARMENIAN: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Armenian"));
static BENGALI: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Bengali"));
static CYRILLIC: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Cyrillic"));
static DEVANAGARI: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Devanagari"));
static GEORGIAN: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Georgian"));
static GREEK: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Greek"));
static GUJARATI: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Gujarati"));
static GURMUKHI: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Gurmukhi"));
static HAN: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Han"));
static HANGUL: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Hangul"));
static HEBREW: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Hebrew"));
static HIRAGANA: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Hiragana"));
static KATAKANA: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Katakana"));
static LATIN: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Latin"));
static TAMIL: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Tamil"));
static TELUGU: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Telugu"));
static THAI: LazyLock<CharSet> = LazyLock::new(|| CharSet::from_char_class("Thai"));

#[cfg(test)]
mod tests {
    use crate::alphabet::Alphabet;
    use crate::language::Language;

    #[test]
    fn test_alphabets_supporting_single_language() {
        assert_eq!(
            Alphabet::all_supporting_single_language(),
            hashmap!(
                Alphabet::Armenian => Language::Armenian,
                Alphabet::Bengali => Language::Bengali,
                Alphabet::Georgian => Language::Georgian,
                Alphabet::Greek => Language::Greek,
                Alphabet::Gujarati => Language::Gujarati,
                Alphabet::Gurmukhi => Language::Punjabi,
                Alphabet::Hangul => Language::Korean,
                Alphabet::Hebrew => Language::Hebrew,
                Alphabet::Hiragana => Language::Japanese,
                Alphabet::Katakana => Language::Japanese,
                Alphabet::Tamil => Language::Tamil,
                Alphabet::Telugu => Language::Telugu,
                Alphabet::Thai => Language::Thai
            )
        );
    }
}
