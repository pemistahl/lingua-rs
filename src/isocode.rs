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

use std::fmt::{Debug, Display, Formatter, Result};

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

/// This enum specifies the ISO 639-1 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, module = "lingua")
)]
pub enum IsoCode639_1 {
    #[cfg(feature = "afrikaans")]
    /// The ISO 639-1 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AF,

    #[cfg(feature = "arabic")]
    /// The ISO 639-1 code for [`Arabic`](crate::language::Language::Arabic)
    AR,

    #[cfg(feature = "azerbaijani")]
    /// The ISO 639-1 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZ,

    #[cfg(feature = "belarusian")]
    /// The ISO 639-1 code for [`Belarusian`](crate::language::Language::Belarusian)
    BE,

    #[cfg(feature = "bulgarian")]
    /// The ISO 639-1 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BG,

    #[cfg(feature = "bengali")]
    /// The ISO 639-1 code for [`Bengali`](crate::language::Language::Bengali)
    BN,

    #[cfg(feature = "bosnian")]
    /// The ISO 639-1 code for [`Bosnian`](crate::language::Language::Bosnian)
    BS,

    #[cfg(feature = "catalan")]
    /// The ISO 639-1 code for [`Catalan`](crate::language::Language::Catalan)
    CA,

    #[cfg(feature = "czech")]
    /// The ISO 639-1 code for [`Czech`](crate::language::Language::Czech)
    CS,

    #[cfg(feature = "welsh")]
    /// The ISO 639-1 code for [`Welsh`](crate::language::Language::Welsh)
    CY,

    #[cfg(feature = "danish")]
    /// The ISO 639-1 code for [`Danish`](crate::language::Language::Danish)
    DA,

    #[cfg(feature = "german")]
    /// The ISO 639-1 code for [`German`](crate::language::Language::German)
    DE,

    #[cfg(feature = "greek")]
    /// The ISO 639-1 code for [`Greek`](crate::language::Language::Greek)
    EL,

    #[cfg(feature = "english")]
    /// The ISO 639-1 code for [`English`](crate::language::Language::English)
    EN,

    #[cfg(feature = "esperanto")]
    /// The ISO 639-1 code for [`Esperanto`](crate::language::Language::Esperanto)
    EO,

    #[cfg(feature = "spanish")]
    /// The ISO 639-1 code for [`Spanish`](crate::language::Language::Spanish)
    ES,

    #[cfg(feature = "estonian")]
    /// The ISO 639-1 code for [`Estonian`](crate::language::Language::Estonian)
    ET,

    #[cfg(feature = "basque")]
    /// The ISO 639-1 code for [`Basque`](crate::language::Language::Basque)
    EU,

    #[cfg(feature = "persian")]
    /// The ISO 639-1 code for [`Persian`](crate::language::Language::Persian)
    FA,

    #[cfg(feature = "finnish")]
    /// The ISO 639-1 code for [`Finnish`](crate::language::Language::Finnish)
    FI,

    #[cfg(feature = "french")]
    /// The ISO 639-1 code for [`French`](crate::language::Language::French)
    FR,

    #[cfg(feature = "irish")]
    /// The ISO 639-1 code for [`Irish`](crate::language::Language::Irish)
    GA,

    #[cfg(feature = "gujarati")]
    /// The ISO 639-1 code for [`Gujarati`](crate::language::Language::Gujarati)
    GU,

    #[cfg(feature = "hebrew")]
    /// The ISO 639-1 code for [`Hebrew`](crate::language::Language::Hebrew)
    HE,

    #[cfg(feature = "hindi")]
    /// The ISO 639-1 code for [`Hindi`](crate::language::Language::Hindi)
    HI,

    #[cfg(feature = "croatian")]
    /// The ISO 639-1 code for [`Croatian`](crate::language::Language::Croatian)
    HR,

    #[cfg(feature = "hungarian")]
    /// The ISO 639-1 code for [`Hungarian`](crate::language::Language::Hungarian)
    HU,

    #[cfg(feature = "armenian")]
    /// The ISO 639-1 code for [`Armenian`](crate::language::Language::Armenian)
    HY,

    #[cfg(feature = "indonesian")]
    /// The ISO 639-1 code for [`Indonesian`](crate::language::Language::Indonesian)
    ID,

    #[cfg(feature = "icelandic")]
    /// The ISO 639-1 code for [`Icelandic`](crate::language::Language::Icelandic)
    IS,

