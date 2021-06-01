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
pub enum IsoCode639_1 {
    /// The ISO 639-1 code for [`Afrikaans`](./enum.Language.html#variant.Afrikaans)
    #[strum(serialize = "af")]
    AF,

    /// The ISO 639-1 code for [`Arabic`](./enum.Language.html#variant.Arabic)
    #[strum(serialize = "ar")]
    AR,

    /// The ISO 639-1 code for [`Azerbaijani`](./enum.Language.html#variant.Azerbaijani)
    #[strum(serialize = "az")]
    AZ,

    /// The ISO 639-1 code for [`Belarusian`](./enum.Language.html#variant.Belarusian)
    #[strum(serialize = "be")]
    BE,

    /// The ISO 639-1 code for [`Bulgarian`](./enum.Language.html#variant.Bulgarian)
    #[strum(serialize = "bg")]
    BG,

    /// The ISO 639-1 code for [`Bengali`](./enum.Language.html#variant.Bengali)
    #[strum(serialize = "bn")]
    BN,

    /// The ISO 639-1 code for [`Bosnian`](./enum.Language.html#variant.Bosnian)
    #[strum(serialize = "bs")]
    BS,

    /// The ISO 639-1 code for [`Catalan`](./enum.Language.html#variant.Catalan)
    #[strum(serialize = "ca")]
    CA,

    /// The ISO 639-1 code for [`Czech`](./enum.Language.html#variant.Czech)
    #[strum(serialize = "cs")]
    CS,

    /// The ISO 639-1 code for [`Welsh`](./enum.Language.html#variant.Welsh)
    #[strum(serialize = "cy")]
    CY,

    /// The ISO 639-1 code for [`Danish`](./enum.Language.html#variant.Danish)
    #[strum(serialize = "da")]
    DA,

    /// The ISO 639-1 code for [`German`](./enum.Language.html#variant.German)
    #[strum(serialize = "de")]
    DE,

    /// The ISO 639-1 code for [`Greek`](./enum.Language.html#variant.Greek)
    #[strum(serialize = "el")]
    EL,

    /// The ISO 639-1 code for [`English`](./enum.Language.html#variant.English)
    #[strum(serialize = "en")]
    EN,

    /// The ISO 639-1 code for [`Esperanto`](./enum.Language.html#variant.Esperanto)
    #[strum(serialize = "eo")]
    EO,

    /// The ISO 639-1 code for [`Spanish`](./enum.Language.html#variant.Spanish)
    #[strum(serialize = "es")]
    ES,

    /// The ISO 639-1 code for [`Estonian`](./enum.Language.html#variant.Estonian)
    #[strum(serialize = "et")]
    ET,

    /// The ISO 639-1 code for [`Basque`](./enum.Language.html#variant.Basque)
    #[strum(serialize = "eu")]
    EU,

    /// The ISO 639-1 code for [`Persian`](./enum.Language.html#variant.Persian)
    #[strum(serialize = "fa")]
    FA,

    /// The ISO 639-1 code for [`Finnish`](./enum.Language.html#variant.Finnish)
    #[strum(serialize = "fi")]
    FI,

    /// The ISO 639-1 code for [`French`](./enum.Language.html#variant.French)
    #[strum(serialize = "fr")]
    FR,

    /// The ISO 639-1 code for [`Irish`](./enum.Language.html#variant.Irish)
    #[strum(serialize = "ga")]
    GA,

    /// The ISO 639-1 code for [`Gujarati`](./enum.Language.html#variant.Gujarati)
    #[strum(serialize = "gu")]
    GU,

    /// The ISO 639-1 code for [`Hebrew`](./enum.Language.html#variant.Hebrew)
    #[strum(serialize = "he")]
    HE,

    /// The ISO 639-1 code for [`Hindi`](./enum.Language.html#variant.Hindi)
    #[strum(serialize = "hi")]
    HI,

