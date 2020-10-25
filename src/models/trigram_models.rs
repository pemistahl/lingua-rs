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

use crate::constant::LANGUAGE_MODELS_DIRECTORY;
use crate::models::{
    load_json, LanguageToNgramsMappingCell, LazyLanguageToNgramsMapping,
    LazyTrainingDataLanguageModel,
};
use crate::Language;
use crate::Language::*;
use cfg_if::cfg_if;
use once_cell::sync::OnceCell;

cfg_if! {
    if #[cfg(test)] {
        use crate::model::MockTrainingDataLanguageModel as TrainingDataLanguageModel;
    } else {
        use crate::model::TrainingDataLanguageModel;
    }
}

pub(crate) fn trigram_models() -> LazyLanguageToNgramsMapping {
    static TRIGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    TRIGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_trigram_model(),
            Albanian => albanian_trigram_model(),
            Arabic => arabic_trigram_model(),
            Armenian => armenian_trigram_model(),
            Azerbaijani => azerbaijani_trigram_model(),
            Basque => basque_trigram_model(),
            Belarusian => belarusian_trigram_model(),
            Bengali => bengali_trigram_model(),
            Bokmal => bokmal_trigram_model(),
            Bosnian => bosnian_trigram_model(),
            Bulgarian => bulgarian_trigram_model(),
            Catalan => catalan_trigram_model(),
            Chinese => chinese_trigram_model(),
            Croatian => croatian_trigram_model(),
            Czech => czech_trigram_model(),
            Danish => danish_trigram_model(),
            Dutch => dutch_trigram_model(),
            English => english_trigram_model(),
            Esperanto => esperanto_trigram_model(),
            Estonian => estonian_trigram_model(),
            Finnish => finnish_trigram_model(),
            French => french_trigram_model(),
            Ganda => ganda_trigram_model(),
            Georgian => georgian_trigram_model(),
            German => german_trigram_model(),
            Greek => greek_trigram_model(),
            Gujarati => gujarati_trigram_model(),
            Hebrew => hebrew_trigram_model(),
            Hindi => hindi_trigram_model(),
            Hungarian => hungarian_trigram_model(),
            Icelandic => icelandic_trigram_model(),
            Indonesian => indonesian_trigram_model(),
            Irish => irish_trigram_model(),
            Italian => italian_trigram_model(),
            Japanese => japanese_trigram_model(),
            Kazakh => kazakh_trigram_model(),
            Korean => korean_trigram_model(),
            Latin => latin_trigram_model(),
            Latvian => latvian_trigram_model(),
            Lithuanian => lithuanian_trigram_model(),
            Macedonian => macedonian_trigram_model(),
            Malay => malay_trigram_model(),
            Marathi => marathi_trigram_model(),
            Mongolian => mongolian_trigram_model(),
            Nynorsk => nynorsk_trigram_model(),
            Persian => persian_trigram_model(),
            Polish => polish_trigram_model(),
            Portuguese => portuguese_trigram_model(),
            Punjabi => punjabi_trigram_model(),
            Romanian => romanian_trigram_model(),
            Russian => russian_trigram_model(),
            Serbian => serbian_trigram_model(),
            Shona => shona_trigram_model(),
            Slovak => slovak_trigram_model(),
            Slovene => slovene_trigram_model(),
            Somali => somali_trigram_model(),
            Sotho => sotho_trigram_model(),
            Spanish => spanish_trigram_model(),
            Swahili => swahili_trigram_model(),
            Swedish => swedish_trigram_model(),
            Tagalog => tagalog_trigram_model(),
            Tamil => tamil_trigram_model(),
            Telugu => telugu_trigram_model(),
            Thai => thai_trigram_model(),
            Tsonga => tsonga_trigram_model(),
            Tswana => tswana_trigram_model(),
            Turkish => turkish_trigram_model(),
            Ukrainian => ukrainian_trigram_model(),
            Urdu => urdu_trigram_model(),
            Vietnamese => vietnamese_trigram_model(),
            Welsh => welsh_trigram_model(),
            Xhosa => xhosa_trigram_model(),
            Yoruba => yoruba_trigram_model(),
            Zulu => zulu_trigram_model()
        )
    })
}