    #[cfg(feature = "italian")]
    /// The ISO 639-1 code for [`Italian`](crate::language::Language::Italian)
    IT,

    #[cfg(feature = "japanese")]
    /// The ISO 639-1 code for [`Japanese`](crate::language::Language::Japanese)
    JA,

    #[cfg(feature = "georgian")]
    /// The ISO 639-1 code for [`Georgian`](crate::language::Language::Georgian)
    KA,

    #[cfg(feature = "kazakh")]
    /// The ISO 639-1 code for [`Kazakh`](crate::language::Language::Kazakh)
    KK,

    #[cfg(feature = "korean")]
    /// The ISO 639-1 code for [`Korean`](crate::language::Language::Korean)
    KO,

    #[cfg(feature = "latin")]
    /// The ISO 639-1 code for [`Latin`](crate::language::Language::Latin)
    LA,

    #[cfg(feature = "ganda")]
    /// The ISO 639-1 code for [`Ganda`](crate::language::Language::Ganda)
    LG,

    #[cfg(feature = "lithuanian")]
    /// The ISO 639-1 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LT,

    #[cfg(feature = "latvian")]
    /// The ISO 639-1 code for [`Latvian`](crate::language::Language::Latvian)
    LV,

    #[cfg(feature = "maori")]
    /// The ISO 639-1 code for [`Maori`](crate::language::Language::Maori)
    MI,

    #[cfg(feature = "macedonian")]
    /// The ISO 639-1 code for [`Macedonian`](crate::language::Language::Macedonian)
    MK,

    #[cfg(feature = "mongolian")]
    /// The ISO 639-1 code for [`Mongolian`](crate::language::Language::Mongolian)
    MN,

    #[cfg(feature = "marathi")]
    /// The ISO 639-1 code for [`Marathi`](crate::language::Language::Marathi)
    MR,

    #[cfg(feature = "malay")]
    /// The ISO 639-1 code for [`Malay`](crate::language::Language::Malay)
    MS,

    #[cfg(feature = "bokmal")]
    /// The ISO 639-1 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NB,

    #[cfg(feature = "dutch")]
    /// The ISO 639-1 code for [`Dutch`](crate::language::Language::Dutch)
    NL,

    #[cfg(feature = "nynorsk")]
    /// The ISO 639-1 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NN,

    #[cfg(feature = "punjabi")]
    /// The ISO 639-1 code for [`Punjabi`](crate::language::Language::Punjabi)
    PA,

    #[cfg(feature = "polish")]
    /// The ISO 639-1 code for [`Polish`](crate::language::Language::Polish)
    PL,

    #[cfg(feature = "portuguese")]
    /// The ISO 639-1 code for [`Portuguese`](crate::language::Language::Portuguese)
    PT,

    #[cfg(feature = "romanian")]
    /// The ISO 639-1 code for [`Romanian`](crate::language::Language::Romanian)
    RO,

    #[cfg(feature = "russian")]
    /// The ISO 639-1 code for [`Russian`](crate::language::Language::Russian)
    RU,

    #[cfg(feature = "slovak")]
    /// The ISO 639-1 code for [`Slovak`](crate::language::Language::Slovak)
    SK,

    #[cfg(feature = "slovene")]
    /// The ISO 639-1 code for [`Slovene`](crate::language::Language::Slovene)
    SL,

    #[cfg(feature = "shona")]
    /// The ISO 639-1 code for [`Shona`](crate::language::Language::Shona)
    SN,

    #[cfg(feature = "somali")]
    /// The ISO 639-1 code for [`Somali`](crate::language::Language::Somali)
    SO,

    #[cfg(feature = "albanian")]
    /// The ISO 639-1 code for [`Albanian`](crate::language::Language::Albanian)
    SQ,

    #[cfg(feature = "serbian")]
    /// The ISO 639-1 code for [`Serbian`](crate::language::Language::Serbian)
    SR,

    #[cfg(feature = "sotho")]
    /// The ISO 639-1 code for [`Sotho`](crate::language::Language::Sotho)
    ST,

    #[cfg(feature = "swedish")]
    /// The ISO 639-1 code for [`Swedish`](crate::language::Language::Swedish)
    SV,

    #[cfg(feature = "swahili")]
    /// The ISO 639-1 code for [`Swahili`](crate::language::Language::Swahili)
    SW,

    #[cfg(feature = "tamil")]
    /// The ISO 639-1 code for [`Tamil`](crate::language::Language::Tamil)
    TA,

