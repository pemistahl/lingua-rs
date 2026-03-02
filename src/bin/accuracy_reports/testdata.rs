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

use include_dir::Dir;
use lingua::Language;
use lingua_afrikaans_language_model::AFRIKAANS_TESTDATA_DIRECTORY;
use lingua_albanian_language_model::ALBANIAN_TESTDATA_DIRECTORY;
use lingua_arabic_language_model::ARABIC_TESTDATA_DIRECTORY;
use lingua_armenian_language_model::ARMENIAN_TESTDATA_DIRECTORY;
use lingua_azerbaijani_language_model::AZERBAIJANI_TESTDATA_DIRECTORY;
use lingua_basque_language_model::BASQUE_TESTDATA_DIRECTORY;
use lingua_belarusian_language_model::BELARUSIAN_TESTDATA_DIRECTORY;
use lingua_bengali_language_model::BENGALI_TESTDATA_DIRECTORY;
use lingua_bokmal_language_model::BOKMAL_TESTDATA_DIRECTORY;
use lingua_bosnian_language_model::BOSNIAN_TESTDATA_DIRECTORY;
use lingua_bulgarian_language_model::BULGARIAN_TESTDATA_DIRECTORY;
use lingua_catalan_language_model::CATALAN_TESTDATA_DIRECTORY;
use lingua_chinese_language_model::CHINESE_TESTDATA_DIRECTORY;
use lingua_croatian_language_model::CROATIAN_TESTDATA_DIRECTORY;
use lingua_czech_language_model::CZECH_TESTDATA_DIRECTORY;
use lingua_danish_language_model::DANISH_TESTDATA_DIRECTORY;
use lingua_dutch_language_model::DUTCH_TESTDATA_DIRECTORY;
use lingua_english_language_model::ENGLISH_TESTDATA_DIRECTORY;
use lingua_esperanto_language_model::ESPERANTO_TESTDATA_DIRECTORY;
use lingua_estonian_language_model::ESTONIAN_TESTDATA_DIRECTORY;
use lingua_finnish_language_model::FINNISH_TESTDATA_DIRECTORY;
use lingua_french_language_model::FRENCH_TESTDATA_DIRECTORY;
use lingua_ganda_language_model::GANDA_TESTDATA_DIRECTORY;
use lingua_georgian_language_model::GEORGIAN_TESTDATA_DIRECTORY;
use lingua_german_language_model::GERMAN_TESTDATA_DIRECTORY;
use lingua_greek_language_model::GREEK_TESTDATA_DIRECTORY;
use lingua_gujarati_language_model::GUJARATI_TESTDATA_DIRECTORY;
use lingua_hebrew_language_model::HEBREW_TESTDATA_DIRECTORY;
use lingua_hindi_language_model::HINDI_TESTDATA_DIRECTORY;
use lingua_hungarian_language_model::HUNGARIAN_TESTDATA_DIRECTORY;
use lingua_icelandic_language_model::ICELANDIC_TESTDATA_DIRECTORY;
use lingua_indonesian_language_model::INDONESIAN_TESTDATA_DIRECTORY;
use lingua_irish_language_model::IRISH_TESTDATA_DIRECTORY;
use lingua_italian_language_model::ITALIAN_TESTDATA_DIRECTORY;
use lingua_japanese_language_model::JAPANESE_TESTDATA_DIRECTORY;
use lingua_kazakh_language_model::KAZAKH_TESTDATA_DIRECTORY;
use lingua_korean_language_model::KOREAN_TESTDATA_DIRECTORY;
use lingua_latin_language_model::LATIN_TESTDATA_DIRECTORY;
use lingua_latvian_language_model::LATVIAN_TESTDATA_DIRECTORY;
use lingua_lithuanian_language_model::LITHUANIAN_TESTDATA_DIRECTORY;
use lingua_macedonian_language_model::MACEDONIAN_TESTDATA_DIRECTORY;
use lingua_malay_language_model::MALAY_TESTDATA_DIRECTORY;
use lingua_maori_language_model::MAORI_TESTDATA_DIRECTORY;
use lingua_marathi_language_model::MARATHI_TESTDATA_DIRECTORY;
use lingua_mongolian_language_model::MONGOLIAN_TESTDATA_DIRECTORY;
use lingua_nynorsk_language_model::NYNORSK_TESTDATA_DIRECTORY;
use lingua_persian_language_model::PERSIAN_TESTDATA_DIRECTORY;
use lingua_polish_language_model::POLISH_TESTDATA_DIRECTORY;
use lingua_portuguese_language_model::PORTUGUESE_TESTDATA_DIRECTORY;
use lingua_punjabi_language_model::PUNJABI_TESTDATA_DIRECTORY;
use lingua_romanian_language_model::ROMANIAN_TESTDATA_DIRECTORY;
use lingua_russian_language_model::RUSSIAN_TESTDATA_DIRECTORY;
use lingua_serbian_language_model::SERBIAN_TESTDATA_DIRECTORY;
use lingua_shona_language_model::SHONA_TESTDATA_DIRECTORY;
use lingua_slovak_language_model::SLOVAK_TESTDATA_DIRECTORY;
use lingua_slovene_language_model::SLOVENE_TESTDATA_DIRECTORY;
use lingua_somali_language_model::SOMALI_TESTDATA_DIRECTORY;
use lingua_sotho_language_model::SOTHO_TESTDATA_DIRECTORY;
use lingua_spanish_language_model::SPANISH_TESTDATA_DIRECTORY;
use lingua_swahili_language_model::SWAHILI_TESTDATA_DIRECTORY;
use lingua_swedish_language_model::SWEDISH_TESTDATA_DIRECTORY;
use lingua_tagalog_language_model::TAGALOG_TESTDATA_DIRECTORY;
use lingua_tamil_language_model::TAMIL_TESTDATA_DIRECTORY;
use lingua_telugu_language_model::TELUGU_TESTDATA_DIRECTORY;
use lingua_thai_language_model::THAI_TESTDATA_DIRECTORY;
use lingua_tsonga_language_model::TSONGA_TESTDATA_DIRECTORY;
use lingua_tswana_language_model::TSWANA_TESTDATA_DIRECTORY;
use lingua_turkish_language_model::TURKISH_TESTDATA_DIRECTORY;
use lingua_ukrainian_language_model::UKRAINIAN_TESTDATA_DIRECTORY;
use lingua_urdu_language_model::URDU_TESTDATA_DIRECTORY;
use lingua_venetian_language_model::VENETIAN_TESTDATA_DIRECTORY;
use lingua_vietnamese_language_model::VIETNAMESE_TESTDATA_DIRECTORY;
use lingua_welsh_language_model::WELSH_TESTDATA_DIRECTORY;
use lingua_xhosa_language_model::XHOSA_TESTDATA_DIRECTORY;
use lingua_yoruba_language_model::YORUBA_TESTDATA_DIRECTORY;
use lingua_zulu_language_model::ZULU_TESTDATA_DIRECTORY;

