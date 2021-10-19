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

use crate::ngram::Ngram;
use crate::Language;
use include_dir::Dir;
use lingua_afrikaans_language_model::AFRIKAANS_MODELS_DIRECTORY;
use lingua_albanian_language_model::ALBANIAN_MODELS_DIRECTORY;
use lingua_arabic_language_model::ARABIC_MODELS_DIRECTORY;
use lingua_armenian_language_model::ARMENIAN_MODELS_DIRECTORY;
use lingua_azerbaijani_language_model::AZERBAIJANI_MODELS_DIRECTORY;
use lingua_basque_language_model::BASQUE_MODELS_DIRECTORY;
use lingua_belarusian_language_model::BELARUSIAN_MODELS_DIRECTORY;
use lingua_bengali_language_model::BENGALI_MODELS_DIRECTORY;
use lingua_bokmal_language_model::BOKMAL_MODELS_DIRECTORY;
use lingua_bosnian_language_model::BOSNIAN_MODELS_DIRECTORY;
use lingua_bulgarian_language_model::BULGARIAN_MODELS_DIRECTORY;
use lingua_catalan_language_model::CATALAN_MODELS_DIRECTORY;
use lingua_chinese_language_model::CHINESE_MODELS_DIRECTORY;
use lingua_croatian_language_model::CROATIAN_MODELS_DIRECTORY;
use lingua_czech_language_model::CZECH_MODELS_DIRECTORY;
use lingua_danish_language_model::DANISH_MODELS_DIRECTORY;
use lingua_dutch_language_model::DUTCH_MODELS_DIRECTORY;
use lingua_english_language_model::ENGLISH_MODELS_DIRECTORY;
use lingua_esperanto_language_model::ESPERANTO_MODELS_DIRECTORY;
use lingua_estonian_language_model::ESTONIAN_MODELS_DIRECTORY;
use lingua_finnish_language_model::FINNISH_MODELS_DIRECTORY;
use lingua_french_language_model::FRENCH_MODELS_DIRECTORY;
use lingua_ganda_language_model::GANDA_MODELS_DIRECTORY;
use lingua_georgian_language_model::GEORGIAN_MODELS_DIRECTORY;
use lingua_german_language_model::GERMAN_MODELS_DIRECTORY;
use lingua_greek_language_model::GREEK_MODELS_DIRECTORY;
use lingua_gujarati_language_model::GUJARATI_MODELS_DIRECTORY;
use lingua_hebrew_language_model::HEBREW_MODELS_DIRECTORY;
use lingua_hindi_language_model::HINDI_MODELS_DIRECTORY;
use lingua_hungarian_language_model::HUNGARIAN_MODELS_DIRECTORY;
use lingua_icelandic_language_model::ICELANDIC_MODELS_DIRECTORY;
use lingua_indonesian_language_model::INDONESIAN_MODELS_DIRECTORY;
use lingua_irish_language_model::IRISH_MODELS_DIRECTORY;
use lingua_italian_language_model::ITALIAN_MODELS_DIRECTORY;
use lingua_japanese_language_model::JAPANESE_MODELS_DIRECTORY;
use lingua_kazakh_language_model::KAZAKH_MODELS_DIRECTORY;
use lingua_korean_language_model::KOREAN_MODELS_DIRECTORY;
use lingua_latin_language_model::LATIN_MODELS_DIRECTORY;
use lingua_latvian_language_model::LATVIAN_MODELS_DIRECTORY;
use lingua_lithuanian_language_model::LITHUANIAN_MODELS_DIRECTORY;
use lingua_macedonian_language_model::MACEDONIAN_MODELS_DIRECTORY;
use lingua_malay_language_model::MALAY_MODELS_DIRECTORY;
use lingua_maori_language_model::MAORI_MODELS_DIRECTORY;
use lingua_marathi_language_model::MARATHI_MODELS_DIRECTORY;
use lingua_mongolian_language_model::MONGOLIAN_MODELS_DIRECTORY;
use lingua_nynorsk_language_model::NYNORSK_MODELS_DIRECTORY;
use lingua_persian_language_model::PERSIAN_MODELS_DIRECTORY;
use lingua_polish_language_model::POLISH_MODELS_DIRECTORY;
use lingua_portuguese_language_model::PORTUGUESE_MODELS_DIRECTORY;
use lingua_punjabi_language_model::PUNJABI_MODELS_DIRECTORY;
use lingua_romanian_language_model::ROMANIAN_MODELS_DIRECTORY;
use lingua_russian_language_model::RUSSIAN_MODELS_DIRECTORY;
use lingua_serbian_language_model::SERBIAN_MODELS_DIRECTORY;
use lingua_shona_language_model::SHONA_MODELS_DIRECTORY;
use lingua_slovak_language_model::SLOVAK_MODELS_DIRECTORY;
use lingua_slovene_language_model::SLOVENE_MODELS_DIRECTORY;
use lingua_somali_language_model::SOMALI_MODELS_DIRECTORY;
use lingua_sotho_language_model::SOTHO_MODELS_DIRECTORY;
use lingua_spanish_language_model::SPANISH_MODELS_DIRECTORY;
use lingua_swahili_language_model::SWAHILI_MODELS_DIRECTORY;
use lingua_swedish_language_model::SWEDISH_MODELS_DIRECTORY;
use lingua_tagalog_language_model::TAGALOG_MODELS_DIRECTORY;
use lingua_tamil_language_model::TAMIL_MODELS_DIRECTORY;
use lingua_telugu_language_model::TELUGU_MODELS_DIRECTORY;
use lingua_thai_language_model::THAI_MODELS_DIRECTORY;
use lingua_tsonga_language_model::TSONGA_MODELS_DIRECTORY;
use lingua_tswana_language_model::TSWANA_MODELS_DIRECTORY;
use lingua_turkish_language_model::TURKISH_MODELS_DIRECTORY;
use lingua_ukrainian_language_model::UKRAINIAN_MODELS_DIRECTORY;
use lingua_urdu_language_model::URDU_MODELS_DIRECTORY;
use lingua_vietnamese_language_model::VIETNAMESE_MODELS_DIRECTORY;
use lingua_welsh_language_model::WELSH_MODELS_DIRECTORY;
use lingua_xhosa_language_model::XHOSA_MODELS_DIRECTORY;
use lingua_yoruba_language_model::YORUBA_MODELS_DIRECTORY;
use lingua_zulu_language_model::ZULU_MODELS_DIRECTORY;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub(crate) fn load_json(language: Language, ngram_length: usize) -> std::io::Result<String> {
    let ngram_name = Ngram::find_ngram_name_by_length(ngram_length);
    let file_path = format!("{}s.json.zip", ngram_name);
    let directory = get_language_models_directory(language);
    let zip_file = directory.get_file(file_path).unwrap();
    let zip_file_reader = Cursor::new(zip_file.contents());
    let mut archive = ZipArchive::new(zip_file_reader).unwrap();
    let mut json_file = archive.by_index(0).unwrap();
    let mut json = String::new();
    json_file.read_to_string(&mut json)?;
    Ok(json)
}