    /// The ISO 639-1 code for [`Croatian`](./enum.Language.html#variant.Croatian)
    #[strum(serialize = "hr")]
    HR,

    /// The ISO 639-1 code for [`Hungarian`](./enum.Language.html#variant.Hungarian)
    #[strum(serialize = "hu")]
    HU,

    /// The ISO 639-1 code for [`Armenian`](./enum.Language.html#variant.Armenian)
    #[strum(serialize = "hy")]
    HY,

    /// The ISO 639-1 code for [`Indonesian`](./enum.Language.html#variant.Indonesian)
    #[strum(serialize = "id")]
    ID,

    /// The ISO 639-1 code for [`Icelandic`](./enum.Language.html#variant.Icelandic)
    #[strum(serialize = "is")]
    IS,

    /// The ISO 639-1 code for [`Italian`](./enum.Language.html#variant.Italian)
    #[strum(serialize = "it")]
    IT,

    /// The ISO 639-1 code for [`Japanese`](./enum.Language.html#variant.Japanese)
    #[strum(serialize = "ja")]
    JA,

    /// The ISO 639-1 code for [`Georgian`](./enum.Language.html#variant.Georgian)
    #[strum(serialize = "ka")]
    KA,

    /// The ISO 639-1 code for [`Kazakh`](./enum.Language.html#variant.Kazakh)
    #[strum(serialize = "kk")]
    KK,

    /// The ISO 639-1 code for [`Korean`](./enum.Language.html#variant.Korean)
    #[strum(serialize = "ko")]
    KO,

    /// The ISO 639-1 code for [`Latin`](./enum.Language.html#variant.Latin)
    #[strum(serialize = "la")]
    LA,

    /// The ISO 639-1 code for [`Ganda`](./enum.Language.html#variant.Ganda)
    #[strum(serialize = "lg")]
    LG,

    /// The ISO 639-1 code for [`Lithuanian`](./enum.Language.html#variant.Lithuanian)
    #[strum(serialize = "lt")]
    LT,

    /// The ISO 639-1 code for [`Latvian`](./enum.Language.html#variant.Latvian)
    #[strum(serialize = "lv")]
    LV,

    /// The ISO 639-1 code for [`Maori`](./enum.Language.html#variant.Maori)
    #[strum(serialize = "mi")]
    MI,

    /// The ISO 639-1 code for [`Macedonian`](./enum.Language.html#variant.Macedonian)
    #[strum(serialize = "mk")]
    MK,

    /// The ISO 639-1 code for [`Mongolian`](./enum.Language.html#variant.Mongolian)
    #[strum(serialize = "mn")]
    MN,

    /// The ISO 639-1 code for [`Marathi`](./enum.Language.html#variant.Marathi)
    #[strum(serialize = "mr")]
    MR,

    /// The ISO 639-1 code for [`Malay`](./enum.Language.html#variant.Malay)
    #[strum(serialize = "ms")]
    MS,

    /// The ISO 639-1 code for [`Norwegian Bokmal`](./enum.Language.html#variant.Bokmal)
    #[strum(serialize = "nb")]
    NB,

    /// The ISO 639-1 code for [`Dutch`](./enum.Language.html#variant.Dutch)
    #[strum(serialize = "nl")]
    NL,

    /// The ISO 639-1 code for [`Norwegian Nynorsk`](./enum.Language.html#variant.Nynorsk)
    #[strum(serialize = "nn")]
    NN,

    /// The ISO 639-1 code for [`Punjabi`](./enum.Language.html#variant.Punjabi)
    #[strum(serialize = "pa")]
    PA,

    /// The ISO 639-1 code for [`Polish`](./enum.Language.html#variant.Polish)
    #[strum(serialize = "pl")]
    PL,

    /// The ISO 639-1 code for [`Portuguese`](./enum.Language.html#variant.Portuguese)
    #[strum(serialize = "pt")]
    PT,

    /// The ISO 639-1 code for [`Romanian`](./enum.Language.html#variant.Romanian)
    #[strum(serialize = "ro")]
    RO,

