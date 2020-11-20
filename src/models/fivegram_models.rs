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

pub(crate) fn fivegram_models() -> LazyLanguageToNgramsMapping {
    static FIVEGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    FIVEGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_fivegram_model(),
            Albanian => albanian_fivegram_model(),
            Arabic => arabic_fivegram_model(),
            Armenian => armenian_fivegram_model(),
            Azerbaijani => azerbaijani_fivegram_model(),
            Basque => basque_fivegram_model(),
            Belarusian => belarusian_fivegram_model(),
            Bengali => bengali_fivegram_model(),
            Bokmal => bokmal_fivegram_model(),
            Bosnian => bosnian_fivegram_model(),
            Bulgarian => bulgarian_fivegram_model(),
            Catalan => catalan_fivegram_model(),
            Chinese => chinese_fivegram_model(),
            Croatian => croatian_fivegram_model(),
            Czech => czech_fivegram_model(),
            Danish => danish_fivegram_model(),
            Dutch => dutch_fivegram_model(),
            English => english_fivegram_model(),
            Esperanto => esperanto_fivegram_model(),
            Estonian => estonian_fivegram_model(),
            Finnish => finnish_fivegram_model(),
            French => french_fivegram_model(),
            Ganda => ganda_fivegram_model(),
            Georgian => georgian_fivegram_model(),
            German => german_fivegram_model(),
            Greek => greek_fivegram_model(),
            Gujarati => gujarati_fivegram_model(),
            Hebrew => hebrew_fivegram_model(),
            Hindi => hindi_fivegram_model(),
            Hungarian => hungarian_fivegram_model(),
            Icelandic => icelandic_fivegram_model(),
            Indonesian => indonesian_fivegram_model(),
            Irish => irish_fivegram_model(),
            Italian => italian_fivegram_model(),
            Japanese => japanese_fivegram_model(),
            Kazakh => kazakh_fivegram_model(),
            Korean => korean_fivegram_model(),
            Latin => latin_fivegram_model(),
            Latvian => latvian_fivegram_model(),
            Lithuanian => lithuanian_fivegram_model(),
            Macedonian => macedonian_fivegram_model(),
            Malay => malay_fivegram_model(),
            Marathi => marathi_fivegram_model(),
            Mongolian => mongolian_fivegram_model(),
            Nynorsk => nynorsk_fivegram_model(),
            Persian => persian_fivegram_model(),
            Polish => polish_fivegram_model(),
            Portuguese => portuguese_fivegram_model(),
            Punjabi => punjabi_fivegram_model(),
            Romanian => romanian_fivegram_model(),
            Russian => russian_fivegram_model(),
            Serbian => serbian_fivegram_model(),
            Shona => shona_fivegram_model(),
            Slovak => slovak_fivegram_model(),
            Slovene => slovene_fivegram_model(),
            Somali => somali_fivegram_model(),
            Sotho => sotho_fivegram_model(),
            Spanish => spanish_fivegram_model(),
            Swahili => swahili_fivegram_model(),
            Swedish => swedish_fivegram_model(),
            Tagalog => tagalog_fivegram_model(),
            Tamil => tamil_fivegram_model(),
            Telugu => telugu_fivegram_model(),
            Thai => thai_fivegram_model(),
            Tsonga => tsonga_fivegram_model(),
            Tswana => tswana_fivegram_model(),
            Turkish => turkish_fivegram_model(),
            Ukrainian => ukrainian_fivegram_model(),
            Urdu => urdu_fivegram_model(),
            Vietnamese => vietnamese_fivegram_model(),
            Welsh => welsh_fivegram_model(),
            Xhosa => xhosa_fivegram_model(),
            Yoruba => yoruba_fivegram_model(),
            Zulu => zulu_fivegram_model()
        )
    })
}

