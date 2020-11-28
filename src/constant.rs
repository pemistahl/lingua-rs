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
use crate::language::Language::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub(crate) static JAPANESE_CHARACTER_SET: Lazy<Regex> =
    Lazy::new(|| Regex::new("^[\\p{Hiragana}\\p{Katakana}\\p{Han}]+$").unwrap());
pub(crate) static LETTER: Lazy<Regex> = Lazy::new(|| Regex::new("^\\p{L}+$").unwrap());
pub(crate) static MULTIPLE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
pub(crate) static NO_LETTER: Lazy<Regex> = Lazy::new(|| Regex::new("^[^\\p{L}]+$").unwrap());
pub(crate) static NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{N}").unwrap());
pub(crate) static PUNCTUATION: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{P}").unwrap());

pub(crate) static CHARS_TO_LANGUAGES_MAPPING: Lazy<HashMap<&'static str, HashSet<Language>>> =
    Lazy::new(|| {
        hashmap!(
            "Ãã" => hashset!(Portuguese, Vietnamese),
            "ĄąĘę" => hashset!(Lithuanian, Polish),
            "Żż" => hashset!(Polish, Romanian),
            "Îî" => hashset!(French, Romanian),
            "Ññ" => hashset!(Basque, Spanish),
            "ŇňŤť" => hashset!(Czech, Slovak),
            "Ăă" => hashset!(Romanian, Vietnamese),
            "İıĞğ" => hashset!(Azerbaijani, Turkish),
            "ЈјЉљЊњ" => hashset!(Macedonian, Serbian),
            "ĀāĒēĪī" => hashset!(Latvian, Maori, Yoruba),
            "ẸẹỌọ" => hashset!(Vietnamese, Yoruba),

            "Ūū" => hashset!(Latvian, Lithuanian, Maori, Yoruba),
            "Şş" => hashset!(Azerbaijani, Romanian, Turkish),
            "Ďď" => hashset!(Czech, Romanian, Slovak),
            "ÐðÞþ" => hashset!(Icelandic, Latvian, Turkish),
            "Ûû" => hashset!(French, Hungarian, Latvian),
            "Ćć" => hashset!(Bosnian, Croatian, Polish),
            "Đđ" => hashset!(Bosnian, Croatian, Vietnamese),
            "Іі" => hashset!(Belarusian, Kazakh, Ukrainian),
            "Ìì" => hashset!(Italian, Vietnamese, Yoruba),

            "Ëë" => hashset!(Afrikaans, Albanian, Dutch, French),
            "ÈèÙù" => hashset!(French, Italian, Vietnamese, Yoruba),
            "Êê" => hashset!(Afrikaans, French, Portuguese, Vietnamese),
            "Õõ" => hashset!(Estonian, Hungarian, Portuguese, Vietnamese),
            "Ôô" => hashset!(French, Portuguese, Slovak, Vietnamese),
            "Øø" => hashset!(Bokmal, Danish, Nynorsk),
            "ЁёЫыЭэ" => hashset!(Belarusian, Kazakh, Mongolian, Russian),
            "ЩщЪъ" => hashset!(Bulgarian, Kazakh, Mongolian, Russian),

            "Òò" => hashset!(Catalan, Italian, Latvian, Vietnamese, Yoruba),
            "Ōō" => hashset!(Maori, Yoruba),
            "Ýý" => hashset!(Czech, Icelandic, Slovak, Turkish, Vietnamese),
            "Ää" => hashset!(Estonian, Finnish, German, Slovak, Swedish),
            "Ââ" => hashset!(Latvian, Portuguese, Romanian, Turkish, Vietnamese),
            "Àà" => hashset!(Catalan, French, Italian, Portuguese, Vietnamese),
            "Ææ" => hashset!(Bokmal, Danish, Icelandic, Nynorsk),
            "Åå" => hashset!(Bokmal, Danish, Nynorsk, Swedish),

            "Üü" => hashset!(Azerbaijani, Catalan, Estonian, German, Hungarian, Spanish, Turkish),

            "ČčŠšŽž" => hashset!(Bosnian, Czech, Croatian, Latvian, Lithuanian, Slovak, Slovene),

            "Çç" => hashset!(
                Albanian, Azerbaijani, Basque, Catalan, French, Latvian, Portuguese, Turkish
            ),
            "Öö" => hashset!(
                Azerbaijani, Estonian, Finnish, German, Hungarian, Icelandic, Swedish, Turkish
            ),

            "Óó" => hashset!(
                Catalan, Hungarian, Icelandic, Irish, Polish, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            ),
            "ÁáÍíÚú" => hashset!(
                Catalan, Czech, Icelandic, Irish, Hungarian, Portuguese, Slovak, Spanish,
                Vietnamese, Yoruba
            ),

            "Éé" => hashset!(
                Catalan, Czech, French, Hungarian, Icelandic, Irish, Italian, Portuguese, Slovak,
                Spanish, Vietnamese, Yoruba
            )
        )
    });
