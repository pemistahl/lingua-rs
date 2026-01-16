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

use crate::Language;
use crate::detector::{CountModelFst, LanguageModelFst};
use include_dir::Dir;
#[cfg(feature = "afrikaans")]
use lingua_afrikaans_language_model::{AFRIKAANS_MODELS_DIRECTORY, AFRIKAANS_TESTDATA_DIRECTORY};
#[cfg(feature = "albanian")]
use lingua_albanian_language_model::{ALBANIAN_MODELS_DIRECTORY, ALBANIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "arabic")]
use lingua_arabic_language_model::{ARABIC_MODELS_DIRECTORY, ARABIC_TESTDATA_DIRECTORY};
#[cfg(feature = "armenian")]
use lingua_armenian_language_model::{ARMENIAN_MODELS_DIRECTORY, ARMENIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "azerbaijani")]
use lingua_azerbaijani_language_model::{
    AZERBAIJANI_MODELS_DIRECTORY, AZERBAIJANI_TESTDATA_DIRECTORY,
};
#[cfg(feature = "basque")]
use lingua_basque_language_model::{BASQUE_MODELS_DIRECTORY, BASQUE_TESTDATA_DIRECTORY};
#[cfg(feature = "belarusian")]
use lingua_belarusian_language_model::{
    BELARUSIAN_MODELS_DIRECTORY, BELARUSIAN_TESTDATA_DIRECTORY,
};
#[cfg(feature = "bengali")]
use lingua_bengali_language_model::{BENGALI_MODELS_DIRECTORY, BENGALI_TESTDATA_DIRECTORY};
#[cfg(feature = "bokmal")]
use lingua_bokmal_language_model::{BOKMAL_MODELS_DIRECTORY, BOKMAL_TESTDATA_DIRECTORY};
#[cfg(feature = "bosnian")]
use lingua_bosnian_language_model::{BOSNIAN_MODELS_DIRECTORY, BOSNIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "bulgarian")]
use lingua_bulgarian_language_model::{BULGARIAN_MODELS_DIRECTORY, BULGARIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "catalan")]
use lingua_catalan_language_model::{CATALAN_MODELS_DIRECTORY, CATALAN_TESTDATA_DIRECTORY};
#[cfg(feature = "chinese")]
use lingua_chinese_language_model::{CHINESE_MODELS_DIRECTORY, CHINESE_TESTDATA_DIRECTORY};
#[cfg(feature = "croatian")]
use lingua_croatian_language_model::{CROATIAN_MODELS_DIRECTORY, CROATIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "czech")]
use lingua_czech_language_model::{CZECH_MODELS_DIRECTORY, CZECH_TESTDATA_DIRECTORY};
#[cfg(feature = "danish")]
use lingua_danish_language_model::{DANISH_MODELS_DIRECTORY, DANISH_TESTDATA_DIRECTORY};
#[cfg(feature = "dutch")]
use lingua_dutch_language_model::{DUTCH_MODELS_DIRECTORY, DUTCH_TESTDATA_DIRECTORY};
#[cfg(feature = "english")]
use lingua_english_language_model::{ENGLISH_MODELS_DIRECTORY, ENGLISH_TESTDATA_DIRECTORY};
#[cfg(feature = "esperanto")]
use lingua_esperanto_language_model::{ESPERANTO_MODELS_DIRECTORY, ESPERANTO_TESTDATA_DIRECTORY};
#[cfg(feature = "estonian")]
use lingua_estonian_language_model::{ESTONIAN_MODELS_DIRECTORY, ESTONIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "finnish")]
use lingua_finnish_language_model::{FINNISH_MODELS_DIRECTORY, FINNISH_TESTDATA_DIRECTORY};
#[cfg(feature = "french")]
use lingua_french_language_model::{FRENCH_MODELS_DIRECTORY, FRENCH_TESTDATA_DIRECTORY};
#[cfg(feature = "ganda")]
use lingua_ganda_language_model::{GANDA_MODELS_DIRECTORY, GANDA_TESTDATA_DIRECTORY};
#[cfg(feature = "georgian")]
use lingua_georgian_language_model::{GEORGIAN_MODELS_DIRECTORY, GEORGIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "german")]
use lingua_german_language_model::{GERMAN_MODELS_DIRECTORY, GERMAN_TESTDATA_DIRECTORY};
#[cfg(feature = "greek")]
use lingua_greek_language_model::{GREEK_MODELS_DIRECTORY, GREEK_TESTDATA_DIRECTORY};
#[cfg(feature = "gujarati")]
use lingua_gujarati_language_model::{GUJARATI_MODELS_DIRECTORY, GUJARATI_TESTDATA_DIRECTORY};
#[cfg(feature = "hebrew")]
use lingua_hebrew_language_model::{HEBREW_MODELS_DIRECTORY, HEBREW_TESTDATA_DIRECTORY};
#[cfg(feature = "hindi")]
use lingua_hindi_language_model::{HINDI_MODELS_DIRECTORY, HINDI_TESTDATA_DIRECTORY};
#[cfg(feature = "hungarian")]
use lingua_hungarian_language_model::{HUNGARIAN_MODELS_DIRECTORY, HUNGARIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "icelandic")]
use lingua_icelandic_language_model::{ICELANDIC_MODELS_DIRECTORY, ICELANDIC_TESTDATA_DIRECTORY};
#[cfg(feature = "indonesian")]
use lingua_indonesian_language_model::{
    INDONESIAN_MODELS_DIRECTORY, INDONESIAN_TESTDATA_DIRECTORY,
};
#[cfg(feature = "irish")]
use lingua_irish_language_model::{IRISH_MODELS_DIRECTORY, IRISH_TESTDATA_DIRECTORY};
#[cfg(feature = "italian")]
use lingua_italian_language_model::{ITALIAN_MODELS_DIRECTORY, ITALIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "japanese")]
use lingua_japanese_language_model::{JAPANESE_MODELS_DIRECTORY, JAPANESE_TESTDATA_DIRECTORY};
#[cfg(feature = "kazakh")]
use lingua_kazakh_language_model::{KAZAKH_MODELS_DIRECTORY, KAZAKH_TESTDATA_DIRECTORY};
#[cfg(feature = "korean")]
use lingua_korean_language_model::{KOREAN_MODELS_DIRECTORY, KOREAN_TESTDATA_DIRECTORY};
#[cfg(feature = "latin")]
use lingua_latin_language_model::{LATIN_MODELS_DIRECTORY, LATIN_TESTDATA_DIRECTORY};
#[cfg(feature = "latvian")]
use lingua_latvian_language_model::{LATVIAN_MODELS_DIRECTORY, LATVIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "lithuanian")]
use lingua_lithuanian_language_model::{
    LITHUANIAN_MODELS_DIRECTORY, LITHUANIAN_TESTDATA_DIRECTORY,
};
#[cfg(feature = "macedonian")]
use lingua_macedonian_language_model::{
    MACEDONIAN_MODELS_DIRECTORY, MACEDONIAN_TESTDATA_DIRECTORY,
};
#[cfg(feature = "malay")]
use lingua_malay_language_model::{MALAY_MODELS_DIRECTORY, MALAY_TESTDATA_DIRECTORY};
#[cfg(feature = "maori")]
use lingua_maori_language_model::{MAORI_MODELS_DIRECTORY, MAORI_TESTDATA_DIRECTORY};
#[cfg(feature = "marathi")]
use lingua_marathi_language_model::{MARATHI_MODELS_DIRECTORY, MARATHI_TESTDATA_DIRECTORY};
#[cfg(feature = "mongolian")]
use lingua_mongolian_language_model::{MONGOLIAN_MODELS_DIRECTORY, MONGOLIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "nynorsk")]
use lingua_nynorsk_language_model::{NYNORSK_MODELS_DIRECTORY, NYNORSK_TESTDATA_DIRECTORY};
#[cfg(feature = "persian")]
use lingua_persian_language_model::{PERSIAN_MODELS_DIRECTORY, PERSIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "polish")]
use lingua_polish_language_model::{POLISH_MODELS_DIRECTORY, POLISH_TESTDATA_DIRECTORY};
#[cfg(feature = "portuguese")]
use lingua_portuguese_language_model::{
    PORTUGUESE_MODELS_DIRECTORY, PORTUGUESE_TESTDATA_DIRECTORY,
};
#[cfg(feature = "punjabi")]
use lingua_punjabi_language_model::{PUNJABI_MODELS_DIRECTORY, PUNJABI_TESTDATA_DIRECTORY};
#[cfg(feature = "romanian")]
use lingua_romanian_language_model::{ROMANIAN_MODELS_DIRECTORY, ROMANIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "russian")]
use lingua_russian_language_model::{RUSSIAN_MODELS_DIRECTORY, RUSSIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "serbian")]
use lingua_serbian_language_model::{SERBIAN_MODELS_DIRECTORY, SERBIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "shona")]
use lingua_shona_language_model::{SHONA_MODELS_DIRECTORY, SHONA_TESTDATA_DIRECTORY};
#[cfg(feature = "slovak")]
use lingua_slovak_language_model::{SLOVAK_MODELS_DIRECTORY, SLOVAK_TESTDATA_DIRECTORY};
#[cfg(feature = "slovene")]
use lingua_slovene_language_model::{SLOVENE_MODELS_DIRECTORY, SLOVENE_TESTDATA_DIRECTORY};
#[cfg(feature = "somali")]
use lingua_somali_language_model::{SOMALI_MODELS_DIRECTORY, SOMALI_TESTDATA_DIRECTORY};
#[cfg(feature = "sotho")]
use lingua_sotho_language_model::{SOTHO_MODELS_DIRECTORY, SOTHO_TESTDATA_DIRECTORY};
#[cfg(feature = "spanish")]
use lingua_spanish_language_model::{SPANISH_MODELS_DIRECTORY, SPANISH_TESTDATA_DIRECTORY};
#[cfg(feature = "swahili")]
use lingua_swahili_language_model::{SWAHILI_MODELS_DIRECTORY, SWAHILI_TESTDATA_DIRECTORY};
#[cfg(feature = "swedish")]
use lingua_swedish_language_model::{SWEDISH_MODELS_DIRECTORY, SWEDISH_TESTDATA_DIRECTORY};
#[cfg(feature = "tagalog")]
use lingua_tagalog_language_model::{TAGALOG_MODELS_DIRECTORY, TAGALOG_TESTDATA_DIRECTORY};
#[cfg(feature = "tamil")]
use lingua_tamil_language_model::{TAMIL_MODELS_DIRECTORY, TAMIL_TESTDATA_DIRECTORY};
#[cfg(feature = "telugu")]
use lingua_telugu_language_model::{TELUGU_MODELS_DIRECTORY, TELUGU_TESTDATA_DIRECTORY};
#[cfg(feature = "thai")]
use lingua_thai_language_model::{THAI_MODELS_DIRECTORY, THAI_TESTDATA_DIRECTORY};
#[cfg(feature = "tsonga")]
use lingua_tsonga_language_model::{TSONGA_MODELS_DIRECTORY, TSONGA_TESTDATA_DIRECTORY};
#[cfg(feature = "tswana")]
use lingua_tswana_language_model::{TSWANA_MODELS_DIRECTORY, TSWANA_TESTDATA_DIRECTORY};
#[cfg(feature = "turkish")]
use lingua_turkish_language_model::{TURKISH_MODELS_DIRECTORY, TURKISH_TESTDATA_DIRECTORY};
#[cfg(feature = "ukrainian")]
use lingua_ukrainian_language_model::{UKRAINIAN_MODELS_DIRECTORY, UKRAINIAN_TESTDATA_DIRECTORY};
#[cfg(feature = "urdu")]
use lingua_urdu_language_model::{URDU_MODELS_DIRECTORY, URDU_TESTDATA_DIRECTORY};
#[cfg(feature = "vietnamese")]
use lingua_vietnamese_language_model::{
    VIETNAMESE_MODELS_DIRECTORY, VIETNAMESE_TESTDATA_DIRECTORY,
};
#[cfg(feature = "welsh")]
use lingua_welsh_language_model::{WELSH_MODELS_DIRECTORY, WELSH_TESTDATA_DIRECTORY};
#[cfg(feature = "xhosa")]
use lingua_xhosa_language_model::{XHOSA_MODELS_DIRECTORY, XHOSA_TESTDATA_DIRECTORY};
#[cfg(feature = "yoruba")]
use lingua_yoruba_language_model::{YORUBA_MODELS_DIRECTORY, YORUBA_TESTDATA_DIRECTORY};
#[cfg(feature = "zulu")]
use lingua_zulu_language_model::{ZULU_MODELS_DIRECTORY, ZULU_TESTDATA_DIRECTORY};
use std::borrow::Cow;
use std::io::ErrorKind;