fn afrikaans_trigram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AFRIKAANS_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Afrikaans);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn albanian_trigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ALBANIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Albanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn arabic_trigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARABIC_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Arabic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn armenian_trigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARMENIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Armenian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn azerbaijani_trigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AZERBAIJANI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Azerbaijani);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn basque_trigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BASQUE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Basque);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn belarusian_trigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BELARUSIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Belarusian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bengali_trigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BENGALI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Bengali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bokmal_trigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOKMAL_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Bokmal);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bosnian_trigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOSNIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Bosnian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bulgarian_trigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BULGARIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Bulgarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn catalan_trigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CATALAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Catalan);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn chinese_trigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CHINESE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Chinese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn croatian_trigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CROATIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Croatian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn czech_trigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CZECH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Czech);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn danish_trigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DANISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Danish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn dutch_trigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DUTCH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Dutch);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn english_trigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ENGLISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(English);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn esperanto_trigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESPERANTO_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Esperanto);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn estonian_trigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESTONIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Estonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn finnish_trigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FINNISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Finnish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn french_trigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FRENCH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(French);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ganda_trigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GANDA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Ganda);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn georgian_trigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GEORGIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Georgian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn german_trigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GERMAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(German);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn greek_trigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GREEK_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Greek);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn gujarati_trigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GUJARATI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Gujarati);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hebrew_trigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HEBREW_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Hebrew);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hindi_trigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HINDI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Hindi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hungarian_trigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HUNGARIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Hungarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn icelandic_trigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ICELANDIC_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Icelandic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn indonesian_trigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    INDONESIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Indonesian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn irish_trigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    IRISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Irish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn italian_trigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ITALIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Italian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn japanese_trigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    JAPANESE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Japanese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn kazakh_trigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KAZAKH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Kazakh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn korean_trigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KOREAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Korean);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latin_trigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATIN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Latin);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latvian_trigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATVIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Latvian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn lithuanian_trigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LITHUANIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Lithuanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn macedonian_trigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MACEDONIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Macedonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn malay_trigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MALAY_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Malay);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn marathi_trigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MARATHI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Marathi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn mongolian_trigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MONGOLIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Mongolian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn nynorsk_trigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    NYNORSK_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Nynorsk);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn persian_trigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PERSIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Persian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn polish_trigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    POLISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Polish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn portuguese_trigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PORTUGUESE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Portuguese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn punjabi_trigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PUNJABI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Punjabi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn romanian_trigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ROMANIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Romanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn russian_trigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    RUSSIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Russian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn serbian_trigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SERBIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Serbian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn shona_trigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SHONA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Shona);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovak_trigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVAK_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Slovak);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovene_trigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVENE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Slovene);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn somali_trigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOMALI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Somali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn sotho_trigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOTHO_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Sotho);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn spanish_trigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SPANISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Spanish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swahili_trigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWAHILI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Swahili);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swedish_trigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWEDISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Swedish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tagalog_trigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAGALOG_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Tagalog);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tamil_trigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAMIL_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Tamil);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn telugu_trigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TELUGU_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Telugu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn thai_trigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    THAI_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Thai);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tsonga_trigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSONGA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Tsonga);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tswana_trigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSWANA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Tswana);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn turkish_trigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TURKISH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Turkish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ukrainian_trigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    UKRAINIAN_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Ukrainian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn urdu_trigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    URDU_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Urdu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn vietnamese_trigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    VIETNAMESE_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Vietnamese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn welsh_trigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    WELSH_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Welsh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn xhosa_trigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    XHOSA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Xhosa);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn yoruba_trigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    YORUBA_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Yoruba);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn zulu_trigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_TRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ZULU_TRIGRAM_MODEL.get_or_init(|| {
        let json = load_trigrams(Zulu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn load_trigrams(language: Language) -> String {
    load_json(LANGUAGE_MODELS_DIRECTORY, language, 3).unwrap()
}