fn afrikaans_fivegram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AFRIKAANS_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Afrikaans);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn albanian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ALBANIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Albanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn arabic_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARABIC_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Arabic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn armenian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ARMENIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Armenian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn azerbaijani_fivegram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    AZERBAIJANI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Azerbaijani);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn basque_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BASQUE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Basque);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn belarusian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BELARUSIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Belarusian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bengali_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BENGALI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Bengali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bokmal_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOKMAL_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Bokmal);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bosnian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BOSNIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Bosnian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn bulgarian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    BULGARIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Bulgarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn catalan_fivegram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CATALAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Catalan);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn chinese_fivegram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CHINESE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Chinese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn croatian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CROATIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Croatian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn czech_fivegram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    CZECH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Czech);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn danish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DANISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Danish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn dutch_fivegram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    DUTCH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Dutch);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn english_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ENGLISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(English);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn esperanto_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESPERANTO_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Esperanto);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn estonian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ESTONIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Estonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn finnish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FINNISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Finnish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn french_fivegram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    FRENCH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(French);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ganda_fivegram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GANDA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Ganda);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn georgian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GEORGIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Georgian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn german_fivegram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GERMAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(German);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn greek_fivegram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GREEK_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Greek);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn gujarati_fivegram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    GUJARATI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Gujarati);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hebrew_fivegram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HEBREW_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Hebrew);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hindi_fivegram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HINDI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Hindi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn hungarian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    HUNGARIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Hungarian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn icelandic_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ICELANDIC_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Icelandic);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn indonesian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    INDONESIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Indonesian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn irish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    IRISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Irish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn italian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ITALIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Italian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn japanese_fivegram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    JAPANESE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Japanese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn kazakh_fivegram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KAZAKH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Kazakh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn korean_fivegram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    KOREAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Korean);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latin_fivegram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATIN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Latin);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn latvian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LATVIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Latvian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn lithuanian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    LITHUANIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Lithuanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn macedonian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MACEDONIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Macedonian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn malay_fivegram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MALAY_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Malay);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn marathi_fivegram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MARATHI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Marathi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn mongolian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    MONGOLIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Mongolian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn nynorsk_fivegram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    NYNORSK_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Nynorsk);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn persian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PERSIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Persian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn polish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    POLISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Polish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn portuguese_fivegram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PORTUGUESE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Portuguese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn punjabi_fivegram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    PUNJABI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Punjabi);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn romanian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ROMANIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Romanian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn russian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    RUSSIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Russian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn serbian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SERBIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Serbian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn shona_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SHONA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Shona);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovak_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVAK_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Slovak);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn slovene_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SLOVENE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Slovene);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn somali_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOMALI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Somali);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn sotho_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SOTHO_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Sotho);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn spanish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SPANISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Spanish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swahili_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWAHILI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Swahili);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn swedish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    SWEDISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Swedish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tagalog_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAGALOG_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Tagalog);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tamil_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TAMIL_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Tamil);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn telugu_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TELUGU_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Telugu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn thai_fivegram_model() -> LazyTrainingDataLanguageModel {
    static THAI_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    THAI_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Thai);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tsonga_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSONGA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Tsonga);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn tswana_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TSWANA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Tswana);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn turkish_fivegram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    TURKISH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Turkish);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn ukrainian_fivegram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    UKRAINIAN_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Ukrainian);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn urdu_fivegram_model() -> LazyTrainingDataLanguageModel {
    static URDU_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    URDU_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Urdu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn vietnamese_fivegram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    VIETNAMESE_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Vietnamese);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn welsh_fivegram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    WELSH_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Welsh);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn xhosa_fivegram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    XHOSA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Xhosa);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn yoruba_fivegram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    YORUBA_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Yoruba);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn zulu_fivegram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_FIVEGRAM_MODEL: OnceCell<TrainingDataLanguageModel> = OnceCell::new();
    ZULU_FIVEGRAM_MODEL.get_or_init(|| {
        let json = load_fivegrams(Zulu);
        TrainingDataLanguageModel::from_json(&json)
    })
}

fn load_fivegrams(language: Language) -> String {
    load_json(language, 5).unwrap()
}