    /// The ISO 639-1 code for [`Russian`](./enum.Language.html#variant.Russian)
    #[strum(serialize = "ru")]
    RU,

    /// The ISO 639-1 code for [`Slovak`](./enum.Language.html#variant.Slovak)
    #[strum(serialize = "sk")]
    SK,

    /// The ISO 639-1 code for [`Slovene`](./enum.Language.html#variant.Slovene)
    #[strum(serialize = "sl")]
    SL,

    /// The ISO 639-1 code for [`Shona`](./enum.Language.html#variant.Shona)
    #[strum(serialize = "sn")]
    SN,

    /// The ISO 639-1 code for [`Somali`](./enum.Language.html#variant.Somali)
    #[strum(serialize = "so")]
    SO,

    /// The ISO 639-1 code for [`Albanian`](./enum.Language.html#variant.Albanian)
    #[strum(serialize = "sq")]
    SQ,

    /// The ISO 639-1 code for [`Serbian`](./enum.Language.html#variant.Serbian)
    #[strum(serialize = "sr")]
    SR,

    /// The ISO 639-1 code for [`Sotho`](./enum.Language.html#variant.Sotho)
    #[strum(serialize = "st")]
    ST,

    /// The ISO 639-1 code for [`Swedish`](./enum.Language.html#variant.Swedish)
    #[strum(serialize = "sv")]
    SV,

    /// The ISO 639-1 code for [`Swahili`](./enum.Language.html#variant.Swahili)
    #[strum(serialize = "sw")]
    SW,

    /// The ISO 639-1 code for [`Tamil`](./enum.Language.html#variant.Tamil)
    #[strum(serialize = "ta")]
    TA,

    /// The ISO 639-1 code for [`Telugu`](./enum.Language.html#variant.Telugu)
    #[strum(serialize = "te")]
    TE,

    /// The ISO 639-1 code for [`Thai`](./enum.Language.html#variant.Thai)
    #[strum(serialize = "th")]
    TH,

    /// The ISO 639-1 code for [`Tagalog`](./enum.Language.html#variant.Tagalog)
    #[strum(serialize = "tl")]
    TL,

    /// The ISO 639-1 code for [`Tswana`](./enum.Language.html#variant.Tswana)
    #[strum(serialize = "tn")]
    TN,

    /// The ISO 639-1 code for [`Turkish`](./enum.Language.html#variant.Turkish)
    #[strum(serialize = "tr")]
    TR,

    /// The ISO 639-1 code for [`Tsonga`](./enum.Language.html#variant.Tsonga)
    #[strum(serialize = "ts")]
    TS,

    /// The ISO 639-1 code for [`Ukrainian`](./enum.Language.html#variant.Ukrainian)
    #[strum(serialize = "uk")]
    UK,

    /// The ISO 639-1 code for [`Urdu`](./enum.Language.html#variant.Urdu)
    #[strum(serialize = "ur")]
    UR,

    /// The ISO 639-1 code for [`Vietnamese`](./enum.Language.html#variant.Vietnamese)
    #[strum(serialize = "vi")]
    VI,

    /// The ISO 639-1 code for [`Xhosa`](./enum.Language.html#variant.Xhosa)
    #[strum(serialize = "xh")]
    XH,

    /// The ISO 639-1 code for [`Yoruba`](./enum.Language.html#variant.Yoruba)
    #[strum(serialize = "yo")]
    YO,

    /// The ISO 639-1 code for [`Chinese`](./enum.Language.html#variant.Chinese)
    #[strum(serialize = "zh")]
    ZH,

    /// The ISO 639-1 code for [`Zulu`](./enum.Language.html#variant.Zulu)
    #[strum(serialize = "zu")]
    ZU,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(Debug, Eq, PartialEq, EnumString)]
