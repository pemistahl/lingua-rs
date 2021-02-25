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

pub(crate) fn bigram_models() -> LazyLanguageToNgramsMapping {
    static BIGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    BIGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_bigram_model as fn() -> LazyTrainingDataLanguageModel,
            Albanian => albanian_bigram_model,
            Arabic => arabic_bigram_model,
            Armenian => armenian_bigram_model,
            Azerbaijani => azerbaijani_bigram_model,
            Basque => basque_bigram_model,
            Belarusian => belarusian_bigram_model,
            Bengali => bengali_bigram_model,
            Bokmal => bokmal_bigram_model,
            Bosnian => bosnian_bigram_model,
            Bulgarian => bulgarian_bigram_model,
            Catalan => catalan_bigram_model,
            Chinese => chinese_bigram_model,
            Croatian => croatian_bigram_model,
            Czech => czech_bigram_model,
            Danish => danish_bigram_model,
            Dutch => dutch_bigram_model,
            English => english_bigram_model,
            Esperanto => esperanto_bigram_model,
            Estonian => estonian_bigram_model,
            Finnish => finnish_bigram_model,
            French => french_bigram_model,
            Ganda => ganda_bigram_model,
            Georgian => georgian_bigram_model,
            German => german_bigram_model,
            Greek => greek_bigram_model,
            Gujarati => gujarati_bigram_model,
            Hebrew => hebrew_bigram_model,
            Hindi => hindi_bigram_model,
            Hungarian => hungarian_bigram_model,
            Icelandic => icelandic_bigram_model,
            Indonesian => indonesian_bigram_model,
            Irish => irish_bigram_model,
            Italian => italian_bigram_model,
            Japanese => japanese_bigram_model,
            Kazakh => kazakh_bigram_model,
            Korean => korean_bigram_model,
            Latin => latin_bigram_model,
            Latvian => latvian_bigram_model,
            Lithuanian => lithuanian_bigram_model,
            Macedonian => macedonian_bigram_model,
            Malay => malay_bigram_model,
            Maori => maori_bigram_model,
            Marathi => marathi_bigram_model,
            Mongolian => mongolian_bigram_model,
            Nynorsk => nynorsk_bigram_model,
            Persian => persian_bigram_model,
            Polish => polish_bigram_model,
            Portuguese => portuguese_bigram_model,
            Punjabi => punjabi_bigram_model,
            Romanian => romanian_bigram_model,
            Russian => russian_bigram_model,
            Serbian => serbian_bigram_model,
            Shona => shona_bigram_model,
            Slovak => slovak_bigram_model,
            Slovene => slovene_bigram_model,
            Somali => somali_bigram_model,
            Sotho => sotho_bigram_model,
            Spanish => spanish_bigram_model,
            Swahili => swahili_bigram_model,
            Swedish => swedish_bigram_model,
            Tagalog => tagalog_bigram_model,
            Tamil => tamil_bigram_model,
            Telugu => telugu_bigram_model,
            Thai => thai_bigram_model,
            Tsonga => tsonga_bigram_model,
            Tswana => tswana_bigram_model,
            Turkish => turkish_bigram_model,
            Ukrainian => ukrainian_bigram_model,
            Urdu => urdu_bigram_model,
            Vietnamese => vietnamese_bigram_model,
            Welsh => welsh_bigram_model,
            Xhosa => xhosa_bigram_model,
            Yoruba => yoruba_bigram_model,
            Zulu => zulu_bigram_model
        )
    })
}

fn afrikaans_bigram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AFRIKAANS_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Afrikaans);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn albanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ALBANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Albanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn arabic_bigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARABIC_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Arabic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn armenian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARMENIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Armenian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn azerbaijani_bigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AZERBAIJANI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Azerbaijani);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn basque_bigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BASQUE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Basque);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn belarusian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BELARUSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Belarusian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bengali_bigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BENGALI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bengali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bokmal_bigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOKMAL_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bokmal);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bosnian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOSNIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bosnian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bulgarian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BULGARIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bulgarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn catalan_bigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CATALAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Catalan);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn chinese_bigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CHINESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Chinese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn croatian_bigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CROATIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Croatian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn czech_bigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CZECH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Czech);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn danish_bigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DANISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Danish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn dutch_bigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DUTCH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Dutch);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn english_bigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ENGLISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(English);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn esperanto_bigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESPERANTO_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Esperanto);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn estonian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESTONIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Estonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn finnish_bigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FINNISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Finnish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn french_bigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FRENCH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(French);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ganda_bigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GANDA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Ganda);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn georgian_bigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GEORGIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Georgian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn german_bigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GERMAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(German);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn greek_bigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GREEK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Greek);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn gujarati_bigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GUJARATI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Gujarati);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hebrew_bigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HEBREW_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hebrew);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hindi_bigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HINDI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hindi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hungarian_bigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HUNGARIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hungarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn icelandic_bigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ICELANDIC_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Icelandic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn indonesian_bigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    INDONESIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Indonesian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn irish_bigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    IRISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Irish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn italian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ITALIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Italian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn japanese_bigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    JAPANESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Japanese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn kazakh_bigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KAZAKH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Kazakh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn korean_bigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KOREAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Korean);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latin_bigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATIN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Latin);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latvian_bigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATVIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Latvian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn lithuanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LITHUANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Lithuanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn macedonian_bigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MACEDONIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Macedonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn malay_bigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MALAY_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Malay);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn maori_bigram_model() -> LazyTrainingDataLanguageModel {
    static MAORI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MAORI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Maori);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn marathi_bigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MARATHI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Marathi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn mongolian_bigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MONGOLIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Mongolian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn nynorsk_bigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    NYNORSK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Nynorsk);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn persian_bigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PERSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Persian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn polish_bigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    POLISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Polish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn portuguese_bigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PORTUGUESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Portuguese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn punjabi_bigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PUNJABI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Punjabi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn romanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ROMANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Romanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn russian_bigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    RUSSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Russian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn serbian_bigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SERBIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Serbian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn shona_bigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SHONA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Shona);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovak_bigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVAK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Slovak);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovene_bigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVENE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Slovene);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn somali_bigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOMALI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Somali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn sotho_bigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOTHO_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Sotho);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn spanish_bigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SPANISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Spanish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swahili_bigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWAHILI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Swahili);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swedish_bigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWEDISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Swedish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tagalog_bigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAGALOG_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tagalog);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tamil_bigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAMIL_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tamil);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn telugu_bigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TELUGU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Telugu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn thai_bigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    THAI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Thai);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tsonga_bigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSONGA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tsonga);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tswana_bigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSWANA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tswana);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn turkish_bigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TURKISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Turkish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ukrainian_bigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    UKRAINIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Ukrainian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn urdu_bigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    URDU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Urdu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn vietnamese_bigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    VIETNAMESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Vietnamese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn welsh_bigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    WELSH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Welsh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn xhosa_bigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    XHOSA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Xhosa);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn yoruba_bigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    YORUBA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Yoruba);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn zulu_bigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_BIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ZULU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Zulu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn load_bigrams(language: Language) -> String {
    load_json(language, 2).unwrap()
}
