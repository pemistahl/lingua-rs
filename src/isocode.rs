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

use std::fmt::{Debug, Display, Formatter, Result};
use std::str::FromStr;
use strum_macros::EnumString;

/// This enum specifies the ISO 639-1 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(Debug, Eq, PartialEq, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
pub enum IsoCode639_1 {
    /// The ISO 639-1 code for [`Afrikaans`](./enum.Language.html#variant.Afrikaans)
    AF,

    /// The ISO 639-1 code for [`Arabic`](./enum.Language.html#variant.Arabic)
    AR,

    /// The ISO 639-1 code for [`Azerbaijani`](./enum.Language.html#variant.Azerbaijani)
    AZ,

    /// The ISO 639-1 code for [`Belarusian`](./enum.Language.html#variant.Belarusian)
    BE,

    /// The ISO 639-1 code for [`Bulgarian`](./enum.Language.html#variant.Bulgarian)
    BG,

    /// The ISO 639-1 code for [`Bengali`](./enum.Language.html#variant.Bengali)
    BN,

    /// The ISO 639-1 code for [`Bosnian`](./enum.Language.html#variant.Bosnian)
    BS,

    /// The ISO 639-1 code for [`Catalan`](./enum.Language.html#variant.Catalan)
    CA,

    /// The ISO 639-1 code for [`Czech`](./enum.Language.html#variant.Czech)
    CS,

    /// The ISO 639-1 code for [`Welsh`](./enum.Language.html#variant.Welsh)
    CY,

    /// The ISO 639-1 code for [`Danish`](./enum.Language.html#variant.Danish)
    DA,

    /// The ISO 639-1 code for [`German`](./enum.Language.html#variant.German)
    DE,

    /// The ISO 639-1 code for [`Greek`](./enum.Language.html#variant.Greek)
    EL,

    /// The ISO 639-1 code for [`English`](./enum.Language.html#variant.English)
    EN,

    /// The ISO 639-1 code for [`Esperanto`](./enum.Language.html#variant.Esperanto)
    EO,

    /// The ISO 639-1 code for [`Spanish`](./enum.Language.html#variant.Spanish)
    ES,

    /// The ISO 639-1 code for [`Estonian`](./enum.Language.html#variant.Estonian)
    ET,

    /// The ISO 639-1 code for [`Basque`](./enum.Language.html#variant.Basque)
    EU,

    /// The ISO 639-1 code for [`Persian`](./enum.Language.html#variant.Persian)
    FA,

    /// The ISO 639-1 code for [`Finnish`](./enum.Language.html#variant.Finnish)
    FI,

    /// The ISO 639-1 code for [`French`](./enum.Language.html#variant.French)
    FR,

    /// The ISO 639-1 code for [`Irish`](./enum.Language.html#variant.Irish)
    GA,

    /// The ISO 639-1 code for [`Gujarati`](./enum.Language.html#variant.Gujarati)
    GU,

    /// The ISO 639-1 code for [`Hebrew`](./enum.Language.html#variant.Hebrew)
    HE,

    /// The ISO 639-1 code for [`Hindi`](./enum.Language.html#variant.Hindi)
    HI,

    /// The ISO 639-1 code for [`Croatian`](./enum.Language.html#variant.Croatian)
    HR,

    /// The ISO 639-1 code for [`Hungarian`](./enum.Language.html#variant.Hungarian)
    HU,

    /// The ISO 639-1 code for [`Armenian`](./enum.Language.html#variant.Armenian)
    HY,

    /// The ISO 639-1 code for [`Indonesian`](./enum.Language.html#variant.Indonesian)
    ID,

    /// The ISO 639-1 code for [`Icelandic`](./enum.Language.html#variant.Icelandic)
    IS,

    /// The ISO 639-1 code for [`Italian`](./enum.Language.html#variant.Italian)
    IT,

    /// The ISO 639-1 code for [`Japanese`](./enum.Language.html#variant.Japanese)
    JA,

    /// The ISO 639-1 code for [`Georgian`](./enum.Language.html#variant.Georgian)
    KA,

    /// The ISO 639-1 code for [`Kazakh`](./enum.Language.html#variant.Kazakh)
    KK,

    /// The ISO 639-1 code for [`Korean`](./enum.Language.html#variant.Korean)
    KO,

    /// The ISO 639-1 code for [`Latin`](./enum.Language.html#variant.Latin)
    LA,

    /// The ISO 639-1 code for [`Ganda`](./enum.Language.html#variant.Ganda)
    LG,