#[allow(clippy::upper_case_acronyms)]
pub enum IsoCode639_3 {
    /// The ISO 639-3 code for [`Afrikaans`](./enum.Language.html#variant.Afrikaans)
    #[strum(serialize = "afr")]
    AFR,

    /// The ISO 639-3 code for [`Arabic`](./enum.Language.html#variant.Arabic)
    #[strum(serialize = "ara")]
    ARA,

    /// The ISO 639-3 code for [`Azerbaijani`](./enum.Language.html#variant.Azerbaijani)
    #[strum(serialize = "aze")]
    AZE,

    /// The ISO 639-3 code for [`Belarusian`](./enum.Language.html#variant.Belarusian)
    #[strum(serialize = "bel")]
    BEL,

    /// The ISO 639-3 code for [`Bengali`](./enum.Language.html#variant.Bengali)
    #[strum(serialize = "ben")]
    BEN,

    /// The ISO 639-3 code for [`Bosnian`](./enum.Language.html#variant.Bosnian)
    #[strum(serialize = "bos")]
    BOS,

    /// The ISO 639-3 code for [`Bulgarian`](./enum.Language.html#variant.Bulgarian)
    #[strum(serialize = "bul")]
    BUL,

    /// The ISO 639-3 code for [`Catalan`](./enum.Language.html#variant.Catalan)
    #[strum(serialize = "cat")]
    CAT,

    /// The ISO 639-3 code for [`Czech`](./enum.Language.html#variant.Czech)
    #[strum(serialize = "ces")]
    CES,

    /// The ISO 639-3 code for [`Welsh`](./enum.Language.html#variant.Welsh)
    #[strum(serialize = "cym")]
    CYM,

    /// The ISO 639-3 code for [`Danish`](./enum.Language.html#variant.Danish)
    #[strum(serialize = "dan")]
    DAN,

    /// The ISO 639-3 code for [`German`](./enum.Language.html#variant.German)
    #[strum(serialize = "deu")]
    DEU,

    /// The ISO 639-3 code for [`Greek`](./enum.Language.html#variant.Greek)
    #[strum(serialize = "ell")]
    ELL,

    /// The ISO 639-3 code for [`English`](./enum.Language.html#variant.English)
    #[strum(serialize = "eng")]
    ENG,

    /// The ISO 639-3 code for [`Esperanto`](./enum.Language.html#variant.Esperanto)
    #[strum(serialize = "epo")]
    EPO,

    /// The ISO 639-3 code for [`Estonian`](./enum.Language.html#variant.Estonian)
    #[strum(serialize = "est")]
    EST,

    /// The ISO 639-3 code for [`Basque`](./enum.Language.html#variant.Basque)
    #[strum(serialize = "eus")]
    EUS,

    /// The ISO 639-3 code for [`Persian`](./enum.Language.html#variant.Persian)
    #[strum(serialize = "fas")]
    FAS,

    /// The ISO 639-3 code for [`Finnish`](./enum.Language.html#variant.Finnish)
    #[strum(serialize = "fin")]
    FIN,

    /// The ISO 639-3 code for [`French`](./enum.Language.html#variant.French)
    #[strum(serialize = "fra")]
    FRA,

    /// The ISO 639-3 code for [`Irish`](./enum.Language.html#variant.Irish)
    #[strum(serialize = "gle")]
    GLE,

    /// The ISO 639-3 code for [`Gujarati`](./enum.Language.html#variant.Gujarati)
    #[strum(serialize = "guj")]
    GUJ,

    /// The ISO 639-3 code for [`Hebrew`](./enum.Language.html#variant.Hebrew)
    #[strum(serialize = "heb")]
    HEB,

    /// The ISO 639-3 code for [`Hindi`](./enum.Language.html#variant.Hindi)
    #[strum(serialize = "hin")]
    HIN,

    /// The ISO 639-3 code for [`Croatian`](./enum.Language.html#variant.Croatian)
    #[strum(serialize = "hrv")]
    HRV,