    #[cfg(feature = "telugu")]
    /// The ISO 639-1 code for [`Telugu`](crate::language::Language::Telugu)
    TE,

    #[cfg(feature = "thai")]
    /// The ISO 639-1 code for [`Thai`](crate::language::Language::Thai)
    TH,

    #[cfg(feature = "tagalog")]
    /// The ISO 639-1 code for [`Tagalog`](crate::language::Language::Tagalog)
    TL,

    #[cfg(feature = "tswana")]
    /// The ISO 639-1 code for [`Tswana`](crate::language::Language::Tswana)
    TN,

    #[cfg(feature = "turkish")]
    /// The ISO 639-1 code for [`Turkish`](crate::language::Language::Turkish)
    TR,

    #[cfg(feature = "tsonga")]
    /// The ISO 639-1 code for [`Tsonga`](crate::language::Language::Tsonga)
    TS,

    #[cfg(feature = "ukrainian")]
    /// The ISO 639-1 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UK,

    #[cfg(feature = "urdu")]
    /// The ISO 639-1 code for [`Urdu`](crate::language::Language::Urdu)
    UR,

    #[cfg(feature = "vietnamese")]
    /// The ISO 639-1 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VI,

    #[cfg(feature = "xhosa")]
    /// The ISO 639-1 code for [`Xhosa`](crate::language::Language::Xhosa)
    XH,

    #[cfg(feature = "yoruba")]
    /// The ISO 639-1 code for [`Yoruba`](crate::language::Language::Yoruba)
    YO,

    #[cfg(feature = "chinese")]
    /// The ISO 639-1 code for [`Chinese`](crate::language::Language::Chinese)
    ZH,

    #[cfg(feature = "zulu")]
    /// The ISO 639-1 code for [`Zulu`](crate::language::Language::Zulu)
    ZU,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
pub enum IsoCode639_3 {
    #[cfg(feature = "afrikaans")]
    /// The ISO 639-3 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AFR,

    #[cfg(feature = "arabic")]
    /// The ISO 639-3 code for [`Arabic`](crate::language::Language::Arabic)
    ARA,

    #[cfg(feature = "azerbaijani")]
    /// The ISO 639-3 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZE,

    #[cfg(feature = "belarusian")]
    /// The ISO 639-3 code for [`Belarusian`](crate::language::Language::Belarusian)
    BEL,

    #[cfg(feature = "bengali")]
    /// The ISO 639-3 code for [`Bengali`](crate::language::Language::Bengali)
    BEN,

    #[cfg(feature = "bosnian")]
    /// The ISO 639-3 code for [`Bosnian`](crate::language::Language::Bosnian)
    BOS,

    #[cfg(feature = "bulgarian")]
    /// The ISO 639-3 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BUL,

    #[cfg(feature = "catalan")]
    /// The ISO 639-3 code for [`Catalan`](crate::language::Language::Catalan)
    CAT,

    #[cfg(feature = "czech")]
    /// The ISO 639-3 code for [`Czech`](crate::language::Language::Czech)
    CES,

    #[cfg(feature = "welsh")]
    /// The ISO 639-3 code for [`Welsh`](crate::language::Language::Welsh)
    CYM,

    #[cfg(feature = "danish")]
    /// The ISO 639-3 code for [`Danish`](crate::language::Language::Danish)
    DAN,

    #[cfg(feature = "german")]
    /// The ISO 639-3 code for [`German`](crate::language::Language::German)
    DEU,

    #[cfg(feature = "greek")]
    /// The ISO 639-3 code for [`Greek`](crate::language::Language::Greek)
    ELL,

    #[cfg(feature = "english")]
    /// The ISO 639-3 code for [`English`](crate::language::Language::English)
    ENG,

    #[cfg(feature = "esperanto")]
    /// The ISO 639-3 code for [`Esperanto`](crate::language::Language::Esperanto)
    EPO,

    #[cfg(feature = "estonian")]
    /// The ISO 639-3 code for [`Estonian`](crate::language::Language::Estonian)
    EST,

    #[cfg(feature = "basque")]
    /// The ISO 639-3 code for [`Basque`](crate::language::Language::Basque)
    EUS,

    #[cfg(feature = "persian")]
    /// The ISO 639-3 code for [`Persian`](crate::language::Language::Persian)
    FAS,

    #[cfg(feature = "finnish")]
    /// The ISO 639-3 code for [`Finnish`](crate::language::Language::Finnish)
    FIN,

    #[cfg(feature = "french")]
    /// The ISO 639-3 code for [`French`](crate::language::Language::French)
    FRA,