    /// The ISO 639-1 code for [`Lithuanian`](./enum.Language.html#variant.Lithuanian)
    LT,

    /// The ISO 639-1 code for [`Latvian`](./enum.Language.html#variant.Latvian)
    LV,

    /// The ISO 639-1 code for [`Maori`](./enum.Language.html#variant.Maori)
    MI,

    /// The ISO 639-1 code for [`Macedonian`](./enum.Language.html#variant.Macedonian)
    MK,

    /// The ISO 639-1 code for [`Mongolian`](./enum.Language.html#variant.Mongolian)
    MN,

    /// The ISO 639-1 code for [`Marathi`](./enum.Language.html#variant.Marathi)
    MR,

    /// The ISO 639-1 code for [`Malay`](./enum.Language.html#variant.Malay)
    MS,

    /// The ISO 639-1 code for [`Norwegian Bokmal`](./enum.Language.html#variant.Bokmal)
    NB,

    /// The ISO 639-1 code for [`Dutch`](./enum.Language.html#variant.Dutch)
    NL,

    /// The ISO 639-1 code for [`Norwegian Nynorsk`](./enum.Language.html#variant.Nynorsk)
    NN,

    /// The ISO 639-1 code for [`Punjabi`](./enum.Language.html#variant.Punjabi)
    PA,

    /// The ISO 639-1 code for [`Polish`](./enum.Language.html#variant.Polish)
    PL,

    /// The ISO 639-1 code for [`Portuguese`](./enum.Language.html#variant.Portuguese)
    PT,

    /// The ISO 639-1 code for [`Romanian`](./enum.Language.html#variant.Romanian)
    RO,

    /// The ISO 639-1 code for [`Russian`](./enum.Language.html#variant.Russian)
    RU,

    /// The ISO 639-1 code for [`Slovak`](./enum.Language.html#variant.Slovak)
    SK,

    /// The ISO 639-1 code for [`Slovene`](./enum.Language.html#variant.Slovene)
    SL,

    /// The ISO 639-1 code for [`Shona`](./enum.Language.html#variant.Shona)
    SN,

    /// The ISO 639-1 code for [`Somali`](./enum.Language.html#variant.Somali)
    SO,

    /// The ISO 639-1 code for [`Albanian`](./enum.Language.html#variant.Albanian)
    SQ,

    /// The ISO 639-1 code for [`Serbian`](./enum.Language.html#variant.Serbian)
    SR,

    /// The ISO 639-1 code for [`Sotho`](./enum.Language.html#variant.Sotho)
    ST,

    /// The ISO 639-1 code for [`Swedish`](./enum.Language.html#variant.Swedish)
    SV,

    /// The ISO 639-1 code for [`Swahili`](./enum.Language.html#variant.Swahili)
    SW,

    /// The ISO 639-1 code for [`Tamil`](./enum.Language.html#variant.Tamil)
    TA,

    /// The ISO 639-1 code for [`Telugu`](./enum.Language.html#variant.Telugu)
    TE,

    /// The ISO 639-1 code for [`Thai`](./enum.Language.html#variant.Thai)
    TH,

    /// The ISO 639-1 code for [`Tagalog`](./enum.Language.html#variant.Tagalog)
    TL,

    /// The ISO 639-1 code for [`Tswana`](./enum.Language.html#variant.Tswana)
    TN,

    /// The ISO 639-1 code for [`Turkish`](./enum.Language.html#variant.Turkish)
    TR,

    /// The ISO 639-1 code for [`Tsonga`](./enum.Language.html#variant.Tsonga)
    TS,

    /// The ISO 639-1 code for [`Ukrainian`](./enum.Language.html#variant.Ukrainian)
    UK,

    /// The ISO 639-1 code for [`Urdu`](./enum.Language.html#variant.Urdu)
    UR,

    /// The ISO 639-1 code for [`Vietnamese`](./enum.Language.html#variant.Vietnamese)
    VI,

    /// The ISO 639-1 code for [`Xhosa`](./enum.Language.html#variant.Xhosa)
    XH,

    /// The ISO 639-1 code for [`Yoruba`](./enum.Language.html#variant.Yoruba)
    YO,

    /// The ISO 639-1 code for [`Chinese`](./enum.Language.html#variant.Chinese)
    ZH,