    /// The ISO 639-3 code for [`Hungarian`](./enum.Language.html#variant.Hungarian)
    #[strum(serialize = "hun")]
    HUN,

    /// The ISO 639-3 code for [`Armenian`](./enum.Language.html#variant.Armenian)
    #[strum(serialize = "hye")]
    HYE,

    /// The ISO 639-3 code for [`Indonesian`](./enum.Language.html#variant.Indonesian)
    #[strum(serialize = "ind")]
    IND,

    /// The ISO 639-3 code for [`Icelandic`](./enum.Language.html#variant.Icelandic)
    #[strum(serialize = "isl")]
    ISL,

    /// The ISO 639-3 code for [`Italian`](./enum.Language.html#variant.Italian)
    #[strum(serialize = "ita")]
    ITA,

    /// The ISO 639-3 code for [`Japanese`](./enum.Language.html#variant.Japanese)
    #[strum(serialize = "jpn")]
    JPN,

    /// The ISO 639-3 code for [`Georgian`](./enum.Language.html#variant.Georgian)
    #[strum(serialize = "kat")]
    KAT,

    /// The ISO 639-3 code for [`Kazakh`](./enum.Language.html#variant.Kazakh)
    #[strum(serialize = "kaz")]
    KAZ,

    /// The ISO 639-3 code for [`Korean`](./enum.Language.html#variant.Korean)
    #[strum(serialize = "kor")]
    KOR,

    /// The ISO 639-3 code for [`Latin`](./enum.Language.html#variant.Latin)
    #[strum(serialize = "lat")]
    LAT,

    /// The ISO 639-3 code for [`Latvian`](./enum.Language.html#variant.Latvian)
    #[strum(serialize = "lav")]
    LAV,

    /// The ISO 639-3 code for [`Lithuanian`](./enum.Language.html#variant.Lithuanian)
    #[strum(serialize = "lit")]
    LIT,

    /// The ISO 639-3 code for [`Ganda`](./enum.Language.html#variant.Ganda)
    #[strum(serialize = "lug")]
    LUG,

    /// The ISO 639-3 code for [`Marathi`](./enum.Language.html#variant.Marathi)
    #[strum(serialize = "mar")]
    MAR,

    /// The ISO 639-3 code for [`Macedonian`](./enum.Language.html#variant.Macedonian)
    #[strum(serialize = "mkd")]
    MKD,

    /// The ISO 639-3 code for [`Mongolian`](./enum.Language.html#variant.Mongolian)
    #[strum(serialize = "mon")]
    MON,

    /// The ISO 639-3 code for [`Maori`](./enum.Language.html#variant.Maori)
    #[strum(serialize = "mri")]
    MRI,

    /// The ISO 639-3 code for [`Malay`](./enum.Language.html#variant.Malay)
    #[strum(serialize = "msa")]
    MSA,

    /// The ISO 639-3 code for [`Dutch`](./enum.Language.html#variant.Dutch)
    #[strum(serialize = "nld")]
    NLD,

    /// The ISO 639-3 code for [`Norwegian Nynorsk`](./enum.Language.html#variant.Nynorsk)
    #[strum(serialize = "nno")]
    NNO,

    /// The ISO 639-3 code for [`Norwegian Bokmal`](./enum.Language.html#variant.Bokmal)
    #[strum(serialize = "nob")]
    NOB,

    /// The ISO 639-3 code for [`Punjabi`](./enum.Language.html#variant.Punjabi)
    #[strum(serialize = "pan")]
    PAN,

    /// The ISO 639-3 code for [`Polish`](./enum.Language.html#variant.Polish)
    #[strum(serialize = "pol")]
    POL,

    /// The ISO 639-3 code for [`Portuguese`](./enum.Language.html#variant.Portuguese)
    #[strum(serialize = "por")]
    POR,

    /// The ISO 639-3 code for [`Romanian`](./enum.Language.html#variant.Romanian)
    #[strum(serialize = "ron")]
    RON,

