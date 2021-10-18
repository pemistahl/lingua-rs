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

use crate::model::TrainingDataLanguageModel;
use crate::models::{
    load_json, BoxedLanguageModel, LanguageToNgramsMappingCell, LazyLanguageToNgramsMapping,
    LazyTrainingDataLanguageModel,
};
use crate::Language;
use crate::Language::*;
use once_cell::sync::OnceCell;

pub(crate) fn quadrigram_models() -> LazyLanguageToNgramsMapping {
    static QUADRIGRAM_MODELS: LanguageToNgramsMappingCell = OnceCell::new();
    QUADRIGRAM_MODELS.get_or_init(|| {
        hashmap!(
            Afrikaans => afrikaans_quadrigram_model as fn() -> LazyTrainingDataLanguageModel,
            Albanian => albanian_quadrigram_model,
            Arabic => arabic_quadrigram_model,
            Armenian => armenian_quadrigram_model,
            Azerbaijani => azerbaijani_quadrigram_model,
            Basque => basque_quadrigram_model,
            Belarusian => belarusian_quadrigram_model,
            Bengali => bengali_quadrigram_model,
            Bokmal => bokmal_quadrigram_model,
            Bosnian => bosnian_quadrigram_model,
            Bulgarian => bulgarian_quadrigram_model,
            Catalan => catalan_quadrigram_model,
            Chinese => chinese_quadrigram_model,
            Croatian => croatian_quadrigram_model,
            Czech => czech_quadrigram_model,
            Danish => danish_quadrigram_model,
            Dutch => dutch_quadrigram_model,
            English => english_quadrigram_model,
            Esperanto => esperanto_quadrigram_model,
            Estonian => estonian_quadrigram_model,
            Finnish => finnish_quadrigram_model,
            French => french_quadrigram_model,
            Ganda => ganda_quadrigram_model,
            Georgian => georgian_quadrigram_model,
            German => german_quadrigram_model,
            Greek => greek_quadrigram_model,
            Gujarati => gujarati_quadrigram_model,
            Hebrew => hebrew_quadrigram_model,
            Hindi => hindi_quadrigram_model,
            Hungarian => hungarian_quadrigram_model,
            Icelandic => icelandic_quadrigram_model,
            Indonesian => indonesian_quadrigram_model,
            Irish => irish_quadrigram_model,
            Italian => italian_quadrigram_model,
            Japanese => japanese_quadrigram_model,
            Kazakh => kazakh_quadrigram_model,
            Korean => korean_quadrigram_model,
            Latin => latin_quadrigram_model,
            Latvian => latvian_quadrigram_model,
            Lithuanian => lithuanian_quadrigram_model,
            Macedonian => macedonian_quadrigram_model,
            Malay => malay_quadrigram_model,
            Maori => maori_quadrigram_model,
            Marathi => marathi_quadrigram_model,
            Mongolian => mongolian_quadrigram_model,
            Nynorsk => nynorsk_quadrigram_model,
            Persian => persian_quadrigram_model,
            Polish => polish_quadrigram_model,
            Portuguese => portuguese_quadrigram_model,
            Punjabi => punjabi_quadrigram_model,
            Romanian => romanian_quadrigram_model,
            Russian => russian_quadrigram_model,
            Serbian => serbian_quadrigram_model,
            Shona => shona_quadrigram_model,
            Slovak => slovak_quadrigram_model,
            Slovene => slovene_quadrigram_model,
            Somali => somali_quadrigram_model,
            Sotho => sotho_quadrigram_model,
            Spanish => spanish_quadrigram_model,
            Swahili => swahili_quadrigram_model,
            Swedish => swedish_quadrigram_model,
            Tagalog => tagalog_quadrigram_model,
            Tamil => tamil_quadrigram_model,
            Telugu => telugu_quadrigram_model,
            Thai => thai_quadrigram_model,
            Tsonga => tsonga_quadrigram_model,
            Tswana => tswana_quadrigram_model,
            Turkish => turkish_quadrigram_model,
            Ukrainian => ukrainian_quadrigram_model,
            Urdu => urdu_quadrigram_model,
            Vietnamese => vietnamese_quadrigram_model,
            Welsh => welsh_quadrigram_model,
            Xhosa => xhosa_quadrigram_model,
            Yoruba => yoruba_quadrigram_model,
            Zulu => zulu_quadrigram_model
        )
    })
}

