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
    static AFRIKAANS_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    AFRIKAANS_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Afrikaans);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn albanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ALBANIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ALBANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Albanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn arabic_bigram_model() -> LazyTrainingDataLanguageModel {
    static ARABIC_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ARABIC_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Arabic);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn armenian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ARMENIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ARMENIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Armenian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn azerbaijani_bigram_model() -> LazyTrainingDataLanguageModel {
    static AZERBAIJANI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    AZERBAIJANI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Azerbaijani);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn basque_bigram_model() -> LazyTrainingDataLanguageModel {
    static BASQUE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BASQUE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Basque);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn belarusian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BELARUSIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BELARUSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Belarusian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bengali_bigram_model() -> LazyTrainingDataLanguageModel {
    static BENGALI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BENGALI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bengali);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bokmal_bigram_model() -> LazyTrainingDataLanguageModel {
    static BOKMAL_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BOKMAL_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bokmal);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bosnian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BOSNIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BOSNIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bosnian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn bulgarian_bigram_model() -> LazyTrainingDataLanguageModel {
    static BULGARIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    BULGARIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Bulgarian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn catalan_bigram_model() -> LazyTrainingDataLanguageModel {
    static CATALAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CATALAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Catalan);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn chinese_bigram_model() -> LazyTrainingDataLanguageModel {
    static CHINESE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CHINESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Chinese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn croatian_bigram_model() -> LazyTrainingDataLanguageModel {
    static CROATIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CROATIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Croatian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn czech_bigram_model() -> LazyTrainingDataLanguageModel {
    static CZECH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    CZECH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Czech);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn danish_bigram_model() -> LazyTrainingDataLanguageModel {
    static DANISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    DANISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Danish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn dutch_bigram_model() -> LazyTrainingDataLanguageModel {
    static DUTCH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    DUTCH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Dutch);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn english_bigram_model() -> LazyTrainingDataLanguageModel {
    static ENGLISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ENGLISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(English);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn esperanto_bigram_model() -> LazyTrainingDataLanguageModel {
    static ESPERANTO_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ESPERANTO_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Esperanto);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn estonian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ESTONIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ESTONIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Estonian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn finnish_bigram_model() -> LazyTrainingDataLanguageModel {
    static FINNISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    FINNISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Finnish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn french_bigram_model() -> LazyTrainingDataLanguageModel {
    static FRENCH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    FRENCH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(French);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn ganda_bigram_model() -> LazyTrainingDataLanguageModel {
    static GANDA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GANDA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Ganda);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn georgian_bigram_model() -> LazyTrainingDataLanguageModel {
    static GEORGIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GEORGIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Georgian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn german_bigram_model() -> LazyTrainingDataLanguageModel {
    static GERMAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GERMAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(German);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn greek_bigram_model() -> LazyTrainingDataLanguageModel {
    static GREEK_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GREEK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Greek);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn gujarati_bigram_model() -> LazyTrainingDataLanguageModel {
    static GUJARATI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    GUJARATI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Gujarati);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hebrew_bigram_model() -> LazyTrainingDataLanguageModel {
    static HEBREW_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HEBREW_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hebrew);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hindi_bigram_model() -> LazyTrainingDataLanguageModel {
    static HINDI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HINDI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hindi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn hungarian_bigram_model() -> LazyTrainingDataLanguageModel {
    static HUNGARIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    HUNGARIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Hungarian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn icelandic_bigram_model() -> LazyTrainingDataLanguageModel {
    static ICELANDIC_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ICELANDIC_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Icelandic);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn indonesian_bigram_model() -> LazyTrainingDataLanguageModel {
    static INDONESIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    INDONESIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Indonesian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn irish_bigram_model() -> LazyTrainingDataLanguageModel {
    static IRISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    IRISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Irish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn italian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ITALIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ITALIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Italian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn japanese_bigram_model() -> LazyTrainingDataLanguageModel {
    static JAPANESE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    JAPANESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Japanese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn kazakh_bigram_model() -> LazyTrainingDataLanguageModel {
    static KAZAKH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    KAZAKH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Kazakh);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn korean_bigram_model() -> LazyTrainingDataLanguageModel {
    static KOREAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    KOREAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Korean);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn latin_bigram_model() -> LazyTrainingDataLanguageModel {
    static LATIN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LATIN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Latin);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn latvian_bigram_model() -> LazyTrainingDataLanguageModel {
    static LATVIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LATVIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Latvian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn lithuanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static LITHUANIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    LITHUANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Lithuanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn macedonian_bigram_model() -> LazyTrainingDataLanguageModel {
    static MACEDONIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MACEDONIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Macedonian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn malay_bigram_model() -> LazyTrainingDataLanguageModel {
    static MALAY_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MALAY_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Malay);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn maori_bigram_model() -> LazyTrainingDataLanguageModel {
    static MAORI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MAORI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Maori);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn marathi_bigram_model() -> LazyTrainingDataLanguageModel {
    static MARATHI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MARATHI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Marathi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn mongolian_bigram_model() -> LazyTrainingDataLanguageModel {
    static MONGOLIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    MONGOLIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Mongolian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn nynorsk_bigram_model() -> LazyTrainingDataLanguageModel {
    static NYNORSK_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    NYNORSK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Nynorsk);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn persian_bigram_model() -> LazyTrainingDataLanguageModel {
    static PERSIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PERSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Persian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn polish_bigram_model() -> LazyTrainingDataLanguageModel {
    static POLISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    POLISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Polish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn portuguese_bigram_model() -> LazyTrainingDataLanguageModel {
    static PORTUGUESE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PORTUGUESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Portuguese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn punjabi_bigram_model() -> LazyTrainingDataLanguageModel {
    static PUNJABI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    PUNJABI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Punjabi);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn romanian_bigram_model() -> LazyTrainingDataLanguageModel {
    static ROMANIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ROMANIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Romanian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn russian_bigram_model() -> LazyTrainingDataLanguageModel {
    static RUSSIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    RUSSIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Russian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn serbian_bigram_model() -> LazyTrainingDataLanguageModel {
    static SERBIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SERBIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Serbian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn shona_bigram_model() -> LazyTrainingDataLanguageModel {
    static SHONA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SHONA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Shona);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn slovak_bigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVAK_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SLOVAK_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Slovak);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn slovene_bigram_model() -> LazyTrainingDataLanguageModel {
    static SLOVENE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SLOVENE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Slovene);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn somali_bigram_model() -> LazyTrainingDataLanguageModel {
    static SOMALI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SOMALI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Somali);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn sotho_bigram_model() -> LazyTrainingDataLanguageModel {
    static SOTHO_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SOTHO_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Sotho);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn spanish_bigram_model() -> LazyTrainingDataLanguageModel {
    static SPANISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SPANISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Spanish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn swahili_bigram_model() -> LazyTrainingDataLanguageModel {
    static SWAHILI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SWAHILI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Swahili);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn swedish_bigram_model() -> LazyTrainingDataLanguageModel {
    static SWEDISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    SWEDISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Swedish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tagalog_bigram_model() -> LazyTrainingDataLanguageModel {
    static TAGALOG_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TAGALOG_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tagalog);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tamil_bigram_model() -> LazyTrainingDataLanguageModel {
    static TAMIL_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TAMIL_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tamil);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn telugu_bigram_model() -> LazyTrainingDataLanguageModel {
    static TELUGU_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TELUGU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Telugu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn thai_bigram_model() -> LazyTrainingDataLanguageModel {
    static THAI_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    THAI_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Thai);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tsonga_bigram_model() -> LazyTrainingDataLanguageModel {
    static TSONGA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TSONGA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tsonga);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn tswana_bigram_model() -> LazyTrainingDataLanguageModel {
    static TSWANA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TSWANA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Tswana);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn turkish_bigram_model() -> LazyTrainingDataLanguageModel {
    static TURKISH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    TURKISH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Turkish);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn ukrainian_bigram_model() -> LazyTrainingDataLanguageModel {
    static UKRAINIAN_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    UKRAINIAN_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Ukrainian);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn urdu_bigram_model() -> LazyTrainingDataLanguageModel {
    static URDU_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    URDU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Urdu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn vietnamese_bigram_model() -> LazyTrainingDataLanguageModel {
    static VIETNAMESE_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    VIETNAMESE_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Vietnamese);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn welsh_bigram_model() -> LazyTrainingDataLanguageModel {
    static WELSH_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    WELSH_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Welsh);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn xhosa_bigram_model() -> LazyTrainingDataLanguageModel {
    static XHOSA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    XHOSA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Xhosa);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn yoruba_bigram_model() -> LazyTrainingDataLanguageModel {
    static YORUBA_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    YORUBA_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Yoruba);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn zulu_bigram_model() -> LazyTrainingDataLanguageModel {
    static ZULU_BIGRAM_MODEL: OnceCell<BoxedLanguageModel> = OnceCell::new();
    ZULU_BIGRAM_MODEL.get_or_init(|| {
        let json = load_bigrams(Zulu);
        Box::new(TrainingDataLanguageModel::from_json(&json))
    })
}

fn load_bigrams(language: Language) -> String {
    load_json(language, 2).unwrap()
}