    #[cfg(feature = "irish")]
    /// The ISO 639-3 code for [`Irish`](crate::language::Language::Irish)
    GLE,

    #[cfg(feature = "gujarati")]
    /// The ISO 639-3 code for [`Gujarati`](crate::language::Language::Gujarati)
    GUJ,

    #[cfg(feature = "hebrew")]
    /// The ISO 639-3 code for [`Hebrew`](crate::language::Language::Hebrew)
    HEB,

    #[cfg(feature = "hindi")]
    /// The ISO 639-3 code for [`Hindi`](crate::language::Language::Hindi)
    HIN,

    #[cfg(feature = "croatian")]
    /// The ISO 639-3 code for [`Croatian`](crate::language::Language::Croatian)
    HRV,

    #[cfg(feature = "hungarian")]
    /// The ISO 639-3 code for [`Hungarian`](crate::language::Language::Hungarian)
    HUN,

    #[cfg(feature = "armenian")]
    /// The ISO 639-3 code for [`Armenian`](crate::language::Language::Armenian)
    HYE,

    #[cfg(feature = "indonesian")]
    /// The ISO 639-3 code for [`Indonesian`](crate::language::Language::Indonesian)
    IND,

    #[cfg(feature = "icelandic")]
    /// The ISO 639-3 code for [`Icelandic`](crate::language::Language::Icelandic)
    ISL,

    #[cfg(feature = "italian")]
    /// The ISO 639-3 code for [`Italian`](crate::language::Language::Italian)
    ITA,

    #[cfg(feature = "japanese")]
    /// The ISO 639-3 code for [`Japanese`](crate::language::Language::Japanese)
    JPN,

    #[cfg(feature = "georgian")]
    /// The ISO 639-3 code for [`Georgian`](crate::language::Language::Georgian)
    KAT,

    #[cfg(feature = "kazakh")]
    /// The ISO 639-3 code for [`Kazakh`](crate::language::Language::Kazakh)
    KAZ,

    #[cfg(feature = "korean")]
    /// The ISO 639-3 code for [`Korean`](crate::language::Language::Korean)
    KOR,

    #[cfg(feature = "latin")]
    /// The ISO 639-3 code for [`Latin`](crate::language::Language::Latin)
    LAT,

    #[cfg(feature = "latvian")]
    /// The ISO 639-3 code for [`Latvian`](crate::language::Language::Latvian)
    LAV,

    #[cfg(feature = "lithuanian")]
    /// The ISO 639-3 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LIT,

    #[cfg(feature = "ganda")]
    /// The ISO 639-3 code for [`Ganda`](crate::language::Language::Ganda)
    LUG,

    #[cfg(feature = "marathi")]
    /// The ISO 639-3 code for [`Marathi`](crate::language::Language::Marathi)
    MAR,

    #[cfg(feature = "macedonian")]
    /// The ISO 639-3 code for [`Macedonian`](crate::language::Language::Macedonian)
    MKD,

    #[cfg(feature = "mongolian")]
    /// The ISO 639-3 code for [`Mongolian`](crate::language::Language::Mongolian)
    MON,

    #[cfg(feature = "maori")]
    /// The ISO 639-3 code for [`Maori`](crate::language::Language::Maori)
    MRI,

    #[cfg(feature = "malay")]
    /// The ISO 639-3 code for [`Malay`](crate::language::Language::Malay)
    MSA,

    #[cfg(feature = "dutch")]
    /// The ISO 639-3 code for [`Dutch`](crate::language::Language::Dutch)
    NLD,

    #[cfg(feature = "nynorsk")]
    /// The ISO 639-3 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NNO,

    #[cfg(feature = "bokmal")]
    /// The ISO 639-3 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NOB,

    #[cfg(feature = "punjabi")]
    /// The ISO 639-3 code for [`Punjabi`](crate::language::Language::Punjabi)
    PAN,

    #[cfg(feature = "polish")]
    /// The ISO 639-3 code for [`Polish`](crate::language::Language::Polish)
    POL,

    #[cfg(feature = "portuguese")]
    /// The ISO 639-3 code for [`Portuguese`](crate::language::Language::Portuguese)
    POR,

    #[cfg(feature = "romanian")]
    /// The ISO 639-3 code for [`Romanian`](crate::language::Language::Romanian)
    RON,

    #[cfg(feature = "russian")]
    /// The ISO 639-3 code for [`Russian`](crate::language::Language::Russian)
    RUS,