fn get_language_models_directory(language: Language) -> Dir<'static> {
    match language {
        Language::Afrikaans => AFRIKAANS_MODELS_DIRECTORY,
        Language::Albanian => ALBANIAN_MODELS_DIRECTORY,
        Language::Arabic => ARABIC_MODELS_DIRECTORY,
        Language::Armenian => ARMENIAN_MODELS_DIRECTORY,
        Language::Azerbaijani => AZERBAIJANI_MODELS_DIRECTORY,
        Language::Basque => BASQUE_MODELS_DIRECTORY,
        Language::Belarusian => BELARUSIAN_MODELS_DIRECTORY,
        Language::Bengali => BENGALI_MODELS_DIRECTORY,
        Language::Bokmal => BOKMAL_MODELS_DIRECTORY,
        Language::Bosnian => BOSNIAN_MODELS_DIRECTORY,
        Language::Bulgarian => BULGARIAN_MODELS_DIRECTORY,
        Language::Catalan => CATALAN_MODELS_DIRECTORY,
        Language::Chinese => CHINESE_MODELS_DIRECTORY,
        Language::Croatian => CROATIAN_MODELS_DIRECTORY,
        Language::Czech => CZECH_MODELS_DIRECTORY,
        Language::Danish => DANISH_MODELS_DIRECTORY,
        Language::Dutch => DUTCH_MODELS_DIRECTORY,
        Language::English => ENGLISH_MODELS_DIRECTORY,
        Language::Esperanto => ESPERANTO_MODELS_DIRECTORY,
        Language::Estonian => ESTONIAN_MODELS_DIRECTORY,
        Language::Finnish => FINNISH_MODELS_DIRECTORY,
        Language::French => FRENCH_MODELS_DIRECTORY,
        Language::Ganda => GANDA_MODELS_DIRECTORY,
        Language::Georgian => GEORGIAN_MODELS_DIRECTORY,
        Language::German => GERMAN_MODELS_DIRECTORY,
        Language::Greek => GREEK_MODELS_DIRECTORY,
        Language::Gujarati => GUJARATI_MODELS_DIRECTORY,
        Language::Hebrew => HEBREW_MODELS_DIRECTORY,
        Language::Hindi => HINDI_MODELS_DIRECTORY,
        Language::Hungarian => HUNGARIAN_MODELS_DIRECTORY,
        Language::Icelandic => ICELANDIC_MODELS_DIRECTORY,
        Language::Indonesian => INDONESIAN_MODELS_DIRECTORY,
        Language::Irish => IRISH_MODELS_DIRECTORY,
        Language::Italian => ITALIAN_MODELS_DIRECTORY,
        Language::Japanese => JAPANESE_MODELS_DIRECTORY,
        Language::Kazakh => KAZAKH_MODELS_DIRECTORY,
        Language::Korean => KOREAN_MODELS_DIRECTORY,
        Language::Latin => LATIN_MODELS_DIRECTORY,
        Language::Latvian => LATVIAN_MODELS_DIRECTORY,
        Language::Lithuanian => LITHUANIAN_MODELS_DIRECTORY,
        Language::Macedonian => MACEDONIAN_MODELS_DIRECTORY,
        Language::Malay => MALAY_MODELS_DIRECTORY,
        Language::Maori => MAORI_MODELS_DIRECTORY,
        Language::Marathi => MARATHI_MODELS_DIRECTORY,
        Language::Mongolian => MONGOLIAN_MODELS_DIRECTORY,
        Language::Nynorsk => NYNORSK_MODELS_DIRECTORY,
        Language::Persian => PERSIAN_MODELS_DIRECTORY,
        Language::Polish => POLISH_MODELS_DIRECTORY,
        Language::Portuguese => PORTUGUESE_MODELS_DIRECTORY,
        Language::Punjabi => PUNJABI_MODELS_DIRECTORY,
        Language::Romanian => ROMANIAN_MODELS_DIRECTORY,
        Language::Russian => RUSSIAN_MODELS_DIRECTORY,
        Language::Serbian => SERBIAN_MODELS_DIRECTORY,
        Language::Shona => SHONA_MODELS_DIRECTORY,
        Language::Slovak => SLOVAK_MODELS_DIRECTORY,
        Language::Slovene => SLOVENE_MODELS_DIRECTORY,
        Language::Somali => SOMALI_MODELS_DIRECTORY,
        Language::Sotho => SOTHO_MODELS_DIRECTORY,
        Language::Spanish => SPANISH_MODELS_DIRECTORY,
        Language::Swahili => SWAHILI_MODELS_DIRECTORY,
        Language::Swedish => SWEDISH_MODELS_DIRECTORY,
        Language::Tagalog => TAGALOG_MODELS_DIRECTORY,
        Language::Tamil => TAMIL_MODELS_DIRECTORY,
        Language::Telugu => TELUGU_MODELS_DIRECTORY,
        Language::Thai => THAI_MODELS_DIRECTORY,
        Language::Tsonga => TSONGA_MODELS_DIRECTORY,
        Language::Tswana => TSWANA_MODELS_DIRECTORY,
        Language::Turkish => TURKISH_MODELS_DIRECTORY,
        Language::Ukrainian => UKRAINIAN_MODELS_DIRECTORY,
        Language::Urdu => URDU_MODELS_DIRECTORY,
        Language::Vietnamese => VIETNAMESE_MODELS_DIRECTORY,
        Language::Welsh => WELSH_MODELS_DIRECTORY,
        Language::Xhosa => XHOSA_MODELS_DIRECTORY,
        Language::Yoruba => YORUBA_MODELS_DIRECTORY,
        Language::Zulu => ZULU_MODELS_DIRECTORY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minify;

    const EXPECTED_UNIGRAM_MODEL: &str = r#"
    {
        "language":"ENGLISH",
        "ngrams":{
            "2/93616591":"ﬀ ċ ė ĩ ȼ ɔ ţ ũ ʔ ơ ả ộ ù",
            "36/93616591":"ā",
            "16/93616591":"ﬁ",
            "7/93616591":"ă ệ",
            "5/93616591":"ą ħ ś",
            "26/93616591":"ć",
            "49/93616591":"č",
            "8/93616591":"đ ě ź",
            "1/93616591":"ē ț ġ ḵ ņ ɑ ə ɛ ɦ ű ƅ ạ ƴ ặ ế ỉ ờ ủ ứ",
            "4/93616591":"ș ÿ",
            "9/93616591":"ę ż",
            "40/93616591":"ğ",
            "13/93616591":"ī ß",
            "31/93616591":"ı",
            "39/93616591":"ł",
            "25/93616591":"ń",
            "3/93616591":"ň ｍ ů ư ị",
            "10/93616591":"ō",
            "60/93616591":"œ",
            "11/93616591":"ř ì",
            "18/93616591":"ş",
            "52/93616591":"š ô",
            "7915445/93616591":"a",
            "1461095/93616591":"b",
            "3003229/93616591":"c",
            "3622548/93616591":"d",
            "11308892/93616591":"e",
            "2006896/93616591":"f",
            "1963483/93616591":"g",
            "234603/4927189":"h",
            "6800966/93616591":"i",
            "207477/93616591":"j",
            "14/93616591":"ū û",
            "760186/93616591":"k",
            "3928800/93616591":"l",
            "2358339/93616591":"m",
            "6698842/93616591":"n",
            "7137868/93616591":"o",
            "1994813/93616591":"p",
            "82818/93616591":"q",
            "5939665/93616591":"r",
            "6234570/93616591":"s",
            "8431167/93616591":"t",
            "2559048/93616591":"u",
            "1024914/93616591":"v",
            "1751793/93616591":"w",
            "172448/93616591":"x",
            "1683314/93616591":"y",
            "103267/93616591":"z",
            "20/93616591":"ž",
            "37/93616591":"º ë",
            "4/4927189":"à",
            "539/93616591":"á",
            "913/93616591":"â",
            "28/93616591":"ã",
            "118/93616591":"ä",
            "42/93616591":"å",
            "6/93616591":"æ",
            "126/93616591":"ç",
            "136/93616591":"è",
            "2259/93616591":"é",
            "45/93616591":"ê",
            "428/93616591":"í",
            "1/4927189":"î",
            "77/93616591":"ï",
            "21/93616591":"ð",
            "478/93616591":"ñ",
            "48/93616591":"ò",
            "490/93616591":"ó",
            "93/93616591":"õ",
            "200/93616591":"ö",
            "32/93616591":"ø",
            "142/93616591":"ú",
            "149/93616591":"ü",
            "23/93616591":"ý"
        }
    }
    "#;

    #[test]
    fn test_load_json() {
        let result = load_json(Language::English, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), minify(EXPECTED_UNIGRAM_MODEL));
    }
}
