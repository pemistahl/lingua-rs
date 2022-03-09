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

use crate::alphabet::Alphabet;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use std::fmt::{Debug, Display, Formatter, Result};

/// This enum specifies the so far 75 supported languages which can be detected by *Lingua*.
#[derive(
    Clone, Debug, Serialize, Deserialize, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, EnumString,
)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[cfg(feature = "afrikaans")]
    Afrikaans,

    #[cfg(feature = "albanian")]
    Albanian,

    #[cfg(feature = "arabic")]
    Arabic,

    #[cfg(feature = "armenian")]
    Armenian,

    #[cfg(feature = "azerbaijani")]
    Azerbaijani,

    #[cfg(feature = "basque")]
    Basque,

    #[cfg(feature = "belarusian")]
    Belarusian,

    #[cfg(feature = "bengali")]
    Bengali,

    #[cfg(feature = "bokmal")]
    Bokmal,

    #[cfg(feature = "bosnian")]
    Bosnian,

    #[cfg(feature = "bulgarian")]
    Bulgarian,

    #[cfg(feature = "catalan")]
    Catalan,

    #[cfg(feature = "chinese")]
    Chinese,

    #[cfg(feature = "croatian")]
    Croatian,

    #[cfg(feature = "czech")]
    Czech,

    #[cfg(feature = "danish")]
    Danish,

    #[cfg(feature = "dutch")]
    Dutch,

    #[cfg(feature = "english")]
    English,

    #[cfg(feature = "esperanto")]
    Esperanto,

    #[cfg(feature = "estonian")]
    Estonian,

    #[cfg(feature = "finnish")]
    Finnish,

    #[cfg(feature = "french")]
    French,

    #[cfg(feature = "ganda")]
    Ganda,

    #[cfg(feature = "georgian")]
    Georgian,

    #[cfg(feature = "german")]
    German,

    #[cfg(feature = "greek")]
    Greek,

    #[cfg(feature = "gujarati")]
    Gujarati,

    #[cfg(feature = "hebrew")]
    Hebrew,

    #[cfg(feature = "hindi")]
    Hindi,

    #[cfg(feature = "hungarian")]
    Hungarian,

    #[cfg(feature = "icelandic")]
    Icelandic,

    #[cfg(feature = "indonesian")]
    Indonesian,

    #[cfg(feature = "irish")]
    Irish,

    #[cfg(feature = "italian")]
    Italian,

    #[cfg(feature = "japanese")]
    Japanese,

    #[cfg(feature = "kazakh")]
    Kazakh,

    #[cfg(feature = "korean")]
    Korean,

    #[cfg(feature = "latin")]
    Latin,

    #[cfg(feature = "latvian")]
    Latvian,

    #[cfg(feature = "lithuanian")]
    Lithuanian,

    #[cfg(feature = "macedonian")]
    Macedonian,

    #[cfg(feature = "malay")]
    Malay,

    #[cfg(feature = "maori")]
    Maori,

    #[cfg(feature = "marathi")]
    Marathi,

    #[cfg(feature = "mongolian")]
    Mongolian,

    #[cfg(feature = "nynorsk")]
    Nynorsk,

    #[cfg(feature = "persian")]
    Persian,

    #[cfg(feature = "polish")]
    Polish,

    #[cfg(feature = "portuguese")]
    Portuguese,

    #[cfg(feature = "punjabi")]
    Punjabi,

    #[cfg(feature = "romanian")]
    Romanian,

    #[cfg(feature = "russian")]
    Russian,

    #[cfg(feature = "serbian")]
    Serbian,

    #[cfg(feature = "shona")]
    Shona,

    #[cfg(feature = "slovak")]
    Slovak,

    #[cfg(feature = "slovene")]
    Slovene,

    #[cfg(feature = "somali")]
    Somali,

    #[cfg(feature = "sotho")]
    Sotho,

    #[cfg(feature = "spanish")]
    Spanish,

    #[cfg(feature = "swahili")]
    Swahili,

    #[cfg(feature = "swedish")]
    Swedish,

    #[cfg(feature = "tagalog")]
    Tagalog,

    #[cfg(feature = "tamil")]
    Tamil,

    #[cfg(feature = "telugu")]
    Telugu,

    #[cfg(feature = "thai")]
    Thai,

    #[cfg(feature = "tsonga")]
    Tsonga,

    #[cfg(feature = "tswana")]
    Tswana,

    #[cfg(feature = "turkish")]
    Turkish,

    #[cfg(feature = "ukrainian")]
    Ukrainian,

    #[cfg(feature = "urdu")]
    Urdu,

    #[cfg(feature = "vietnamese")]
    Vietnamese,

    #[cfg(feature = "welsh")]
    Welsh,

    #[cfg(feature = "xhosa")]
    Xhosa,

    #[cfg(feature = "yoruba")]
    Yoruba,

    #[cfg(feature = "zulu")]
    Zulu,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{:?}", self);
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Language {
    pub fn all() -> HashSet<Language> {
        Language::iter().collect()
    }

    pub fn all_spoken_ones() -> HashSet<Language> {
        Language::iter()
            .filter(|it| {
                if cfg!(feature = "latin") {
                    it != &Language::from_str("Latin").unwrap()
                } else {
                    true
                }
            })
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
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => IsoCode639_1::AF,

            #[cfg(feature = "albanian")]
            Language::Albanian => IsoCode639_1::SQ,

            #[cfg(feature = "arabic")]
            Language::Arabic => IsoCode639_1::AR,

            #[cfg(feature = "armenian")]
            Language::Armenian => IsoCode639_1::HY,

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => IsoCode639_1::AZ,

            #[cfg(feature = "basque")]
            Language::Basque => IsoCode639_1::EU,

            #[cfg(feature = "belarusian")]
            Language::Belarusian => IsoCode639_1::BE,

            #[cfg(feature = "bengali")]
            Language::Bengali => IsoCode639_1::BN,

            #[cfg(feature = "bokmal")]
            Language::Bokmal => IsoCode639_1::NB,

            #[cfg(feature = "bosnian")]
            Language::Bosnian => IsoCode639_1::BS,

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => IsoCode639_1::BG,

            #[cfg(feature = "catalan")]
            Language::Catalan => IsoCode639_1::CA,

            #[cfg(feature = "chinese")]
            Language::Chinese => IsoCode639_1::ZH,

            #[cfg(feature = "croatian")]
            Language::Croatian => IsoCode639_1::HR,

            #[cfg(feature = "czech")]
            Language::Czech => IsoCode639_1::CS,

            #[cfg(feature = "danish")]
            Language::Danish => IsoCode639_1::DA,

            #[cfg(feature = "dutch")]
            Language::Dutch => IsoCode639_1::NL,

            #[cfg(feature = "english")]
            Language::English => IsoCode639_1::EN,

            #[cfg(feature = "esperanto")]
            Language::Esperanto => IsoCode639_1::EO,

            #[cfg(feature = "estonian")]
            Language::Estonian => IsoCode639_1::ET,

            #[cfg(feature = "finnish")]
            Language::Finnish => IsoCode639_1::FI,

            #[cfg(feature = "french")]
            Language::French => IsoCode639_1::FR,

            #[cfg(feature = "ganda")]
            Language::Ganda => IsoCode639_1::LG,

            #[cfg(feature = "georgian")]
            Language::Georgian => IsoCode639_1::KA,

            #[cfg(feature = "german")]
            Language::German => IsoCode639_1::DE,

            #[cfg(feature = "greek")]
            Language::Greek => IsoCode639_1::EL,

            #[cfg(feature = "gujarati")]
            Language::Gujarati => IsoCode639_1::GU,

            #[cfg(feature = "hebrew")]
            Language::Hebrew => IsoCode639_1::HE,

            #[cfg(feature = "hindi")]
            Language::Hindi => IsoCode639_1::HI,

            #[cfg(feature = "hungarian")]
            Language::Hungarian => IsoCode639_1::HU,

            #[cfg(feature = "icelandic")]
            Language::Icelandic => IsoCode639_1::IS,

            #[cfg(feature = "indonesian")]
            Language::Indonesian => IsoCode639_1::ID,

            #[cfg(feature = "irish")]
            Language::Irish => IsoCode639_1::GA,

            #[cfg(feature = "italian")]
            Language::Italian => IsoCode639_1::IT,

            #[cfg(feature = "japanese")]
            Language::Japanese => IsoCode639_1::JA,

            #[cfg(feature = "kazakh")]
            Language::Kazakh => IsoCode639_1::KK,

            #[cfg(feature = "korean")]
            Language::Korean => IsoCode639_1::KO,

            #[cfg(feature = "latin")]
            Language::Latin => IsoCode639_1::LA,

            #[cfg(feature = "latvian")]
            Language::Latvian => IsoCode639_1::LV,

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => IsoCode639_1::LT,

            #[cfg(feature = "macedonian")]
            Language::Macedonian => IsoCode639_1::MK,

            #[cfg(feature = "malay")]
            Language::Malay => IsoCode639_1::MS,

            #[cfg(feature = "maori")]
            Language::Maori => IsoCode639_1::MI,

            #[cfg(feature = "marathi")]
            Language::Marathi => IsoCode639_1::MR,

            #[cfg(feature = "mongolian")]
            Language::Mongolian => IsoCode639_1::MN,

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => IsoCode639_1::NN,

            #[cfg(feature = "persian")]
            Language::Persian => IsoCode639_1::FA,

            #[cfg(feature = "polish")]
            Language::Polish => IsoCode639_1::PL,

            #[cfg(feature = "portuguese")]
            Language::Portuguese => IsoCode639_1::PT,

            #[cfg(feature = "punjabi")]
            Language::Punjabi => IsoCode639_1::PA,

            #[cfg(feature = "romanian")]
            Language::Romanian => IsoCode639_1::RO,

            #[cfg(feature = "russian")]
            Language::Russian => IsoCode639_1::RU,

            #[cfg(feature = "serbian")]
            Language::Serbian => IsoCode639_1::SR,

            #[cfg(feature = "shona")]
            Language::Shona => IsoCode639_1::SN,

            #[cfg(feature = "slovak")]
            Language::Slovak => IsoCode639_1::SK,

            #[cfg(feature = "slovene")]
            Language::Slovene => IsoCode639_1::SL,

            #[cfg(feature = "somali")]
            Language::Somali => IsoCode639_1::SO,

            #[cfg(feature = "sotho")]
            Language::Sotho => IsoCode639_1::ST,

            #[cfg(feature = "spanish")]
            Language::Spanish => IsoCode639_1::ES,

            #[cfg(feature = "swahili")]
            Language::Swahili => IsoCode639_1::SW,

            #[cfg(feature = "swedish")]
            Language::Swedish => IsoCode639_1::SV,

            #[cfg(feature = "tagalog")]
            Language::Tagalog => IsoCode639_1::TL,

            #[cfg(feature = "tamil")]
            Language::Tamil => IsoCode639_1::TA,

            #[cfg(feature = "telugu")]
            Language::Telugu => IsoCode639_1::TE,

            #[cfg(feature = "thai")]
            Language::Thai => IsoCode639_1::TH,

            #[cfg(feature = "tsonga")]
            Language::Tsonga => IsoCode639_1::TS,

            #[cfg(feature = "tswana")]
            Language::Tswana => IsoCode639_1::TN,

            #[cfg(feature = "turkish")]
            Language::Turkish => IsoCode639_1::TR,

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => IsoCode639_1::UK,

            #[cfg(feature = "urdu")]
            Language::Urdu => IsoCode639_1::UR,

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => IsoCode639_1::VI,

            #[cfg(feature = "welsh")]
            Language::Welsh => IsoCode639_1::CY,

            #[cfg(feature = "xhosa")]
            Language::Xhosa => IsoCode639_1::XH,

            #[cfg(feature = "yoruba")]
            Language::Yoruba => IsoCode639_1::YO,

            #[cfg(feature = "zulu")]
            Language::Zulu => IsoCode639_1::ZU,
        }
    }

    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => IsoCode639_3::AFR,

            #[cfg(feature = "albanian")]
            Language::Albanian => IsoCode639_3::SQI,

            #[cfg(feature = "arabic")]
            Language::Arabic => IsoCode639_3::ARA,

            #[cfg(feature = "armenian")]
            Language::Armenian => IsoCode639_3::HYE,

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => IsoCode639_3::AZE,

            #[cfg(feature = "basque")]
            Language::Basque => IsoCode639_3::EUS,

            #[cfg(feature = "belarusian")]
            Language::Belarusian => IsoCode639_3::BEL,

            #[cfg(feature = "bengali")]
            Language::Bengali => IsoCode639_3::BEN,

            #[cfg(feature = "bokmal")]
            Language::Bokmal => IsoCode639_3::NOB,

            #[cfg(feature = "bosnian")]
            Language::Bosnian => IsoCode639_3::BOS,

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => IsoCode639_3::BUL,

            #[cfg(feature = "catalan")]
            Language::Catalan => IsoCode639_3::CAT,

            #[cfg(feature = "chinese")]
            Language::Chinese => IsoCode639_3::ZHO,

            #[cfg(feature = "croatian")]
            Language::Croatian => IsoCode639_3::HRV,

            #[cfg(feature = "czech")]
            Language::Czech => IsoCode639_3::CES,

            #[cfg(feature = "danish")]
            Language::Danish => IsoCode639_3::DAN,

            #[cfg(feature = "dutch")]
            Language::Dutch => IsoCode639_3::NLD,

            #[cfg(feature = "english")]
            Language::English => IsoCode639_3::ENG,

            #[cfg(feature = "esperanto")]
            Language::Esperanto => IsoCode639_3::EPO,

            #[cfg(feature = "estonian")]
            Language::Estonian => IsoCode639_3::EST,

            #[cfg(feature = "finnish")]
            Language::Finnish => IsoCode639_3::FIN,

            #[cfg(feature = "french")]
            Language::French => IsoCode639_3::FRA,

            #[cfg(feature = "ganda")]
            Language::Ganda => IsoCode639_3::LUG,

            #[cfg(feature = "georgian")]
            Language::Georgian => IsoCode639_3::KAT,

            #[cfg(feature = "german")]
            Language::German => IsoCode639_3::DEU,

            #[cfg(feature = "greek")]
            Language::Greek => IsoCode639_3::ELL,

            #[cfg(feature = "gujarati")]
            Language::Gujarati => IsoCode639_3::GUJ,

            #[cfg(feature = "hebrew")]
            Language::Hebrew => IsoCode639_3::HEB,

            #[cfg(feature = "hindi")]
            Language::Hindi => IsoCode639_3::HIN,

            #[cfg(feature = "hungarian")]
            Language::Hungarian => IsoCode639_3::HUN,

            #[cfg(feature = "icelandic")]
            Language::Icelandic => IsoCode639_3::ISL,

            #[cfg(feature = "indonesian")]
            Language::Indonesian => IsoCode639_3::IND,

            #[cfg(feature = "irish")]
            Language::Irish => IsoCode639_3::GLE,

            #[cfg(feature = "italian")]
            Language::Italian => IsoCode639_3::ITA,

            #[cfg(feature = "japanese")]
            Language::Japanese => IsoCode639_3::JPN,

            #[cfg(feature = "kazakh")]
            Language::Kazakh => IsoCode639_3::KAZ,

            #[cfg(feature = "korean")]
            Language::Korean => IsoCode639_3::KOR,

            #[cfg(feature = "latin")]
            Language::Latin => IsoCode639_3::LAT,

            #[cfg(feature = "latvian")]
            Language::Latvian => IsoCode639_3::LAV,

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => IsoCode639_3::LIT,

            #[cfg(feature = "macedonian")]
            Language::Macedonian => IsoCode639_3::MKD,

            #[cfg(feature = "malay")]
            Language::Malay => IsoCode639_3::MSA,

            #[cfg(feature = "maori")]
            Language::Maori => IsoCode639_3::MRI,

            #[cfg(feature = "marathi")]
            Language::Marathi => IsoCode639_3::MAR,

            #[cfg(feature = "mongolian")]
            Language::Mongolian => IsoCode639_3::MON,

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => IsoCode639_3::NNO,

            #[cfg(feature = "persian")]
            Language::Persian => IsoCode639_3::FAS,

            #[cfg(feature = "polish")]
            Language::Polish => IsoCode639_3::POL,

            #[cfg(feature = "portuguese")]
            Language::Portuguese => IsoCode639_3::POR,

            #[cfg(feature = "punjabi")]
            Language::Punjabi => IsoCode639_3::PAN,

            #[cfg(feature = "romanian")]
            Language::Romanian => IsoCode639_3::RON,

            #[cfg(feature = "russian")]
            Language::Russian => IsoCode639_3::RUS,

            #[cfg(feature = "serbian")]
            Language::Serbian => IsoCode639_3::SRP,

            #[cfg(feature = "shona")]
            Language::Shona => IsoCode639_3::SNA,

            #[cfg(feature = "slovak")]
            Language::Slovak => IsoCode639_3::SLK,

            #[cfg(feature = "slovene")]
            Language::Slovene => IsoCode639_3::SLV,

            #[cfg(feature = "somali")]
            Language::Somali => IsoCode639_3::SOM,

            #[cfg(feature = "sotho")]
            Language::Sotho => IsoCode639_3::SOT,

            #[cfg(feature = "spanish")]
            Language::Spanish => IsoCode639_3::SPA,

            #[cfg(feature = "swahili")]
            Language::Swahili => IsoCode639_3::SWA,

            #[cfg(feature = "swedish")]
            Language::Swedish => IsoCode639_3::SWE,

            #[cfg(feature = "tagalog")]
            Language::Tagalog => IsoCode639_3::TGL,

            #[cfg(feature = "tamil")]
            Language::Tamil => IsoCode639_3::TAM,

            #[cfg(feature = "telugu")]
            Language::Telugu => IsoCode639_3::TEL,

            #[cfg(feature = "thai")]
            Language::Thai => IsoCode639_3::THA,

            #[cfg(feature = "tsonga")]
            Language::Tsonga => IsoCode639_3::TSO,

            #[cfg(feature = "tswana")]
            Language::Tswana => IsoCode639_3::TSN,

            #[cfg(feature = "turkish")]
            Language::Turkish => IsoCode639_3::TUR,

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => IsoCode639_3::UKR,

            #[cfg(feature = "urdu")]
            Language::Urdu => IsoCode639_3::URD,

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => IsoCode639_3::VIE,

            #[cfg(feature = "welsh")]
            Language::Welsh => IsoCode639_3::CYM,

            #[cfg(feature = "xhosa")]
            Language::Xhosa => IsoCode639_3::XHO,

            #[cfg(feature = "yoruba")]
            Language::Yoruba => IsoCode639_3::YOR,

            #[cfg(feature = "zulu")]
            Language::Zulu => IsoCode639_3::ZUL,
        }
    }

    pub(crate) fn alphabets(&self) -> HashSet<Alphabet> {
        match self {
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => hashset!(Alphabet::Latin),

            #[cfg(feature = "albanian")]
            Language::Albanian => hashset!(Alphabet::Latin),

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => hashset!(Alphabet::Latin),

            #[cfg(feature = "basque")]
            Language::Basque => hashset!(Alphabet::Latin),

            #[cfg(feature = "bokmal")]
            Language::Bokmal => hashset!(Alphabet::Latin),

            #[cfg(feature = "bosnian")]
            Language::Bosnian => hashset!(Alphabet::Latin),

            #[cfg(feature = "catalan")]
            Language::Catalan => hashset!(Alphabet::Latin),

            #[cfg(feature = "croatian")]
            Language::Croatian => hashset!(Alphabet::Latin),

            #[cfg(feature = "czech")]
            Language::Czech => hashset!(Alphabet::Latin),

            #[cfg(feature = "danish")]
            Language::Danish => hashset!(Alphabet::Latin),

            #[cfg(feature = "dutch")]
            Language::Dutch => hashset!(Alphabet::Latin),

            #[cfg(feature = "english")]
            Language::English => hashset!(Alphabet::Latin),

            #[cfg(feature = "esperanto")]
            Language::Esperanto => hashset!(Alphabet::Latin),

            #[cfg(feature = "estonian")]
            Language::Estonian => hashset!(Alphabet::Latin),

            #[cfg(feature = "finnish")]
            Language::Finnish => hashset!(Alphabet::Latin),

            #[cfg(feature = "french")]
            Language::French => hashset!(Alphabet::Latin),

            #[cfg(feature = "ganda")]
            Language::Ganda => hashset!(Alphabet::Latin),

            #[cfg(feature = "german")]
            Language::German => hashset!(Alphabet::Latin),

            #[cfg(feature = "hungarian")]
            Language::Hungarian => hashset!(Alphabet::Latin),

            #[cfg(feature = "icelandic")]
            Language::Icelandic => hashset!(Alphabet::Latin),

            #[cfg(feature = "indonesian")]
            Language::Indonesian => hashset!(Alphabet::Latin),

            #[cfg(feature = "irish")]
            Language::Irish => hashset!(Alphabet::Latin),

            #[cfg(feature = "italian")]
            Language::Italian => hashset!(Alphabet::Latin),

            #[cfg(feature = "latin")]
            Language::Latin => hashset!(Alphabet::Latin),

            #[cfg(feature = "latvian")]
            Language::Latvian => hashset!(Alphabet::Latin),

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => hashset!(Alphabet::Latin),

            #[cfg(feature = "malay")]
            Language::Malay => hashset!(Alphabet::Latin),

            #[cfg(feature = "maori")]
            Language::Maori => hashset!(Alphabet::Latin),

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => hashset!(Alphabet::Latin),

            #[cfg(feature = "polish")]
            Language::Polish => hashset!(Alphabet::Latin),

            #[cfg(feature = "portuguese")]
            Language::Portuguese => hashset!(Alphabet::Latin),

            #[cfg(feature = "romanian")]
            Language::Romanian => hashset!(Alphabet::Latin),

            #[cfg(feature = "shona")]
            Language::Shona => hashset!(Alphabet::Latin),

            #[cfg(feature = "slovak")]
            Language::Slovak => hashset!(Alphabet::Latin),

            #[cfg(feature = "slovene")]
            Language::Slovene => hashset!(Alphabet::Latin),

            #[cfg(feature = "somali")]
            Language::Somali => hashset!(Alphabet::Latin),

            #[cfg(feature = "sotho")]
            Language::Sotho => hashset!(Alphabet::Latin),

            #[cfg(feature = "spanish")]
            Language::Spanish => hashset!(Alphabet::Latin),

            #[cfg(feature = "swahili")]
            Language::Swahili => hashset!(Alphabet::Latin),

            #[cfg(feature = "swedish")]
            Language::Swedish => hashset!(Alphabet::Latin),

            #[cfg(feature = "tagalog")]
            Language::Tagalog => hashset!(Alphabet::Latin),

            #[cfg(feature = "tsonga")]
            Language::Tsonga => hashset!(Alphabet::Latin),

            #[cfg(feature = "tswana")]
            Language::Tswana => hashset!(Alphabet::Latin),

            #[cfg(feature = "turkish")]
            Language::Turkish => hashset!(Alphabet::Latin),

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => hashset!(Alphabet::Latin),

            #[cfg(feature = "welsh")]
            Language::Welsh => hashset!(Alphabet::Latin),

            #[cfg(feature = "xhosa")]
            Language::Xhosa => hashset!(Alphabet::Latin),

            #[cfg(feature = "yoruba")]
            Language::Yoruba => hashset!(Alphabet::Latin),

            #[cfg(feature = "zulu")]
            Language::Zulu => hashset!(Alphabet::Latin),

            #[cfg(feature = "belarusian")]
            Language::Belarusian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "kazakh")]
            Language::Kazakh => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "macedonian")]
            Language::Macedonian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "mongolian")]
            Language::Mongolian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "russian")]
            Language::Russian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "serbian")]
            Language::Serbian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => hashset!(Alphabet::Cyrillic),

            #[cfg(feature = "arabic")]
            Language::Arabic => hashset!(Alphabet::Arabic),

            #[cfg(feature = "persian")]
            Language::Persian => hashset!(Alphabet::Arabic),

            #[cfg(feature = "urdu")]
            Language::Urdu => hashset!(Alphabet::Arabic),

            #[cfg(feature = "hindi")]
            Language::Hindi => hashset!(Alphabet::Devanagari),

            #[cfg(feature = "marathi")]
            Language::Marathi => hashset!(Alphabet::Devanagari),

            #[cfg(feature = "armenian")]
            Language::Armenian => hashset!(Alphabet::Armenian),

            #[cfg(feature = "bengali")]
            Language::Bengali => hashset!(Alphabet::Bengali),

            #[cfg(feature = "chinese")]
            Language::Chinese => hashset!(Alphabet::Han),

            #[cfg(feature = "georgian")]
            Language::Georgian => hashset!(Alphabet::Georgian),

            #[cfg(feature = "greek")]
            Language::Greek => hashset!(Alphabet::Greek),

            #[cfg(feature = "gujarati")]
            Language::Gujarati => hashset!(Alphabet::Gujarati),

            #[cfg(feature = "hebrew")]
            Language::Hebrew => hashset!(Alphabet::Hebrew),

            #[cfg(feature = "japanese")]
            Language::Japanese => hashset!(Alphabet::Hiragana, Alphabet::Katakana, Alphabet::Han),

            #[cfg(feature = "korean")]
            Language::Korean => hashset!(Alphabet::Hangul),

            #[cfg(feature = "punjabi")]
            Language::Punjabi => hashset!(Alphabet::Gurmukhi),

            #[cfg(feature = "tamil")]
            Language::Tamil => hashset!(Alphabet::Tamil),

            #[cfg(feature = "telugu")]
            Language::Telugu => hashset!(Alphabet::Telugu),

            #[cfg(feature = "thai")]
            Language::Thai => hashset!(Alphabet::Thai),
        }
    }

    pub(crate) fn unique_characters(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => Some("Əə"),

            #[cfg(feature = "catalan")]
            Language::Catalan => Some("Ïï"),

            #[cfg(feature = "czech")]
            Language::Czech => Some("ĚěŘřŮů"),

            #[cfg(feature = "esperanto")]
            Language::Esperanto => Some("ĈĉĜĝĤĥĴĵŜŝŬŭ"),

            #[cfg(feature = "german")]
            Language::German => Some("ß"),

            #[cfg(feature = "hungarian")]
            Language::Hungarian => Some("ŐőŰű"),

            #[cfg(feature = "kazakh")]
            Language::Kazakh => Some("ӘәҒғҚқҢңҰұ"),

            #[cfg(feature = "latvian")]
            Language::Latvian => Some("ĢģĶķĻļŅņ"),

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => Some("ĖėĮįŲų"),

            #[cfg(feature = "macedonian")]
            Language::Macedonian => Some("ЃѓЅѕЌќЏџ"),

            #[cfg(feature = "marathi")]
            Language::Marathi => Some("ळ"),

            #[cfg(feature = "mongolian")]
            Language::Mongolian => Some("ӨөҮү"),

            #[cfg(feature = "polish")]
            Language::Polish => Some("ŁłŃńŚśŹź"),

            #[cfg(feature = "romanian")]
            Language::Romanian => Some("Țţ"),

            #[cfg(feature = "serbian")]
            Language::Serbian => Some("ЂђЋћ"),

            #[cfg(feature = "slovak")]
            Language::Slovak => Some("ĹĺĽľŔŕ"),

            #[cfg(feature = "spanish")]
            Language::Spanish => Some("¿¡"),

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => Some("ҐґЄєЇї"),

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => Some("ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ"),

            #[cfg(feature = "yoruba")]
            Language::Yoruba => Some("Ṣṣ"),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::Language::*;
    use std::str::FromStr;

    #[test]
    fn assert_language_string_representation_is_correct() {
        assert_eq!(Language::English.to_string(), "english");
    }

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
    fn test_from_str() {
        let language = Language::from_str("english").unwrap();
        assert_eq!(language, Language::English);
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
                Maori,
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
                Maori,
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
                Maori,
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