pub(crate) fn read_probability_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<LanguageModelFst> {
    let directory = get_language_models_directory(language);
    let fst_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    let fst_map = fst::Map::new(Cow::Borrowed(fst_file.contents())).unwrap();
    Ok(fst_map)
}

pub(crate) fn read_count_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<CountModelFst> {
    let directory = get_language_models_directory(language);
    let fst_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    let fst_set = fst::Set::new(Cow::Borrowed(fst_file.contents())).unwrap();
    Ok(fst_set)
}

pub(crate) fn read_test_data_file(language: Language, file_name: &str) -> std::io::Result<&str> {
    let directory = get_test_data_directory(language);
    let test_data_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    Ok(test_data_file.contents_utf8().unwrap())
}

fn get_language_models_directory(language: Language) -> Dir<'static> {
    match language {
        #[cfg(feature = "afrikaans")]
        Language::Afrikaans => AFRIKAANS_MODELS_DIRECTORY,

        #[cfg(feature = "albanian")]
        Language::Albanian => ALBANIAN_MODELS_DIRECTORY,

        #[cfg(feature = "arabic")]
        Language::Arabic => ARABIC_MODELS_DIRECTORY,

        #[cfg(feature = "armenian")]
        Language::Armenian => ARMENIAN_MODELS_DIRECTORY,

        #[cfg(feature = "azerbaijani")]
        Language::Azerbaijani => AZERBAIJANI_MODELS_DIRECTORY,

        #[cfg(feature = "basque")]
        Language::Basque => BASQUE_MODELS_DIRECTORY,

        #[cfg(feature = "belarusian")]
        Language::Belarusian => BELARUSIAN_MODELS_DIRECTORY,

        #[cfg(feature = "bengali")]
        Language::Bengali => BENGALI_MODELS_DIRECTORY,

        #[cfg(feature = "bokmal")]
        Language::Bokmal => BOKMAL_MODELS_DIRECTORY,

        #[cfg(feature = "bosnian")]
        Language::Bosnian => BOSNIAN_MODELS_DIRECTORY,

        #[cfg(feature = "bulgarian")]
        Language::Bulgarian => BULGARIAN_MODELS_DIRECTORY,

        #[cfg(feature = "catalan")]
        Language::Catalan => CATALAN_MODELS_DIRECTORY,

        #[cfg(feature = "chinese")]
        Language::Chinese => CHINESE_MODELS_DIRECTORY,

        #[cfg(feature = "croatian")]
        Language::Croatian => CROATIAN_MODELS_DIRECTORY,

        #[cfg(feature = "czech")]
        Language::Czech => CZECH_MODELS_DIRECTORY,

        #[cfg(feature = "danish")]
        Language::Danish => DANISH_MODELS_DIRECTORY,

        #[cfg(feature = "dutch")]
        Language::Dutch => DUTCH_MODELS_DIRECTORY,

        #[cfg(feature = "english")]
        Language::English => ENGLISH_MODELS_DIRECTORY,

        #[cfg(feature = "esperanto")]
        Language::Esperanto => ESPERANTO_MODELS_DIRECTORY,

        #[cfg(feature = "estonian")]
        Language::Estonian => ESTONIAN_MODELS_DIRECTORY,

        #[cfg(feature = "finnish")]
        Language::Finnish => FINNISH_MODELS_DIRECTORY,

        #[cfg(feature = "french")]
        Language::French => FRENCH_MODELS_DIRECTORY,

        #[cfg(feature = "ganda")]
        Language::Ganda => GANDA_MODELS_DIRECTORY,

        #[cfg(feature = "georgian")]
        Language::Georgian => GEORGIAN_MODELS_DIRECTORY,

        #[cfg(feature = "german")]
        Language::German => GERMAN_MODELS_DIRECTORY,

        #[cfg(feature = "greek")]
        Language::Greek => GREEK_MODELS_DIRECTORY,

        #[cfg(feature = "gujarati")]
        Language::Gujarati => GUJARATI_MODELS_DIRECTORY,

        #[cfg(feature = "hebrew")]
        Language::Hebrew => HEBREW_MODELS_DIRECTORY,

        #[cfg(feature = "hindi")]
        Language::Hindi => HINDI_MODELS_DIRECTORY,

        #[cfg(feature = "hungarian")]
        Language::Hungarian => HUNGARIAN_MODELS_DIRECTORY,

        #[cfg(feature = "icelandic")]
        Language::Icelandic => ICELANDIC_MODELS_DIRECTORY,

        #[cfg(feature = "indonesian")]
        Language::Indonesian => INDONESIAN_MODELS_DIRECTORY,

        #[cfg(feature = "irish")]
        Language::Irish => IRISH_MODELS_DIRECTORY,

        #[cfg(feature = "italian")]
        Language::Italian => ITALIAN_MODELS_DIRECTORY,

        #[cfg(feature = "japanese")]
        Language::Japanese => JAPANESE_MODELS_DIRECTORY,

        #[cfg(feature = "kazakh")]
        Language::Kazakh => KAZAKH_MODELS_DIRECTORY,

        #[cfg(feature = "korean")]
        Language::Korean => KOREAN_MODELS_DIRECTORY,

        #[cfg(feature = "latin")]
        Language::Latin => LATIN_MODELS_DIRECTORY,

        #[cfg(feature = "latvian")]
        Language::Latvian => LATVIAN_MODELS_DIRECTORY,

        #[cfg(feature = "lithuanian")]
        Language::Lithuanian => LITHUANIAN_MODELS_DIRECTORY,

        #[cfg(feature = "macedonian")]
        Language::Macedonian => MACEDONIAN_MODELS_DIRECTORY,

        #[cfg(feature = "malay")]
        Language::Malay => MALAY_MODELS_DIRECTORY,

        #[cfg(feature = "maori")]
        Language::Maori => MAORI_MODELS_DIRECTORY,

        #[cfg(feature = "marathi")]
        Language::Marathi => MARATHI_MODELS_DIRECTORY,

        #[cfg(feature = "mongolian")]
        Language::Mongolian => MONGOLIAN_MODELS_DIRECTORY,

        #[cfg(feature = "nynorsk")]
        Language::Nynorsk => NYNORSK_MODELS_DIRECTORY,

        #[cfg(feature = "persian")]
        Language::Persian => PERSIAN_MODELS_DIRECTORY,

        #[cfg(feature = "polish")]
        Language::Polish => POLISH_MODELS_DIRECTORY,

        #[cfg(feature = "portuguese")]
        Language::Portuguese => PORTUGUESE_MODELS_DIRECTORY,

        #[cfg(feature = "punjabi")]
        Language::Punjabi => PUNJABI_MODELS_DIRECTORY,

        #[cfg(feature = "romanian")]
        Language::Romanian => ROMANIAN_MODELS_DIRECTORY,

        #[cfg(feature = "russian")]
        Language::Russian => RUSSIAN_MODELS_DIRECTORY,

        #[cfg(feature = "serbian")]
        Language::Serbian => SERBIAN_MODELS_DIRECTORY,

        #[cfg(feature = "shona")]
        Language::Shona => SHONA_MODELS_DIRECTORY,

        #[cfg(feature = "slovak")]
        Language::Slovak => SLOVAK_MODELS_DIRECTORY,

        #[cfg(feature = "slovene")]
        Language::Slovene => SLOVENE_MODELS_DIRECTORY,

        #[cfg(feature = "somali")]
        Language::Somali => SOMALI_MODELS_DIRECTORY,

        #[cfg(feature = "sotho")]
        Language::Sotho => SOTHO_MODELS_DIRECTORY,

        #[cfg(feature = "spanish")]
        Language::Spanish => SPANISH_MODELS_DIRECTORY,

        #[cfg(feature = "swahili")]
        Language::Swahili => SWAHILI_MODELS_DIRECTORY,

        #[cfg(feature = "swedish")]
        Language::Swedish => SWEDISH_MODELS_DIRECTORY,

        #[cfg(feature = "tagalog")]
        Language::Tagalog => TAGALOG_MODELS_DIRECTORY,

        #[cfg(feature = "tamil")]
        Language::Tamil => TAMIL_MODELS_DIRECTORY,

        #[cfg(feature = "telugu")]
        Language::Telugu => TELUGU_MODELS_DIRECTORY,

        #[cfg(feature = "thai")]
        Language::Thai => THAI_MODELS_DIRECTORY,

        #[cfg(feature = "tsonga")]
        Language::Tsonga => TSONGA_MODELS_DIRECTORY,

        #[cfg(feature = "tswana")]
        Language::Tswana => TSWANA_MODELS_DIRECTORY,

        #[cfg(feature = "turkish")]
        Language::Turkish => TURKISH_MODELS_DIRECTORY,

        #[cfg(feature = "ukrainian")]
        Language::Ukrainian => UKRAINIAN_MODELS_DIRECTORY,

        #[cfg(feature = "urdu")]
        Language::Urdu => URDU_MODELS_DIRECTORY,

        #[cfg(feature = "vietnamese")]
        Language::Vietnamese => VIETNAMESE_MODELS_DIRECTORY,

        #[cfg(feature = "welsh")]
        Language::Welsh => WELSH_MODELS_DIRECTORY,

        #[cfg(feature = "xhosa")]
        Language::Xhosa => XHOSA_MODELS_DIRECTORY,

        #[cfg(feature = "yoruba")]
        Language::Yoruba => YORUBA_MODELS_DIRECTORY,

        #[cfg(feature = "zulu")]
        Language::Zulu => ZULU_MODELS_DIRECTORY,
    }
}