    /// The ISO 639-3 code for [`Russian`](./enum.Language.html#variant.Russian)
    #[strum(serialize = "rus")]
    RUS,

    /// The ISO 639-3 code for [`Slovak`](./enum.Language.html#variant.Slovak)
    #[strum(serialize = "slk")]
    SLK,

    /// The ISO 639-3 code for [`Slovene`](./enum.Language.html#variant.Slovene)
    #[strum(serialize = "slv")]
    SLV,

    /// The ISO 639-3 code for [`Shona`](./enum.Language.html#variant.Shona)
    #[strum(serialize = "sna")]
    SNA,

    /// The ISO 639-3 code for [`Somali`](./enum.Language.html#variant.Somali)
    #[strum(serialize = "som")]
    SOM,

    /// The ISO 639-3 code for [`Sotho`](./enum.Language.html#variant.Sotho)
    #[strum(serialize = "sot")]
    SOT,

    /// The ISO 639-3 code for [`Spanish`](./enum.Language.html#variant.Spanish)
    #[strum(serialize = "spa")]
    SPA,

    /// The ISO 639-3 code for [`Albanian`](./enum.Language.html#variant.Albanian)
    #[strum(serialize = "sqi")]
    SQI,

    /// The ISO 639-3 code for [`Serbian`](./enum.Language.html#variant.Serbian)
    #[strum(serialize = "srp")]
    SRP,

    /// The ISO 639-3 code for [`Swahili`](./enum.Language.html#variant.Swahili)
    #[strum(serialize = "swa")]
    SWA,

    /// The ISO 639-3 code for [`Swedish`](./enum.Language.html#variant.Swedish)
    #[strum(serialize = "swe")]
    SWE,

    /// The ISO 639-3 code for [`Tamil`](./enum.Language.html#variant.Tamil)
    #[strum(serialize = "tam")]
    TAM,

    /// The ISO 639-3 code for [`Telugu`](./enum.Language.html#variant.Telugu)
    #[strum(serialize = "tel")]
    TEL,

    /// The ISO 639-3 code for [`Tagalog`](./enum.Language.html#variant.Tagalog)
    #[strum(serialize = "tgl")]
    TGL,

    /// The ISO 639-3 code for [`Thai`](./enum.Language.html#variant.Thai)
    #[strum(serialize = "tha")]
    THA,

    /// The ISO 639-3 code for [`Tswana`](./enum.Language.html#variant.Tswana)
    #[strum(serialize = "tsn")]
    TSN,

    /// The ISO 639-3 code for [`Tsonga`](./enum.Language.html#variant.Tsonga)
    #[strum(serialize = "tso")]
    TSO,

    /// The ISO 639-3 code for [`Turkish`](./enum.Language.html#variant.Turkish)
    #[strum(serialize = "tur")]
    TUR,

    /// The ISO 639-3 code for [`Ukrainian`](./enum.Language.html#variant.Ukrainian)
    #[strum(serialize = "ukr")]
    UKR,

    /// The ISO 639-3 code for [`Urdu`](./enum.Language.html#variant.Urdu)
    #[strum(serialize = "urd")]
    URD,

    /// The ISO 639-3 code for [`Vietnamese`](./enum.Language.html#variant.Vietnamese)
    #[strum(serialize = "vie")]
    VIE,

    /// The ISO 639-3 code for [`Xhosa`](./enum.Language.html#variant.Xhosa)
    #[strum(serialize = "xho")]
    XHO,

    /// The ISO 639-3 code for [`Yoruba`](./enum.Language.html#variant.Yoruba)
    #[strum(serialize = "yor")]
    YOR,

    /// The ISO 639-3 code for [`Chinese`](./enum.Language.html#variant.Chinese)
    #[strum(serialize = "zho")]
    ZHO,

    /// The ISO 639-3 code for [`Zulu`](./enum.Language.html#variant.Zulu)
    #[strum(serialize = "zul")]
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
