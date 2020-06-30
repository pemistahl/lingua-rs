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

//use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

pub mod charclass {
    use super::*;
    use regex::Regex;

    lazy_static! {
        pub static ref JAPANESE_CHARACTER_SET: Regex =
            Regex::new("^[\\p{Hiragana}\\p{Katakana}\\p{Han}]+$").unwrap();
        pub static ref MULTIPLE_WHITESPACE: Regex = Regex::new("\\s+").unwrap();
        pub static ref NO_LETTER: Regex = Regex::new("^[^\\p{L}]+$").unwrap();
        pub static ref NUMBERS: Regex = Regex::new("\\p{N}").unwrap();
        pub static ref PUNCTUATION: Regex = Regex::new("\\p{P}").unwrap();
    }
}

pub mod alphabet {
    use super::*;
    use regex::Regex;

    lazy_static! {
        pub static ref ARABIC: Regex = Regex::new("^\\p{Arabic}+$").unwrap();
        pub static ref ARMENIAN: Regex = Regex::new("^\\p{Armenian}+$").unwrap();
        pub static ref BENGALI: Regex = Regex::new("^\\p{Bengali}+$").unwrap();
        pub static ref CYRILLIC: Regex = Regex::new("^\\p{Cyrillic}+$").unwrap();
        pub static ref DEVANAGARI: Regex = Regex::new("^\\p{Devanagari}+$").unwrap();
        pub static ref GEORGIAN: Regex = Regex::new("^\\p{Georgian}+$").unwrap();
        pub static ref GREEK: Regex = Regex::new("^\\p{Greek}+$").unwrap();
        pub static ref GUJARATI: Regex = Regex::new("^\\p{Gujarati}+$").unwrap();
        pub static ref GURMUKHI: Regex = Regex::new("^\\p{Gurmukhi}+$").unwrap();
        pub static ref HAN: Regex = Regex::new("^\\p{Han}+$").unwrap();
        pub static ref HANGUL: Regex = Regex::new("^\\p{Hangul}+$").unwrap();
        pub static ref HEBREW: Regex = Regex::new("^\\p{Hebrew}+$").unwrap();
        pub static ref HIRAGANA: Regex = Regex::new("^\\p{Hiragana}+$").unwrap();
        pub static ref KATAKANA: Regex = Regex::new("^\\p{Katakana}+$").unwrap();
        pub static ref LATIN: Regex = Regex::new("^\\p{Latin}+$").unwrap();
        pub static ref TAMIL: Regex = Regex::new("^\\p{Tamil}+$").unwrap();
        pub static ref TELUGU: Regex = Regex::new("^\\p{Telugu}+$").unwrap();
        pub static ref THAI: Regex = Regex::new("^\\p{Thai}+$").unwrap();
    }
}

pub mod charmapping {
    use super::*;
    use crate::language::Language;
    use crate::language::Language::*;
    use std::collections::{HashMap, HashSet};

    lazy_static! {
        pub static ref CHARS_TO_LANGUAGES_MAPPING: HashMap<&'static str, HashSet<Language>> = hashmap!(
            "Ãã" => hashset!(Portuguese, Vietnamese),
            "ĄąĘę" => hashset!(Lithuanian, Polish),
            "Żż" => hashset!(Polish, Romanian),
            "Îî" => hashset!(French, Romanian),
            "Ññ" => hashset!(Basque, Spanish),
            "ŇňŤť" => hashset!(Czech, Slovak),
            "Ăă" => hashset!(Romanian, Vietnamese),
            "İıĞğ" => hashset!(Azerbaijani, Turkish),
            "ЈјЉљЊњ" => hashset!(Macedonian, Serbian),
            "ĀāĒēĪī" => hashset!(Latvian, Yoruba),
            "ẸẹỌọ" => hashset!(Vietnamese, Yoruba),

            "Ūū" => hashset!(Latvian, Lithuanian, Yoruba),
            "Şş" => hashset!(Azerbaijani, Romanian, Turkish),
            "Ďď" => hashset!(Czech, Romanian, Slovak),
            "ÐðÞþ" => hashset!(Icelandic, Latvian, Turkish),
            "Ûû" => hashset!(French, Hungarian, Latvian),
            "Ćć" => hashset!(Bosnian, Croatian, Polish),
            "Đđ" => hashset!(Bosnian, Croatian, Vietnamese),
            "Іі" => hashset!(Belarusian, Kazakh, Ukrainian),
            "Ìì" => hashset!(Italian, Vietnamese, Yoruba),

            "ÈèÙù" => hashset!(French, Italian, Vietnamese, Yoruba),
            "Êê" => hashset!(Afrikaans, French, Portuguese, Vietnamese),
            "Õõ" => hashset!(Estonian, Hungarian, Portuguese, Vietnamese),
            "Ôô" => hashset!(French, Portuguese, Slovak, Vietnamese),
            "Øø" => hashset!(Bokmal, Danish, Nynorsk),
            "ЁёЫыЭэ" => hashset!(Belarusian, Kazakh, Mongolian, Russian),
            "ЩщЪъ" => hashset!(Bulgarian, Kazakh, Mongolian, Russian),

            "Òò" => hashset!(Catalan, Italian, Latvian, Vietnamese, Yoruba),
            "Ýý" => hashset!(Czech, Icelandic, Slovak, Turkish, Vietnamese),
            "Ää" => hashset!(Estonian, Finnish, German, Slovak, Swedish),
            "Ââ" => hashset!(Latvian, Portuguese, Romanian, Turkish, Vietnamese),
            "Àà" => hashset!(Catalan, French, Italian, Portuguese, Vietnamese),
            "Ææ" => hashset!(Bokmal, Danish, Icelandic, Nynorsk),
            "Åå" => hashset!(Bokmal, Danish, Nynorsk, Swedish),

            "ЙйЬьЮюЧчЯя" => hashset!(Belarusian, Bulgarian, Kazakh, Mongolian, Russian, Ukrainian),
            "Üü" => hashset!(Azerbaijani, Catalan, Estonian, German, Hungarian, Turkish),

            "ČčŠšŽž" => hashset!(Bosnian, Czech, Croatian, Latvian, Lithuanian, Slovak, Slovene),

            "Çç" => hashset!(
                Albanian, Azerbaijani, Basque, Catalan, French, Latvian, Portuguese, Turkish
            ),
            "Öö" => hashset!(
                Azerbaijani, Estonian, Finnish, German, Hungarian, Icelandic, Swedish, Turkish
            ),

            "Óó" => hashset!(
                Catalan, Hungarian, Icelandic, Irish, Polish, Portuguese, Slovak, Vietnamese, Yoruba
            ),
            "ÁáÍíÚú" => hashset!(
                Catalan, Czech, Icelandic, Irish, Hungarian, Portuguese, Slovak, Vietnamese, Yoruba
            ),

            "Éé" => hashset!(
                Catalan, Czech, French, Hungarian, Icelandic, Irish, Italian, Portuguese, Slovak,
                Vietnamese, Yoruba
            )
        );
    }
}
