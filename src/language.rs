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

use crate::alphabet::Alphabet;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Serialize, Deserialize, EnumIter, Eq, PartialEq, Hash)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Marathi,
    Mongolian,
    Nynorsk,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Xhosa,
    Yoruba,
    Zulu,
}

impl Language {
    pub fn all() -> HashSet<Language> {
        Language::iter().collect()
    }

    pub fn all_spoken_ones() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it != &Language::Latin)
            .collect()
    }

    pub fn all_with_arabic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Arabic))
            .collect()
    }

    pub fn all_with_cyrillic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Cyrillic))
            .collect()
    }

    pub fn all_with_devanagari_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Devanagari))
            .collect()
    }

    pub fn all_with_latin_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Latin))
            .collect()
    }

    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        match self {
            Language::Afrikaans => IsoCode639_1::AF,
            Language::Albanian => IsoCode639_1::SQ,
            Language::Arabic => IsoCode639_1::AR,
            Language::Armenian => IsoCode639_1::HY,
            Language::Azerbaijani => IsoCode639_1::AZ,
            Language::Basque => IsoCode639_1::EU,
            Language::Belarusian => IsoCode639_1::BE,
            Language::Bengali => IsoCode639_1::BN,
            Language::Bokmal => IsoCode639_1::NB,
            Language::Bosnian => IsoCode639_1::BS,
            Language::Bulgarian => IsoCode639_1::BG,
            Language::Catalan => IsoCode639_1::CA,
            Language::Chinese => IsoCode639_1::ZH,
            Language::Croatian => IsoCode639_1::HR,
            Language::Czech => IsoCode639_1::CS,
            Language::Danish => IsoCode639_1::DA,
            Language::Dutch => IsoCode639_1::NL,
            Language::English => IsoCode639_1::EN,
            Language::Esperanto => IsoCode639_1::EO,
            Language::Estonian => IsoCode639_1::ET,
            Language::Finnish => IsoCode639_1::FI,
            Language::French => IsoCode639_1::FR,
            Language::Ganda => IsoCode639_1::LG,
            Language::Georgian => IsoCode639_1::KA,
            Language::German => IsoCode639_1::DE,
            Language::Greek => IsoCode639_1::EL,
            Language::Gujarati => IsoCode639_1::GU,
            Language::Hebrew => IsoCode639_1::HE,
            Language::Hindi => IsoCode639_1::HI,
            Language::Hungarian => IsoCode639_1::HU,
            Language::Icelandic => IsoCode639_1::IS,
            Language::Indonesian => IsoCode639_1::ID,
            Language::Irish => IsoCode639_1::GA,
            Language::Italian => IsoCode639_1::IT,
            Language::Japanese => IsoCode639_1::JA,
            Language::Kazakh => IsoCode639_1::KK,
            Language::Korean => IsoCode639_1::KO,
            Language::Latin => IsoCode639_1::LA,
            Language::Latvian => IsoCode639_1::LV,
            Language::Lithuanian => IsoCode639_1::LT,
            Language::Macedonian => IsoCode639_1::MK,
            Language::Malay => IsoCode639_1::MS,
            Language::Marathi => IsoCode639_1::MR,
            Language::Mongolian => IsoCode639_1::MN,
            Language::Nynorsk => IsoCode639_1::NN,
            Language::Persian => IsoCode639_1::FA,
            Language::Polish => IsoCode639_1::PL,
            Language::Portuguese => IsoCode639_1::PT,
            Language::Punjabi => IsoCode639_1::PA,
            Language::Romanian => IsoCode639_1::RO,
            Language::Russian => IsoCode639_1::RU,
            Language::Serbian => IsoCode639_1::SR,
            Language::Shona => IsoCode639_1::SN,
            Language::Slovak => IsoCode639_1::SK,
            Language::Slovene => IsoCode639_1::SL,
            Language::Somali => IsoCode639_1::SO,
            Language::Sotho => IsoCode639_1::ST,
            Language::Spanish => IsoCode639_1::ES,
            Language::Swahili => IsoCode639_1::SW,
            Language::Swedish => IsoCode639_1::SV,
            Language::Tagalog => IsoCode639_1::TL,
            Language::Tamil => IsoCode639_1::TA,
            Language::Telugu => IsoCode639_1::TE,
            Language::Thai => IsoCode639_1::TH,
            Language::Tsonga => IsoCode639_1::TS,
            Language::Tswana => IsoCode639_1::TN,
            Language::Turkish => IsoCode639_1::TR,
            Language::Ukrainian => IsoCode639_1::UK,
            Language::Urdu => IsoCode639_1::UR,
            Language::Vietnamese => IsoCode639_1::VI,
            Language::Welsh => IsoCode639_1::CY,
            Language::Xhosa => IsoCode639_1::XH,
            Language::Yoruba => IsoCode639_1::YO,
            Language::Zulu => IsoCode639_1::ZU,
        }
    }

    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            Language::Afrikaans => IsoCode639_3::AFR,
            Language::Albanian => IsoCode639_3::SQI,
            Language::Arabic => IsoCode639_3::ARA,
            Language::Armenian => IsoCode639_3::HYE,
            Language::Azerbaijani => IsoCode639_3::AZE,
            Language::Basque => IsoCode639_3::EUS,
            Language::Belarusian => IsoCode639_3::BEL,
            Language::Bengali => IsoCode639_3::BEN,
            Language::Bokmal => IsoCode639_3::NOB,
            Language::Bosnian => IsoCode639_3::BOS,
            Language::Bulgarian => IsoCode639_3::BUL,
            Language::Catalan => IsoCode639_3::CAT,
            Language::Chinese => IsoCode639_3::ZHO,
            Language::Croatian => IsoCode639_3::HRV,
            Language::Czech => IsoCode639_3::CES,
            Language::Danish => IsoCode639_3::DAN,
            Language::Dutch => IsoCode639_3::NLD,
            Language::English => IsoCode639_3::ENG,
            Language::Esperanto => IsoCode639_3::EPO,
            Language::Estonian => IsoCode639_3::EST,
            Language::Finnish => IsoCode639_3::FIN,
            Language::French => IsoCode639_3::FRA,
            Language::Ganda => IsoCode639_3::LUG,
            Language::Georgian => IsoCode639_3::KAT,
            Language::German => IsoCode639_3::DEU,
            Language::Greek => IsoCode639_3::ELL,
            Language::Gujarati => IsoCode639_3::GUJ,
            Language::Hebrew => IsoCode639_3::HEB,
            Language::Hindi => IsoCode639_3::HIN,
            Language::Hungarian => IsoCode639_3::HUN,
            Language::Icelandic => IsoCode639_3::ISL,
            Language::Indonesian => IsoCode639_3::IND,
            Language::Irish => IsoCode639_3::GLE,
            Language::Italian => IsoCode639_3::ITA,
            Language::Japanese => IsoCode639_3::JPN,
            Language::Kazakh => IsoCode639_3::KAZ,
            Language::Korean => IsoCode639_3::KOR,
            Language::Latin => IsoCode639_3::LAT,
            Language::Latvian => IsoCode639_3::LAV,
            Language::Lithuanian => IsoCode639_3::LIT,
            Language::Macedonian => IsoCode639_3::MKD,
            Language::Malay => IsoCode639_3::MSA,
            Language::Marathi => IsoCode639_3::MAR,
            Language::Mongolian => IsoCode639_3::MON,
            Language::Nynorsk => IsoCode639_3::NNO,
            Language::Persian => IsoCode639_3::FAS,
            Language::Polish => IsoCode639_3::POL,
            Language::Portuguese => IsoCode639_3::POR,
            Language::Punjabi => IsoCode639_3::PAN,
            Language::Romanian => IsoCode639_3::RON,
            Language::Russian => IsoCode639_3::RUS,
            Language::Serbian => IsoCode639_3::SRP,
            Language::Shona => IsoCode639_3::SNA,
            Language::Slovak => IsoCode639_3::SLK,
            Language::Slovene => IsoCode639_3::SLV,
            Language::Somali => IsoCode639_3::SOM,
            Language::Sotho => IsoCode639_3::SOT,
            Language::Spanish => IsoCode639_3::SPA,
            Language::Swahili => IsoCode639_3::SWA,
            Language::Swedish => IsoCode639_3::SWE,
            Language::Tagalog => IsoCode639_3::TGL,
            Language::Tamil => IsoCode639_3::TAM,
            Language::Telugu => IsoCode639_3::TEL,
            Language::Thai => IsoCode639_3::THA,
            Language::Tsonga => IsoCode639_3::TSO,
            Language::Tswana => IsoCode639_3::TSN,
            Language::Turkish => IsoCode639_3::TUR,
            Language::Ukrainian => IsoCode639_3::UKR,
            Language::Urdu => IsoCode639_3::URD,
            Language::Vietnamese => IsoCode639_3::VIE,
            Language::Welsh => IsoCode639_3::CYM,
            Language::Xhosa => IsoCode639_3::XHO,
            Language::Yoruba => IsoCode639_3::YOR,
            Language::Zulu => IsoCode639_3::ZUL,
        }
    }

    pub(crate) fn alphabets(&self) -> HashSet<Alphabet> {
        match self {
            Language::Afrikaans
            | Language::Albanian
            | Language::Azerbaijani
            | Language::Basque
            | Language::Bokmal
            | Language::Bosnian
            | Language::Catalan
            | Language::Croatian
            | Language::Czech
            | Language::Danish
            | Language::Dutch
            | Language::English
            | Language::Esperanto
            | Language::Estonian
            | Language::Finnish
            | Language::French
            | Language::Ganda
            | Language::German
            | Language::Hungarian
            | Language::Icelandic
            | Language::Indonesian
            | Language::Irish
            | Language::Italian
            | Language::Latin
            | Language::Latvian
            | Language::Lithuanian
            | Language::Malay
            | Language::Nynorsk
            | Language::Polish
            | Language::Portuguese
            | Language::Romanian
            | Language::Shona
            | Language::Slovak
            | Language::Slovene
            | Language::Somali
            | Language::Sotho
            | Language::Spanish
            | Language::Swahili
            | Language::Swedish
            | Language::Tagalog
            | Language::Tsonga
            | Language::Tswana
            | Language::Turkish
            | Language::Vietnamese
            | Language::Welsh
            | Language::Xhosa
            | Language::Yoruba
            | Language::Zulu => hashset!(Alphabet::Latin),

            Language::Belarusian
            | Language::Bulgarian
            | Language::Kazakh
            | Language::Macedonian
            | Language::Mongolian
            | Language::Russian
            | Language::Serbian
            | Language::Ukrainian => hashset!(Alphabet::Cyrillic),

            Language::Arabic | Language::Persian | Language::Urdu => hashset!(Alphabet::Arabic),

            Language::Hindi | Language::Marathi => hashset!(Alphabet::Devanagari),

            Language::Armenian => hashset!(Alphabet::Armenian),
            Language::Bengali => hashset!(Alphabet::Bengali),
            Language::Chinese => hashset!(Alphabet::Han),
            Language::Georgian => hashset!(Alphabet::Georgian),
            Language::Greek => hashset!(Alphabet::Greek),
            Language::Gujarati => hashset!(Alphabet::Gujarati),
            Language::Hebrew => hashset!(Alphabet::Hebrew),
            Language::Japanese => hashset!(Alphabet::Hiragana, Alphabet::Katakana, Alphabet::Han),
            Language::Korean => hashset!(Alphabet::Hangul),
            Language::Punjabi => hashset!(Alphabet::Gurmukhi),
            Language::Tamil => hashset!(Alphabet::Tamil),
            Language::Telugu => hashset!(Alphabet::Telugu),
            Language::Thai => hashset!(Alphabet::Thai),
        }
    }

    pub(crate) fn unique_characters(&self) -> Option<&str> {
        match self {
            Language::Albanian => Some("Ëë"),
            Language::Azerbaijani => Some("Əə"),
            Language::Catalan => Some("Ïï"),
            Language::Czech => Some("ĚěŘřŮů"),
            Language::Esperanto => Some("ĈĉĜĝĤĥĴĵŜŝŬŭ"),
            Language::German => Some("ß"),
            Language::Hungarian => Some("ŐőŰű"),
            Language::Kazakh => Some("ӘәҒғҚқҢңҰұ"),
            Language::Latvian => Some("ĢģĶķĻļŅņ"),
            Language::Lithuanian => Some("ĖėĮįŲų"),
            Language::Macedonian => Some("ЃѓЅѕЌќЏџ"),
            Language::Marathi => Some("ळ"),
            Language::Mongolian => Some("ӨөҮү"),
            Language::Polish => Some("ŁłŃńŚśŹź"),
            Language::Romanian => Some("Țţ"),
            Language::Serbian => Some("ЂђЋћ"),
            Language::Slovak => Some("ĹĺĽľŔŕ"),
            Language::Spanish => Some("¿¡"),
            Language::Ukrainian => Some("ҐґЄєЇї"),
            Language::Vietnamese => Some("ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ"),
            Language::Yoruba => Some("ŌōṢṣ"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::Language::*;

    #[test]
    fn test_language_serializer() {
        let serialized = serde_json::to_string(&Language::English).unwrap();
        assert_eq!(serialized, "\"ENGLISH\"");
    }

    #[test]
    fn test_language_deserializer() {
        let deserialized = serde_json::from_str::<Language>("\"ENGLISH\"").unwrap();
        assert_eq!(deserialized, Language::English);
    }

    #[test]
    fn assert_all_languages_are_available() {
        assert_eq!(
            Language::all(),
            hashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latin,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_all_spoken_languages_are_available() {
        assert_eq!(
            Language::all_spoken_ones(),
            hashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_arabic_script() {
        assert_eq!(
            Language::all_with_arabic_script(),
            hashset!(Arabic, Persian, Urdu)
        );
    }

    #[test]
    fn assert_certain_languages_support_cyrillic_script() {
        assert_eq!(
            Language::all_with_cyrillic_script(),
            hashset!(
                Belarusian, Bulgarian, Kazakh, Macedonian, Mongolian, Russian, Serbian, Ukrainian
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_devanagari_script() {
        assert_eq!(
            Language::all_with_devanagari_script(),
            hashset!(Hindi, Marathi)
        );
    }

    #[test]
    fn assert_certain_languages_support_latin_script() {
        assert_eq!(
            Language::all_with_latin_script(),
            hashset!(
                Afrikaans,
                Albanian,
                Azerbaijani,
                Basque,
                Bokmal,
                Bosnian,
                Catalan,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                German,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Latin,
                Latvian,
                Lithuanian,
                Malay,
                Nynorsk,
                Polish,
                Portuguese,
                Romanian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tsonga,
                Tswana,
                Turkish,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }
}
