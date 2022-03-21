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

use crate::Language;

pub(crate) fn load_rkyv(language: Language, ngram_length: usize) -> &'static [u8] {
    let ngrams = get_language_models_ngrams(language);

    ngrams[ngram_length - 1]
}

fn get_language_models_ngrams(language: Language) -> [&'static [u8]; 5] {
    match language {
        #[cfg(feature = "afrikaans")]
        Language::Afrikaans => lingua_afrikaans_language_model::NGRAMS,

        #[cfg(feature = "albanian")]
        Language::Albanian => lingua_albanian_language_model::NGRAMS,

        #[cfg(feature = "arabic")]
        Language::Arabic => lingua_arabic_language_model::NGRAMS,

        #[cfg(feature = "armenian")]
        Language::Armenian => lingua_armenian_language_model::NGRAMS,

        #[cfg(feature = "azerbaijani")]
        Language::Azerbaijani => lingua_azerbaijani_language_model::NGRAMS,

        #[cfg(feature = "basque")]
        Language::Basque => lingua_basque_language_model::NGRAMS,

        #[cfg(feature = "belarusian")]
        Language::Belarusian => lingua_belarusian_language_model::NGRAMS,

        #[cfg(feature = "bengali")]
        Language::Bengali => lingua_bengali_language_model::NGRAMS,

        #[cfg(feature = "bokmal")]
        Language::Bokmal => lingua_bokmal_language_model::NGRAMS,

        #[cfg(feature = "bosnian")]
        Language::Bosnian => lingua_bosnian_language_model::NGRAMS,

        #[cfg(feature = "bulgarian")]
        Language::Bulgarian => lingua_bulgarian_language_model::NGRAMS,

        #[cfg(feature = "catalan")]
        Language::Catalan => lingua_catalan_language_model::NGRAMS,

        #[cfg(feature = "chinese")]
        Language::Chinese => lingua_chinese_language_model::NGRAMS,

        #[cfg(feature = "croatian")]
        Language::Croatian => lingua_croatian_language_model::NGRAMS,

        #[cfg(feature = "czech")]
        Language::Czech => lingua_czech_language_model::NGRAMS,

        #[cfg(feature = "danish")]
        Language::Danish => lingua_danish_language_model::NGRAMS,

        #[cfg(feature = "dutch")]
        Language::Dutch => lingua_dutch_language_model::NGRAMS,

        #[cfg(feature = "english")]
        Language::English => lingua_english_language_model::NGRAMS,

        #[cfg(feature = "esperanto")]
        Language::Esperanto => lingua_esperanto_language_model::NGRAMS,

        #[cfg(feature = "estonian")]
        Language::Estonian => lingua_estonian_language_model::NGRAMS,

        #[cfg(feature = "finnish")]
        Language::Finnish => lingua_finnish_language_model::NGRAMS,

        #[cfg(feature = "french")]
        Language::French => lingua_french_language_model::NGRAMS,

        #[cfg(feature = "ganda")]
        Language::Ganda => lingua_ganda_language_model::NGRAMS,

        #[cfg(feature = "georgian")]
        Language::Georgian => lingua_georgian_language_model::NGRAMS,

        #[cfg(feature = "german")]
        Language::German => lingua_german_language_model::NGRAMS,

        #[cfg(feature = "greek")]
        Language::Greek => lingua_greek_language_model::NGRAMS,

        #[cfg(feature = "gujarati")]
        Language::Gujarati => lingua_gujarati_language_model::NGRAMS,

        #[cfg(feature = "hebrew")]
        Language::Hebrew => lingua_hebrew_language_model::NGRAMS,

        #[cfg(feature = "hindi")]
        Language::Hindi => lingua_hindi_language_model::NGRAMS,

        #[cfg(feature = "hungarian")]
        Language::Hungarian => lingua_hungarian_language_model::NGRAMS,

        #[cfg(feature = "icelandic")]
        Language::Icelandic => lingua_icelandic_language_model::NGRAMS,

        #[cfg(feature = "indonesian")]
        Language::Indonesian => lingua_indonesian_language_model::NGRAMS,

        #[cfg(feature = "irish")]
        Language::Irish => lingua_irish_language_model::NGRAMS,

        #[cfg(feature = "italian")]
        Language::Italian => lingua_italian_language_model::NGRAMS,

        #[cfg(feature = "japanese")]
        Language::Japanese => lingua_japanese_language_model::NGRAMS,

        #[cfg(feature = "kazakh")]
        Language::Kazakh => lingua_kazakh_language_model::NGRAMS,

        #[cfg(feature = "korean")]
        Language::Korean => lingua_korean_language_model::NGRAMS,

        #[cfg(feature = "latin")]
        Language::Latin => lingua_latin_language_model::NGRAMS,

        #[cfg(feature = "latvian")]
        Language::Latvian => lingua_latvian_language_model::NGRAMS,

        #[cfg(feature = "lithuanian")]
        Language::Lithuanian => lingua_lithuanian_language_model::NGRAMS,

        #[cfg(feature = "macedonian")]
        Language::Macedonian => lingua_macedonian_language_model::NGRAMS,

        #[cfg(feature = "malay")]
        Language::Malay => lingua_malay_language_model::NGRAMS,

        #[cfg(feature = "maori")]
        Language::Maori => lingua_maori_language_model::NGRAMS,

        #[cfg(feature = "marathi")]
        Language::Marathi => lingua_marathi_language_model::NGRAMS,

        #[cfg(feature = "mongolian")]
        Language::Mongolian => lingua_mongolian_language_model::NGRAMS,

        #[cfg(feature = "nynorsk")]
        Language::Nynorsk => lingua_nynorsk_language_model::NGRAMS,

        #[cfg(feature = "persian")]
        Language::Persian => lingua_persian_language_model::NGRAMS,

        #[cfg(feature = "polish")]
        Language::Polish => lingua_polish_language_model::NGRAMS,

        #[cfg(feature = "portuguese")]
        Language::Portuguese => lingua_portuguese_language_model::NGRAMS,

        #[cfg(feature = "punjabi")]
        Language::Punjabi => lingua_punjabi_language_model::NGRAMS,

        #[cfg(feature = "romanian")]
        Language::Romanian => lingua_romanian_language_model::NGRAMS,

        #[cfg(feature = "russian")]
        Language::Russian => lingua_russian_language_model::NGRAMS,

        #[cfg(feature = "serbian")]
        Language::Serbian => lingua_serbian_language_model::NGRAMS,

        #[cfg(feature = "shona")]
        Language::Shona => lingua_shona_language_model::NGRAMS,

        #[cfg(feature = "slovak")]
        Language::Slovak => lingua_slovak_language_model::NGRAMS,

        #[cfg(feature = "slovene")]
        Language::Slovene => lingua_slovene_language_model::NGRAMS,

        #[cfg(feature = "somali")]
        Language::Somali => lingua_somali_language_model::NGRAMS,

        #[cfg(feature = "sotho")]
        Language::Sotho => lingua_sotho_language_model::NGRAMS,

        #[cfg(feature = "spanish")]
        Language::Spanish => lingua_spanish_language_model::NGRAMS,

        #[cfg(feature = "swahili")]
        Language::Swahili => lingua_swahili_language_model::NGRAMS,

        #[cfg(feature = "swedish")]
        Language::Swedish => lingua_swedish_language_model::NGRAMS,

        #[cfg(feature = "tagalog")]
        Language::Tagalog => lingua_tagalog_language_model::NGRAMS,

        #[cfg(feature = "tamil")]
        Language::Tamil => lingua_tamil_language_model::NGRAMS,

        #[cfg(feature = "telugu")]
        Language::Telugu => lingua_telugu_language_model::NGRAMS,

        #[cfg(feature = "thai")]
        Language::Thai => lingua_thai_language_model::NGRAMS,

        #[cfg(feature = "tsonga")]
        Language::Tsonga => lingua_tsonga_language_model::NGRAMS,

        #[cfg(feature = "tswana")]
        Language::Tswana => lingua_tswana_language_model::NGRAMS,

        #[cfg(feature = "turkish")]
        Language::Turkish => lingua_turkish_language_model::NGRAMS,

        #[cfg(feature = "ukrainian")]
        Language::Ukrainian => lingua_ukrainian_language_model::NGRAMS,

        #[cfg(feature = "urdu")]
        Language::Urdu => lingua_urdu_language_model::NGRAMS,

        #[cfg(feature = "vietnamese")]
        Language::Vietnamese => lingua_vietnamese_language_model::NGRAMS,

        #[cfg(feature = "welsh")]
        Language::Welsh => lingua_welsh_language_model::NGRAMS,

        #[cfg(feature = "xhosa")]
        Language::Xhosa => lingua_xhosa_language_model::NGRAMS,

        #[cfg(feature = "yoruba")]
        Language::Yoruba => lingua_yoruba_language_model::NGRAMS,

        #[cfg(feature = "zulu")]
        Language::Zulu => lingua_zulu_language_model::NGRAMS,
    }
}

#[cfg(test)]
mod tests {
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
}
