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

use crate::ngram::Ngram;
use crate::Language;
use cfg_if::cfg_if;
use include_dir::Dir;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;

cfg_if! {
    if #[cfg(test)] {
        use crate::model::MockTrainingDataLanguageModel as TrainingDataLanguageModel;
    } else {
        use crate::model::TrainingDataLanguageModel;
    }
}

pub(crate) mod bigram_models;
pub(crate) mod fivegram_models;
pub(crate) mod quadrigram_models;
pub(crate) mod trigram_models;
pub(crate) mod unigram_models;

pub(crate) type LazyTrainingDataLanguageModel = &'static TrainingDataLanguageModel;
pub(crate) type LanguageToNgramsMappingCell =
    OnceCell<HashMap<Language, LazyTrainingDataLanguageModel>>;
pub(crate) type LazyLanguageToNgramsMapping =
    &'static HashMap<Language, LazyTrainingDataLanguageModel>;

fn load_json(directory: Dir, language: Language, ngram_length: u32) -> std::io::Result<String> {
    let ngram_name = Ngram::get_ngram_name_by_length(ngram_length);
    let file_path = format!("{}/{}s.json.zip", language.iso_code_639_1(), ngram_name);
    let zip_file = directory.get_file(file_path).unwrap();
    let zip_file_reader = Cursor::new(zip_file.contents());
    let mut archive = ZipArchive::new(zip_file_reader).unwrap();
    let mut json_file = archive.by_index(0).unwrap();
    let mut json = String::new();
    json_file.read_to_string(&mut json)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::include_dir;

    const LANGUAGE_MODELS_TEST_DIRECTORY: Dir = include_dir!("assets/test/language-models");

    #[test]
    fn test_load_json() {
        let result = load_json(LANGUAGE_MODELS_TEST_DIRECTORY, Language::English, 1);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"language":"ENGLISH","ngrams":{"2/93616591":"ﬀ ċ ė ĩ ȼ ɔ ţ ũ ʔ ơ ả ộ ù"}}"#
        );
    }
}