    #[cfg(feature = "slovak")]
    /// The ISO 639-3 code for [`Slovak`](crate::language::Language::Slovak)
    SLK,

    #[cfg(feature = "slovene")]
    /// The ISO 639-3 code for [`Slovene`](crate::language::Language::Slovene)
    SLV,

    #[cfg(feature = "shona")]
    /// The ISO 639-3 code for [`Shona`](crate::language::Language::Shona)
    SNA,

    #[cfg(feature = "somali")]
    /// The ISO 639-3 code for [`Somali`](crate::language::Language::Somali)
    SOM,

    #[cfg(feature = "sotho")]
    /// The ISO 639-3 code for [`Sotho`](crate::language::Language::Sotho)
    SOT,

    #[cfg(feature = "spanish")]
    /// The ISO 639-3 code for [`Spanish`](crate::language::Language::Spanish)
    SPA,

    #[cfg(feature = "albanian")]
    /// The ISO 639-3 code for [`Albanian`](crate::language::Language::Albanian)
    SQI,

    #[cfg(feature = "serbian")]
    /// The ISO 639-3 code for [`Serbian`](crate::language::Language::Serbian)
    SRP,

    #[cfg(feature = "swahili")]
    /// The ISO 639-3 code for [`Swahili`](crate::language::Language::Swahili)
    SWA,

    #[cfg(feature = "swedish")]
    /// The ISO 639-3 code for [`Swedish`](crate::language::Language::Swedish)
    SWE,

    #[cfg(feature = "tamil")]
    /// The ISO 639-3 code for [`Tamil`](crate::language::Language::Tamil)
    TAM,

    #[cfg(feature = "telugu")]
    /// The ISO 639-3 code for [`Telugu`](crate::language::Language::Telugu)
    TEL,

    #[cfg(feature = "tagalog")]
    /// The ISO 639-3 code for [`Tagalog`](crate::language::Language::Tagalog)
    TGL,

    #[cfg(feature = "thai")]
    /// The ISO 639-3 code for [`Thai`](crate::language::Language::Thai)
    THA,

    #[cfg(feature = "tswana")]
    /// The ISO 639-3 code for [`Tswana`](crate::language::Language::Tswana)
    TSN,

    #[cfg(feature = "tsonga")]
    /// The ISO 639-3 code for [`Tsonga`](crate::language::Language::Tsonga)
    TSO,

    #[cfg(feature = "turkish")]
    /// The ISO 639-3 code for [`Turkish`](crate::language::Language::Turkish)
    TUR,

    #[cfg(feature = "ukrainian")]
    /// The ISO 639-3 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UKR,

    #[cfg(feature = "urdu")]
    /// The ISO 639-3 code for [`Urdu`](crate::language::Language::Urdu)
    URD,

    #[cfg(feature = "vietnamese")]
    /// The ISO 639-3 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VIE,

    #[cfg(feature = "xhosa")]
    /// The ISO 639-3 code for [`Xhosa`](crate::language::Language::Xhosa)
    XHO,

    #[cfg(feature = "yoruba")]
    /// The ISO 639-3 code for [`Yoruba`](crate::language::Language::Yoruba)
    YOR,

    #[cfg(feature = "chinese")]
    /// The ISO 639-3 code for [`Chinese`](crate::language::Language::Chinese)
    ZHO,

    #[cfg(feature = "zulu")]
    /// The ISO 639-3 code for [`Zulu`](crate::language::Language::Zulu)
    ZUL,
}

impl Display for IsoCode639_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Display for IsoCode639_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use strum::ParseError::VariantNotFound;

    #[test]
    fn assert_iso_code_639_1_string_representation_is_correct() {
        assert_eq!(IsoCode639_1::EN.to_string(), "en");
    }

    #[test]
    fn assert_iso_code_639_3_string_representation_is_correct() {
        assert_eq!(IsoCode639_3::ENG.to_string(), "eng");
    }

    #[test]
    fn assert_string_to_iso_code_639_1_is_correct() {
        assert_eq!(IsoCode639_1::from_str("en"), Ok(IsoCode639_1::EN));
        assert_eq!(IsoCode639_1::from_str("12"), Err(VariantNotFound));
    }

    #[test]
    fn assert_string_to_iso_code_639_3_is_correct() {
        assert_eq!(IsoCode639_3::from_str("eng"), Ok(IsoCode639_3::ENG));
        assert_eq!(IsoCode639_3::from_str("123"), Err(VariantNotFound));
    }
}