pub(crate) fn get_test_data_directory(language: &Language) -> Dir<'static> {
    match *language {
        Language::Afrikaans => AFRIKAANS_TESTDATA_DIRECTORY,
        Language::Albanian => ALBANIAN_TESTDATA_DIRECTORY,
        Language::Arabic => ARABIC_TESTDATA_DIRECTORY,
        Language::Armenian => ARMENIAN_TESTDATA_DIRECTORY,
        Language::Azerbaijani => AZERBAIJANI_TESTDATA_DIRECTORY,
        Language::Basque => BASQUE_TESTDATA_DIRECTORY,
        Language::Belarusian => BELARUSIAN_TESTDATA_DIRECTORY,
        Language::Bengali => BENGALI_TESTDATA_DIRECTORY,
        Language::Bokmal => BOKMAL_TESTDATA_DIRECTORY,
        Language::Bosnian => BOSNIAN_TESTDATA_DIRECTORY,
        Language::Bulgarian => BULGARIAN_TESTDATA_DIRECTORY,
        Language::Catalan => CATALAN_TESTDATA_DIRECTORY,
        Language::Chinese => CHINESE_TESTDATA_DIRECTORY,
        Language::Croatian => CROATIAN_TESTDATA_DIRECTORY,
        Language::Czech => CZECH_TESTDATA_DIRECTORY,
        Language::Danish => DANISH_TESTDATA_DIRECTORY,
        Language::Dutch => DUTCH_TESTDATA_DIRECTORY,
        Language::English => ENGLISH_TESTDATA_DIRECTORY,
        Language::Esperanto => ESPERANTO_TESTDATA_DIRECTORY,
        Language::Estonian => ESTONIAN_TESTDATA_DIRECTORY,
        Language::Finnish => FINNISH_TESTDATA_DIRECTORY,
        Language::French => FRENCH_TESTDATA_DIRECTORY,
        Language::Ganda => GANDA_TESTDATA_DIRECTORY,
        Language::Georgian => GEORGIAN_TESTDATA_DIRECTORY,
        Language::German => GERMAN_TESTDATA_DIRECTORY,
        Language::Greek => GREEK_TESTDATA_DIRECTORY,
        Language::Gujarati => GUJARATI_TESTDATA_DIRECTORY,
        Language::Hebrew => HEBREW_TESTDATA_DIRECTORY,
        Language::Hindi => HINDI_TESTDATA_DIRECTORY,
        Language::Hungarian => HUNGARIAN_TESTDATA_DIRECTORY,
        Language::Icelandic => ICELANDIC_TESTDATA_DIRECTORY,
        Language::Indonesian => INDONESIAN_TESTDATA_DIRECTORY,
        Language::Irish => IRISH_TESTDATA_DIRECTORY,
        Language::Italian => ITALIAN_TESTDATA_DIRECTORY,
        Language::Japanese => JAPANESE_TESTDATA_DIRECTORY,
        Language::Kazakh => KAZAKH_TESTDATA_DIRECTORY,
        Language::Korean => KOREAN_TESTDATA_DIRECTORY,
        Language::Latin => LATIN_TESTDATA_DIRECTORY,
        Language::Latvian => LATVIAN_TESTDATA_DIRECTORY,
        Language::Lithuanian => LITHUANIAN_TESTDATA_DIRECTORY,
        Language::Macedonian => MACEDONIAN_TESTDATA_DIRECTORY,
        Language::Malay => MALAY_TESTDATA_DIRECTORY,
        Language::Maori => MAORI_TESTDATA_DIRECTORY,
        Language::Marathi => MARATHI_TESTDATA_DIRECTORY,
        Language::Mongolian => MONGOLIAN_TESTDATA_DIRECTORY,
        Language::Nynorsk => NYNORSK_TESTDATA_DIRECTORY,
        Language::Persian => PERSIAN_TESTDATA_DIRECTORY,
        Language::Polish => POLISH_TESTDATA_DIRECTORY,
        Language::Portuguese => PORTUGUESE_TESTDATA_DIRECTORY,
        Language::Punjabi => PUNJABI_TESTDATA_DIRECTORY,
        Language::Romanian => ROMANIAN_TESTDATA_DIRECTORY,
        Language::Russian => RUSSIAN_TESTDATA_DIRECTORY,
        Language::Serbian => SERBIAN_TESTDATA_DIRECTORY,
        Language::Shona => SHONA_TESTDATA_DIRECTORY,
        Language::Slovak => SLOVAK_TESTDATA_DIRECTORY,
        Language::Slovene => SLOVENE_TESTDATA_DIRECTORY,
        Language::Somali => SOMALI_TESTDATA_DIRECTORY,
        Language::Sotho => SOTHO_TESTDATA_DIRECTORY,
        Language::Spanish => SPANISH_TESTDATA_DIRECTORY,
        Language::Swahili => SWAHILI_TESTDATA_DIRECTORY,
        Language::Swedish => SWEDISH_TESTDATA_DIRECTORY,
        Language::Tagalog => TAGALOG_TESTDATA_DIRECTORY,
        Language::Tamil => TAMIL_TESTDATA_DIRECTORY,
        Language::Telugu => TELUGU_TESTDATA_DIRECTORY,
        Language::Thai => THAI_TESTDATA_DIRECTORY,
        Language::Tsonga => TSONGA_TESTDATA_DIRECTORY,
        Language::Tswana => TSWANA_TESTDATA_DIRECTORY,
        Language::Turkish => TURKISH_TESTDATA_DIRECTORY,
        Language::Ukrainian => UKRAINIAN_TESTDATA_DIRECTORY,
        Language::Urdu => URDU_TESTDATA_DIRECTORY,
        Language::Venetian => VENETIAN_TESTDATA_DIRECTORY,
        Language::Vietnamese => VIETNAMESE_TESTDATA_DIRECTORY,
        Language::Welsh => WELSH_TESTDATA_DIRECTORY,
        Language::Xhosa => XHOSA_TESTDATA_DIRECTORY,
        Language::Yoruba => YORUBA_TESTDATA_DIRECTORY,
        Language::Zulu => ZULU_TESTDATA_DIRECTORY,
    }
}