fn get_test_data_directory(language: Language) -> Dir<'static> {
    match language {
        #[cfg(feature = "afrikaans")]
        Language::Afrikaans => AFRIKAANS_TESTDATA_DIRECTORY,

        #[cfg(feature = "albanian")]
        Language::Albanian => ALBANIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "arabic")]
        Language::Arabic => ARABIC_TESTDATA_DIRECTORY,

        #[cfg(feature = "armenian")]
        Language::Armenian => ARMENIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "azerbaijani")]
        Language::Azerbaijani => AZERBAIJANI_TESTDATA_DIRECTORY,

        #[cfg(feature = "basque")]
        Language::Basque => BASQUE_TESTDATA_DIRECTORY,

        #[cfg(feature = "belarusian")]
        Language::Belarusian => BELARUSIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "bengali")]
        Language::Bengali => BENGALI_TESTDATA_DIRECTORY,

        #[cfg(feature = "bokmal")]
        Language::Bokmal => BOKMAL_TESTDATA_DIRECTORY,

        #[cfg(feature = "bosnian")]
        Language::Bosnian => BOSNIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "bulgarian")]
        Language::Bulgarian => BULGARIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "catalan")]
        Language::Catalan => CATALAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "chinese")]
        Language::Chinese => CHINESE_TESTDATA_DIRECTORY,

        #[cfg(feature = "croatian")]
        Language::Croatian => CROATIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "czech")]
        Language::Czech => CZECH_TESTDATA_DIRECTORY,

        #[cfg(feature = "danish")]
        Language::Danish => DANISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "dutch")]
        Language::Dutch => DUTCH_TESTDATA_DIRECTORY,

        #[cfg(feature = "english")]
        Language::English => ENGLISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "esperanto")]
        Language::Esperanto => ESPERANTO_TESTDATA_DIRECTORY,

        #[cfg(feature = "estonian")]
        Language::Estonian => ESTONIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "finnish")]
        Language::Finnish => FINNISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "french")]
        Language::French => FRENCH_TESTDATA_DIRECTORY,

        #[cfg(feature = "ganda")]
        Language::Ganda => GANDA_TESTDATA_DIRECTORY,

        #[cfg(feature = "georgian")]
        Language::Georgian => GEORGIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "german")]
        Language::German => GERMAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "greek")]
        Language::Greek => GREEK_TESTDATA_DIRECTORY,

        #[cfg(feature = "gujarati")]
        Language::Gujarati => GUJARATI_TESTDATA_DIRECTORY,

        #[cfg(feature = "hebrew")]
        Language::Hebrew => HEBREW_TESTDATA_DIRECTORY,

        #[cfg(feature = "hindi")]
        Language::Hindi => HINDI_TESTDATA_DIRECTORY,

        #[cfg(feature = "hungarian")]
        Language::Hungarian => HUNGARIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "icelandic")]
        Language::Icelandic => ICELANDIC_TESTDATA_DIRECTORY,

        #[cfg(feature = "indonesian")]
        Language::Indonesian => INDONESIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "irish")]
        Language::Irish => IRISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "italian")]
        Language::Italian => ITALIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "japanese")]
        Language::Japanese => JAPANESE_TESTDATA_DIRECTORY,

        #[cfg(feature = "kazakh")]
        Language::Kazakh => KAZAKH_TESTDATA_DIRECTORY,

        #[cfg(feature = "korean")]
        Language::Korean => KOREAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "latin")]
        Language::Latin => LATIN_TESTDATA_DIRECTORY,

        #[cfg(feature = "latvian")]
        Language::Latvian => LATVIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "lithuanian")]
        Language::Lithuanian => LITHUANIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "macedonian")]
        Language::Macedonian => MACEDONIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "malay")]
        Language::Malay => MALAY_TESTDATA_DIRECTORY,

        #[cfg(feature = "maori")]
        Language::Maori => MAORI_TESTDATA_DIRECTORY,

        #[cfg(feature = "marathi")]
        Language::Marathi => MARATHI_TESTDATA_DIRECTORY,

        #[cfg(feature = "mongolian")]
        Language::Mongolian => MONGOLIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "nynorsk")]
        Language::Nynorsk => NYNORSK_TESTDATA_DIRECTORY,

        #[cfg(feature = "persian")]
        Language::Persian => PERSIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "polish")]
        Language::Polish => POLISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "portuguese")]
        Language::Portuguese => PORTUGUESE_TESTDATA_DIRECTORY,

        #[cfg(feature = "punjabi")]
        Language::Punjabi => PUNJABI_TESTDATA_DIRECTORY,

        #[cfg(feature = "romanian")]
        Language::Romanian => ROMANIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "russian")]
        Language::Russian => RUSSIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "serbian")]
        Language::Serbian => SERBIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "shona")]
        Language::Shona => SHONA_TESTDATA_DIRECTORY,

        #[cfg(feature = "slovak")]
        Language::Slovak => SLOVAK_TESTDATA_DIRECTORY,

        #[cfg(feature = "slovene")]
        Language::Slovene => SLOVENE_TESTDATA_DIRECTORY,

        #[cfg(feature = "somali")]
        Language::Somali => SOMALI_TESTDATA_DIRECTORY,

        #[cfg(feature = "sotho")]
        Language::Sotho => SOTHO_TESTDATA_DIRECTORY,

        #[cfg(feature = "spanish")]
        Language::Spanish => SPANISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "swahili")]
        Language::Swahili => SWAHILI_TESTDATA_DIRECTORY,

        #[cfg(feature = "swedish")]
        Language::Swedish => SWEDISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "tagalog")]
        Language::Tagalog => TAGALOG_TESTDATA_DIRECTORY,

        #[cfg(feature = "tamil")]
        Language::Tamil => TAMIL_TESTDATA_DIRECTORY,

        #[cfg(feature = "telugu")]
        Language::Telugu => TELUGU_TESTDATA_DIRECTORY,

        #[cfg(feature = "thai")]
        Language::Thai => THAI_TESTDATA_DIRECTORY,

        #[cfg(feature = "tsonga")]
        Language::Tsonga => TSONGA_TESTDATA_DIRECTORY,

        #[cfg(feature = "tswana")]
        Language::Tswana => TSWANA_TESTDATA_DIRECTORY,

        #[cfg(feature = "turkish")]
        Language::Turkish => TURKISH_TESTDATA_DIRECTORY,

        #[cfg(feature = "ukrainian")]
        Language::Ukrainian => UKRAINIAN_TESTDATA_DIRECTORY,

        #[cfg(feature = "urdu")]
        Language::Urdu => URDU_TESTDATA_DIRECTORY,

        #[cfg(feature = "vietnamese")]
        Language::Vietnamese => VIETNAMESE_TESTDATA_DIRECTORY,

        #[cfg(feature = "welsh")]
        Language::Welsh => WELSH_TESTDATA_DIRECTORY,

        #[cfg(feature = "xhosa")]
        Language::Xhosa => XHOSA_TESTDATA_DIRECTORY,

        #[cfg(feature = "yoruba")]
        Language::Yoruba => YORUBA_TESTDATA_DIRECTORY,

        #[cfg(feature = "zulu")]
        Language::Zulu => ZULU_TESTDATA_DIRECTORY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{NGRAM_PROBABILITY_MODEL_FILE_NAME, NgramCountModelType};

    #[test]
    fn test_read_probability_model_data_file() {
        let result =
            read_probability_model_data_file(Language::English, NGRAM_PROBABILITY_MODEL_FILE_NAME);
        assert!(result.is_ok());

        let fst_map = result.unwrap();
        assert!(fst_map.contains_key(b"that".to_vec()));
    }

    #[test]
    fn test_read_count_model_data_file() {
        let result = read_count_model_data_file(
            Language::English,
            &NgramCountModelType::MostCommon.file_name(),
        );
        assert!(result.is_ok());
    }
}