    /// The ISO 639-1 code for [`Zulu`](./enum.Language.html#variant.Zulu)
    ZU,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(Debug, Eq, PartialEq, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
pub enum IsoCode639_3 {
    /// The ISO 639-3 code for [`Afrikaans`](./enum.Language.html#variant.Afrikaans)
    AFR,

    /// The ISO 639-3 code for [`Arabic`](./enum.Language.html#variant.Arabic)
    ARA,

    /// The ISO 639-3 code for [`Azerbaijani`](./enum.Language.html#variant.Azerbaijani)
    AZE,

    /// The ISO 639-3 code for [`Belarusian`](./enum.Language.html#variant.Belarusian)
    BEL,

    /// The ISO 639-3 code for [`Bengali`](./enum.Language.html#variant.Bengali)
    BEN,

    /// The ISO 639-3 code for [`Bosnian`](./enum.Language.html#variant.Bosnian)
    BOS,

    /// The ISO 639-3 code for [`Bulgarian`](./enum.Language.html#variant.Bulgarian)
    BUL,

    /// The ISO 639-3 code for [`Catalan`](./enum.Language.html#variant.Catalan)
    CAT,

    /// The ISO 639-3 code for [`Czech`](./enum.Language.html#variant.Czech)
    CES,

    /// The ISO 639-3 code for [`Welsh`](./enum.Language.html#variant.Welsh)
    CYM,

    /// The ISO 639-3 code for [`Danish`](./enum.Language.html#variant.Danish)
    DAN,

    /// The ISO 639-3 code for [`German`](./enum.Language.html#variant.German)
    DEU,

    /// The ISO 639-3 code for [`Greek`](./enum.Language.html#variant.Greek)
    ELL,

    /// The ISO 639-3 code for [`English`](./enum.Language.html#variant.English)
    ENG,

    /// The ISO 639-3 code for [`Esperanto`](./enum.Language.html#variant.Esperanto)
    EPO,

    /// The ISO 639-3 code for [`Estonian`](./enum.Language.html#variant.Estonian)
    EST,

    /// The ISO 639-3 code for [`Basque`](./enum.Language.html#variant.Basque)
    EUS,

    /// The ISO 639-3 code for [`Persian`](./enum.Language.html#variant.Persian)
    FAS,

    /// The ISO 639-3 code for [`Finnish`](./enum.Language.html#variant.Finnish)
    FIN,

    /// The ISO 639-3 code for [`French`](./enum.Language.html#variant.French)
    FRA,

    /// The ISO 639-3 code for [`Irish`](./enum.Language.html#variant.Irish)
    GLE,

    /// The ISO 639-3 code for [`Gujarati`](./enum.Language.html#variant.Gujarati)
    GUJ,

    /// The ISO 639-3 code for [`Hebrew`](./enum.Language.html#variant.Hebrew)
    HEB,

    /// The ISO 639-3 code for [`Hindi`](./enum.Language.html#variant.Hindi)
    HIN,

    /// The ISO 639-3 code for [`Croatian`](./enum.Language.html#variant.Croatian)
    HRV,

    /// The ISO 639-3 code for [`Hungarian`](./enum.Language.html#variant.Hungarian)
    HUN,

    /// The ISO 639-3 code for [`Armenian`](./enum.Language.html#variant.Armenian)
    HYE,

    /// The ISO 639-3 code for [`Indonesian`](./enum.Language.html#variant.Indonesian)
    IND,

    /// The ISO 639-3 code for [`Icelandic`](./enum.Language.html#variant.Icelandic)
    ISL,

    /// The ISO 639-3 code for [`Italian`](./enum.Language.html#variant.Italian)
    ITA,

    /// The ISO 639-3 code for [`Japanese`](./enum.Language.html#variant.Japanese)
    JPN,

    /// The ISO 639-3 code for [`Georgian`](./enum.Language.html#variant.Georgian)
    KAT,

    /// The ISO 639-3 code for [`Kazakh`](./enum.Language.html#variant.Kazakh)
    KAZ,

    /// The ISO 639-3 code for [`Korean`](./enum.Language.html#variant.Korean)
    KOR,

    /// The ISO 639-3 code for [`Latin`](./enum.Language.html#variant.Latin)
    LAT,

    /// The ISO 639-3 code for [`Latvian`](./enum.Language.html#variant.Latvian)
    LAV,

    /// The ISO 639-3 code for [`Lithuanian`](./enum.Language.html#variant.Lithuanian)
    LIT,

    /// The ISO 639-3 code for [`Ganda`](./enum.Language.html#variant.Ganda)
    LUG,

    /// The ISO 639-3 code for [`Marathi`](./enum.Language.html#variant.Marathi)
    MAR,

    /// The ISO 639-3 code for [`Macedonian`](./enum.Language.html#variant.Macedonian)
    MKD,

    /// The ISO 639-3 code for [`Mongolian`](./enum.Language.html#variant.Mongolian)
    MON,

    /// The ISO 639-3 code for [`Maori`](./enum.Language.html#variant.Maori)
    MRI,

    /// The ISO 639-3 code for [`Malay`](./enum.Language.html#variant.Malay)
    MSA,

    /// The ISO 639-3 code for [`Dutch`](./enum.Language.html#variant.Dutch)
    NLD,

    /// The ISO 639-3 code for [`Norwegian Nynorsk`](./enum.Language.html#variant.Nynorsk)
    NNO,

    /// The ISO 639-3 code for [`Norwegian Bokmal`](./enum.Language.html#variant.Bokmal)
    NOB,

    /// The ISO 639-3 code for [`Punjabi`](./enum.Language.html#variant.Punjabi)
    PAN,

    /// The ISO 639-3 code for [`Polish`](./enum.Language.html#variant.Polish)
    POL,

    /// The ISO 639-3 code for [`Portuguese`](./enum.Language.html#variant.Portuguese)
    POR,

    /// The ISO 639-3 code for [`Romanian`](./enum.Language.html#variant.Romanian)
    RON,

    /// The ISO 639-3 code for [`Russian`](./enum.Language.html#variant.Russian)
    RUS,

    /// The ISO 639-3 code for [`Slovak`](./enum.Language.html#variant.Slovak)
    SLK,

    /// The ISO 639-3 code for [`Slovene`](./enum.Language.html#variant.Slovene)
    SLV,

    /// The ISO 639-3 code for [`Shona`](./enum.Language.html#variant.Shona)
    SNA,

    /// The ISO 639-3 code for [`Somali`](./enum.Language.html#variant.Somali)
    SOM,

    /// The ISO 639-3 code for [`Sotho`](./enum.Language.html#variant.Sotho)
    SOT,

    /// The ISO 639-3 code for [`Spanish`](./enum.Language.html#variant.Spanish)
    SPA,

    /// The ISO 639-3 code for [`Albanian`](./enum.Language.html#variant.Albanian)
    SQI,

    /// The ISO 639-3 code for [`Serbian`](./enum.Language.html#variant.Serbian)
    SRP,

    /// The ISO 639-3 code for [`Swahili`](./enum.Language.html#variant.Swahili)
    SWA,

    /// The ISO 639-3 code for [`Swedish`](./enum.Language.html#variant.Swedish)
    SWE,

    /// The ISO 639-3 code for [`Tamil`](./enum.Language.html#variant.Tamil)
    TAM,

    /// The ISO 639-3 code for [`Telugu`](./enum.Language.html#variant.Telugu)
    TEL,

    /// The ISO 639-3 code for [`Tagalog`](./enum.Language.html#variant.Tagalog)
    TGL,

    /// The ISO 639-3 code for [`Thai`](./enum.Language.html#variant.Thai)
    THA,

    /// The ISO 639-3 code for [`Tswana`](./enum.Language.html#variant.Tswana)
    TSN,

    /// The ISO 639-3 code for [`Tsonga`](./enum.Language.html#variant.Tsonga)
    TSO,

    /// The ISO 639-3 code for [`Turkish`](./enum.Language.html#variant.Turkish)
    TUR,

    /// The ISO 639-3 code for [`Ukrainian`](./enum.Language.html#variant.Ukrainian)
    UKR,

    /// The ISO 639-3 code for [`Urdu`](./enum.Language.html#variant.Urdu)
    URD,

    /// The ISO 639-3 code for [`Vietnamese`](./enum.Language.html#variant.Vietnamese)
    VIE,

    /// The ISO 639-3 code for [`Xhosa`](./enum.Language.html#variant.Xhosa)
    XHO,

    /// The ISO 639-3 code for [`Yoruba`](./enum.Language.html#variant.Yoruba)
    YOR,

    /// The ISO 639-3 code for [`Chinese`](./enum.Language.html#variant.Chinese)
    ZHO,

    /// The ISO 639-3 code for [`Zulu`](./enum.Language.html#variant.Zulu)
    ZUL,
}

impl Display for IsoCode639_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{:?}", self);
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Display for IsoCode639_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{:?}", self);
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(IsoCode639_1::from_str("en").unwrap(), IsoCode639_1::EN);
    }

    #[test]
    fn assert_string_to_iso_code_639_3_is_correct() {
        assert_eq!(IsoCode639_3::from_str("eng").unwrap(), IsoCode639_3::ENG);
    }
}