fn afrikaans_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static AFRIKAANS_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    AFRIKAANS_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Afrikaans);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn albanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ALBANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Albanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn arabic_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ARABIC_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Arabic);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn armenian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ARMENIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Armenian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn azerbaijani_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    AZERBAIJANI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Azerbaijani);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn basque_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BASQUE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Basque);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn belarusian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BELARUSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Belarusian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bengali_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BENGALI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bengali);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bokmal_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BOKMAL_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bokmal);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bosnian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BOSNIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bosnian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bulgarian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BULGARIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Bulgarian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn catalan_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CATALAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Catalan);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn chinese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CHINESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Chinese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn croatian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CROATIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Croatian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn czech_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CZECH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Czech);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn danish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    DANISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Danish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn dutch_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    DUTCH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Dutch);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn english_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ENGLISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(English);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn esperanto_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ESPERANTO_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Esperanto);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn estonian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ESTONIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Estonian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn finnish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    FINNISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Finnish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn french_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    FRENCH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(French);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn ganda_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GANDA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Ganda);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn georgian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GEORGIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Georgian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn german_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GERMAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(German);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn greek_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GREEK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Greek);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn gujarati_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GUJARATI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Gujarati);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hebrew_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HEBREW_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hebrew);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hindi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HINDI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hindi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hungarian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HUNGARIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Hungarian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn icelandic_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ICELANDIC_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Icelandic);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn indonesian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    INDONESIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Indonesian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn irish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    IRISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Irish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn italian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ITALIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Italian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn japanese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    JAPANESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Japanese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn kazakh_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    KAZAKH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Kazakh);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn korean_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    KOREAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Korean);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn latin_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LATIN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Latin);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn latvian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LATVIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Latvian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn lithuanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LITHUANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Lithuanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn macedonian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MACEDONIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Macedonian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn malay_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MALAY_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Malay);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn maori_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MAORI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MAORI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Maori);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn marathi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MARATHI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Marathi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn mongolian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MONGOLIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Mongolian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn nynorsk_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    NYNORSK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Nynorsk);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn persian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PERSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Persian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn polish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    POLISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Polish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn portuguese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PORTUGUESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Portuguese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn punjabi_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PUNJABI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Punjabi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn romanian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ROMANIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Romanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn russian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    RUSSIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Russian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn serbian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SERBIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Serbian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn shona_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SHONA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Shona);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn slovak_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SLOVAK_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Slovak);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn slovene_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SLOVENE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Slovene);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn somali_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SOMALI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Somali);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn sotho_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SOTHO_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Sotho);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn spanish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SPANISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Spanish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn swahili_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SWAHILI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Swahili);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn swedish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SWEDISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Swedish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tagalog_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TAGALOG_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tagalog);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tamil_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TAMIL_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tamil);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn telugu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TELUGU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Telugu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn thai_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    THAI_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Thai);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tsonga_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TSONGA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tsonga);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tswana_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TSWANA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Tswana);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn turkish_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TURKISH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Turkish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn ukrainian_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    UKRAINIAN_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Ukrainian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn urdu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    URDU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Urdu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn vietnamese_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    VIETNAMESE_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Vietnamese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn welsh_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    WELSH_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Welsh);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn xhosa_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    XHOSA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Xhosa);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn yoruba_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    YORUBA_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Yoruba);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn zulu_quadrigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_QUADRIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ZULU_QUADRIGRAM_MODEL.get_or_init(|| {
        let json = load_quadrigrams(Zulu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn load_quadrigrams(language: Language) -> String {
    load_json(language, 4).unwrap()
}
