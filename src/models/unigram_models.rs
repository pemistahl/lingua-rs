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

pub(crate) fn unigram_models() -> LazyLanguageToNgramsMapping {
    static UNIGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    UNIGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_unigram_model(),
            Albanian => albanian_unigram_model(),
            Arabic => arabic_unigram_model(),
            Armenian => armenian_unigram_model(),
            Azerbaijani => azerbaijani_unigram_model(),
            Basque => basque_unigram_model(),
            Belarusian => belarusian_unigram_model(),
            Bengali => bengali_unigram_model(),
            Bokmal => bokmal_unigram_model(),
            Bosnian => bosnian_unigram_model(),
            Bulgarian => bulgarian_unigram_model(),
            Catalan => catalan_unigram_model(),
            Chinese => chinese_unigram_model(),
            Croatian => croatian_unigram_model(),
            Czech => czech_unigram_model(),
            Danish => danish_unigram_model(),
            Dutch => dutch_unigram_model(),
            English => english_unigram_model(),
            Esperanto => esperanto_unigram_model(),
            Estonian => estonian_unigram_model(),
            Finnish => finnish_unigram_model(),
            French => french_unigram_model(),
            Ganda => ganda_unigram_model(),
            Georgian => georgian_unigram_model(),
            German => german_unigram_model(),
            Greek => greek_unigram_model(),
            Gujarati => gujarati_unigram_model(),
            Hebrew => hebrew_unigram_model(),
            Hindi => hindi_unigram_model(),
            Hungarian => hungarian_unigram_model(),
            Icelandic => icelandic_unigram_model(),
            Indonesian => indonesian_unigram_model(),
            Irish => irish_unigram_model(),
            Italian => italian_unigram_model(),
            Japanese => japanese_unigram_model(),
            Kazakh => kazakh_unigram_model(),
            Korean => korean_unigram_model(),
            Latin => latin_unigram_model(),
            Latvian => latvian_unigram_model(),
            Lithuanian => lithuanian_unigram_model(),
            Macedonian => macedonian_unigram_model(),
            Malay => malay_unigram_model(),
            Maori => maori_unigram_model(),
            Marathi => marathi_unigram_model(),
            Mongolian => mongolian_unigram_model(),
            Nynorsk => nynorsk_unigram_model(),
            Persian => persian_unigram_model(),
            Polish => polish_unigram_model(),
            Portuguese => portuguese_unigram_model(),
            Punjabi => punjabi_unigram_model(),
            Romanian => romanian_unigram_model(),
            Russian => russian_unigram_model(),
            Serbian => serbian_unigram_model(),
            Shona => shona_unigram_model(),
            Slovak => slovak_unigram_model(),
            Slovene => slovene_unigram_model(),
            Somali => somali_unigram_model(),
            Sotho => sotho_unigram_model(),
            Spanish => spanish_unigram_model(),
            Swahili => swahili_unigram_model(),
            Swedish => swedish_unigram_model(),
            Tagalog => tagalog_unigram_model(),
            Tamil => tamil_unigram_model(),
            Telugu => telugu_unigram_model(),
            Thai => thai_unigram_model(),
            Tsonga => tsonga_unigram_model(),
            Tswana => tswana_unigram_model(),
            Turkish => turkish_unigram_model(),
            Ukrainian => ukrainian_unigram_model(),
            Urdu => urdu_unigram_model(),
            Vietnamese => vietnamese_unigram_model(),
            Welsh => welsh_unigram_model(),
            Xhosa => xhosa_unigram_model(),
            Yoruba => yoruba_unigram_model(),
            Zulu => zulu_unigram_model()
        )
    })
}

fn afrikaans_unigram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AFRIKAANS_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Afrikaans);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn albanian_unigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ALBANIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Albanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn arabic_unigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARABIC_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Arabic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn armenian_unigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARMENIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Armenian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn azerbaijani_unigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AZERBAIJANI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Azerbaijani);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn basque_unigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BASQUE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Basque);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn belarusian_unigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BELARUSIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Belarusian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bengali_unigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BENGALI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Bengali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bokmal_unigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOKMAL_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Bokmal);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bosnian_unigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOSNIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Bosnian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bulgarian_unigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BULGARIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Bulgarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn catalan_unigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CATALAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Catalan);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn chinese_unigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CHINESE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Chinese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn croatian_unigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CROATIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Croatian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn czech_unigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CZECH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Czech);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn danish_unigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DANISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Danish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn dutch_unigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DUTCH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Dutch);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn english_unigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ENGLISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(English);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn esperanto_unigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESPERANTO_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Esperanto);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn estonian_unigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESTONIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Estonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn finnish_unigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FINNISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Finnish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn french_unigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FRENCH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(French);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ganda_unigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GANDA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Ganda);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn georgian_unigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GEORGIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Georgian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn german_unigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GERMAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(German);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn greek_unigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GREEK_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Greek);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn gujarati_unigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GUJARATI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Gujarati);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hebrew_unigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HEBREW_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Hebrew);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hindi_unigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HINDI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Hindi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hungarian_unigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HUNGARIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Hungarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn icelandic_unigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ICELANDIC_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Icelandic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn indonesian_unigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    INDONESIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Indonesian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn irish_unigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    IRISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Irish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn italian_unigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ITALIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Italian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn japanese_unigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    JAPANESE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Japanese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn kazakh_unigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KAZAKH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Kazakh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn korean_unigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KOREAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Korean);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latin_unigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATIN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Latin);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latvian_unigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATVIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Latvian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn lithuanian_unigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LITHUANIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Lithuanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn macedonian_unigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MACEDONIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Macedonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn malay_unigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MALAY_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Malay);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn maori_unigram_model() -> LazyTrainingDataLanguageModel {
    static MAORI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MAORI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Maori);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn marathi_unigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MARATHI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Marathi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn mongolian_unigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MONGOLIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Mongolian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn nynorsk_unigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    NYNORSK_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Nynorsk);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn persian_unigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PERSIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Persian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn polish_unigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    POLISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Polish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn portuguese_unigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PORTUGUESE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Portuguese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn punjabi_unigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PUNJABI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Punjabi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn romanian_unigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ROMANIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Romanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn russian_unigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    RUSSIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Russian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn serbian_unigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SERBIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Serbian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn shona_unigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SHONA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Shona);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovak_unigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVAK_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Slovak);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovene_unigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVENE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Slovene);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn somali_unigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOMALI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Somali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn sotho_unigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOTHO_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Sotho);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn spanish_unigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SPANISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Spanish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swahili_unigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWAHILI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Swahili);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swedish_unigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWEDISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Swedish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tagalog_unigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAGALOG_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Tagalog);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tamil_unigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAMIL_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Tamil);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn telugu_unigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TELUGU_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Telugu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn thai_unigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    THAI_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Thai);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tsonga_unigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSONGA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Tsonga);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tswana_unigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSWANA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Tswana);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn turkish_unigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TURKISH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Turkish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ukrainian_unigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    UKRAINIAN_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Ukrainian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn urdu_unigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    URDU_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Urdu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn vietnamese_unigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    VIETNAMESE_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Vietnamese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn welsh_unigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    WELSH_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Welsh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn xhosa_unigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    XHOSA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Xhosa);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn yoruba_unigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    YORUBA_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Yoruba);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn zulu_unigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_UNIGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ZULU_UNIGRAM_MODEL.get_or_init(|| {
        let json = load_unigrams(Zulu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn load_unigrams(language: Language) -> String {
    load_json(language, 1).unwrap()
}
