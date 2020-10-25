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

pub(crate) fn quadrigram_models() -> LazyLanguageToNgramsMapping {
    static QUADRIGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    QUADRIGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_quadrigram_model(),
            Albanian => albanian_quadrigram_model(),
            Arabic => arabic_quadrigram_model(),
            Armenian => armenian_quadrigram_model(),
            Azerbaijani => azerbaijani_quadrigram_model(),
            Basque => basque_quadrigram_model(),
            Belarusian => belarusian_quadrigram_model(),
            Bengali => bengali_quadrigram_model(),
            Bokmal => bokmal_quadrigram_model(),
            Bosnian => bosnian_quadrigram_model(),
            Bulgarian => bulgarian_quadrigram_model(),
            Catalan => catalan_quadrigram_model(),
            Chinese => chinese_quadrigram_model(),
            Croatian => croatian_quadrigram_model(),
            Czech => czech_quadrigram_model(),
            Danish => danish_quadrigram_model(),
            Dutch => dutch_quadrigram_model(),
            English => english_quadrigram_model(),
            Esperanto => esperanto_quadrigram_model(),
            Estonian => estonian_quadrigram_model(),
            Finnish => finnish_quadrigram_model(),
            French => french_quadrigram_model(),
            Ganda => ganda_quadrigram_model(),
            Georgian => georgian_quadrigram_model(),
            German => german_quadrigram_model(),
            Greek => greek_quadrigram_model(),
            Gujarati => gujarati_quadrigram_model(),
            Hebrew => hebrew_quadrigram_model(),
            Hindi => hindi_quadrigram_model(),
            Hungarian => hungarian_quadrigram_model(),
            Icelandic => icelandic_quadrigram_model(),
            Indonesian => indonesian_quadrigram_model(),
            Irish => irish_quadrigram_model(),
            Italian => italian_quadrigram_model(),
            Japanese => japanese_quadrigram_model(),
            Kazakh => kazakh_quadrigram_model(),
            Korean => korean_quadrigram_model(),
            Latin => latin_quadrigram_model(),
            Latvian => latvian_quadrigram_model(),
            Lithuanian => lithuanian_quadrigram_model(),
            Macedonian => macedonian_quadrigram_model(),
            Malay => malay_quadrigram_model(),
            Marathi => marathi_quadrigram_model(),
            Mongolian => mongolian_quadrigram_model(),
            Nynorsk => nynorsk_quadrigram_model(),
            Persian => persian_quadrigram_model(),
            Polish => polish_quadrigram_model(),
            Portuguese => portuguese_quadrigram_model(),
            Punjabi => punjabi_quadrigram_model(),
            Romanian => romanian_quadrigram_model(),
            Russian => russian_quadrigram_model(),
            Serbian => serbian_quadrigram_model(),
            Shona => shona_quadrigram_model(),
            Slovak => slovak_quadrigram_model(),
            Slovene => slovene_quadrigram_model(),
            Somali => somali_quadrigram_model(),
            Sotho => sotho_quadrigram_model(),
            Spanish => spanish_quadrigram_model(),
            Swahili => swahili_quadrigram_model(),
            Swedish => swedish_quadrigram_model(),
            Tagalog => tagalog_quadrigram_model(),
            Tamil => tamil_quadrigram_model(),
            Telugu => telugu_quadrigram_model(),
            Thai => thai_quadrigram_model(),
            Tsonga => tsonga_quadrigram_model(),
            Tswana => tswana_quadrigram_model(),
            Turkish => turkish_quadrigram_model(),
            Ukrainian => ukrainian_quadrigram_model(),
            Urdu => urdu_quadrigram_model(),
            Vietnamese => vietnamese_quadrigram_model(),
            Welsh => welsh_quadrigram_model(),
            Xhosa => xhosa_quadrigram_model(),
            Yoruba => yoruba_quadrigram_model(),
            Zulu => zulu_quadrigram_model()
        )
    })
}

fn afrikaans_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AFRIKAANS_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Afrikaans);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn albanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ALBANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Albanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn arabic_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARABIC_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Arabic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn armenian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARMENIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Armenian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn azerbaijani_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AZERBAIJANI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Azerbaijani);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn basque_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BASQUE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Basque);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn belarusian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BELARUSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Belarusian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bengali_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BENGALI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bengali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bokmal_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOKMAL_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bokmal);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bosnian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOSNIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bosnian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bulgarian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BULGARIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bulgarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn catalan_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CATALAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Catalan);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn chinese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CHINESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Chinese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn croatian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CROATIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Croatian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn czech_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CZECH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Czech);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn danish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DANISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Danish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn dutch_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DUTCH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Dutch);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn english_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ENGLISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(English);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn esperanto_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESPERANTO_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Esperanto);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn estonian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESTONIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Estonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn finnish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FINNISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Finnish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn french_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FRENCH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(French);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ganda_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GANDA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Ganda);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn georgian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GEORGIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Georgian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn german_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GERMAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(German);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn greek_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GREEK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Greek);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn gujarati_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GUJARATI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Gujarati);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hebrew_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HEBREW_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hebrew);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hindi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HINDI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hindi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hungarian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HUNGARIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hungarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn icelandic_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ICELANDIC_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Icelandic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn indonesian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    INDONESIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Indonesian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn irish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    IRISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Irish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn italian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ITALIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Italian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn japanese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    JAPANESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Japanese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn kazakh_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KAZAKH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Kazakh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn korean_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KOREAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Korean);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latin_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATIN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Latin);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latvian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATVIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Latvian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn lithuanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LITHUANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Lithuanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn macedonian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MACEDONIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Macedonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn malay_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MALAY_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Malay);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn marathi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MARATHI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Marathi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn mongolian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MONGOLIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Mongolian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn nynorsk_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    NYNORSK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Nynorsk);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn persian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PERSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Persian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn polish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    POLISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Polish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn portuguese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PORTUGUESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Portuguese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn punjabi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PUNJABI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Punjabi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn romanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ROMANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Romanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn russian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    RUSSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Russian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn serbian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SERBIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Serbian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn shona_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SHONA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Shona);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovak_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVAK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Slovak);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovene_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVENE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Slovene);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn somali_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOMALI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Somali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn sotho_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOTHO_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Sotho);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn spanish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SPANISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Spanish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swahili_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWAHILI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Swahili);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swedish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWEDISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Swedish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tagalog_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAGALOG_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tagalog);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tamil_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAMIL_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tamil);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn telugu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TELUGU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Telugu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn thai_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    THAI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Thai);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tsonga_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSONGA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tsonga);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tswana_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSWANA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tswana);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn turkish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TURKISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Turkish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ukrainian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    UKRAINIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Ukrainian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn urdu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    URDU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Urdu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn vietnamese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    VIETNAMESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Vietnamese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn welsh_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    WELSH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Welsh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn xhosa_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    XHOSA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Xhosa);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn yoruba_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    YORUBA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Yoruba);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn zulu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_QUADRIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ZULU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Zulu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn load_quadrigrams(language: Language) -> String {
    load_json(LANGUAGE_MODELS_DIRECTORY, language, 4).unwrap()
}
