/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
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

#![cfg(target_family = "wasm")]

use lingua::{
    ConfidenceValue, IsoCode639_1, IsoCode639_3, Language, WasmDetectionResult,
    WasmLanguageDetectorBuilder,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_all_languages() {
    WasmLanguageDetectorBuilder::fromAllLanguages();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_spoken_languages() {
    WasmLanguageDetectorBuilder::fromAllSpokenLanguages();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_languages_with_arabic_script() {
    WasmLanguageDetectorBuilder::fromAllLanguagesWithArabicScript();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_languages_with_cyrillic_script() {
    WasmLanguageDetectorBuilder::fromAllLanguagesWithCyrillicScript();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_languages_with_devanagari_script() {
    WasmLanguageDetectorBuilder::fromAllLanguagesWithDevanagariScript();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_languages_with_latin_script() {
    WasmLanguageDetectorBuilder::fromAllLanguagesWithLatinScript();
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_blacklist() {
    let languages = Box::new([
        JsValue::from(Language::Turkish.to_string()),
        JsValue::from(Language::Romanian.to_string()),
    ]);
    let result = WasmLanguageDetectorBuilder::fromAllLanguagesWithout(languages);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_long_blacklist() {
    let languages = Box::new([
        JsValue::from(Language::Afrikaans.to_string()),
        JsValue::from(Language::Albanian.to_string()),
        JsValue::from(Language::Arabic.to_string()),
        JsValue::from(Language::Armenian.to_string()),
        JsValue::from(Language::Azerbaijani.to_string()),
        JsValue::from(Language::Basque.to_string()),
        JsValue::from(Language::Belarusian.to_string()),
        JsValue::from(Language::Bengali.to_string()),
        JsValue::from(Language::Bokmal.to_string()),
        JsValue::from(Language::Bosnian.to_string()),
        JsValue::from(Language::Bulgarian.to_string()),
        JsValue::from(Language::Catalan.to_string()),
        JsValue::from(Language::Chinese.to_string()),
        JsValue::from(Language::Croatian.to_string()),
        JsValue::from(Language::Czech.to_string()),
        JsValue::from(Language::Danish.to_string()),
        JsValue::from(Language::Dutch.to_string()),
        JsValue::from(Language::English.to_string()),
        JsValue::from(Language::Esperanto.to_string()),
        JsValue::from(Language::Estonian.to_string()),
        JsValue::from(Language::Finnish.to_string()),
        JsValue::from(Language::French.to_string()),
        JsValue::from(Language::Ganda.to_string()),
        JsValue::from(Language::Georgian.to_string()),
        JsValue::from(Language::Greek.to_string()),
        JsValue::from(Language::Gujarati.to_string()),
        JsValue::from(Language::Hebrew.to_string()),
        JsValue::from(Language::Hindi.to_string()),
        JsValue::from(Language::Hungarian.to_string()),
        JsValue::from(Language::Icelandic.to_string()),
        JsValue::from(Language::Indonesian.to_string()),
        JsValue::from(Language::Irish.to_string()),
        JsValue::from(Language::Italian.to_string()),
        JsValue::from(Language::Japanese.to_string()),
        JsValue::from(Language::Kazakh.to_string()),
        JsValue::from(Language::Korean.to_string()),
        JsValue::from(Language::Latin.to_string()),
        JsValue::from(Language::Latvian.to_string()),
        JsValue::from(Language::Lithuanian.to_string()),
        JsValue::from(Language::Macedonian.to_string()),
        JsValue::from(Language::Malay.to_string()),
        JsValue::from(Language::Maori.to_string()),
        JsValue::from(Language::Marathi.to_string()),
        JsValue::from(Language::Mongolian.to_string()),
        JsValue::from(Language::Nynorsk.to_string()),
        JsValue::from(Language::Persian.to_string()),
        JsValue::from(Language::Polish.to_string()),
        JsValue::from(Language::Portuguese.to_string()),
        JsValue::from(Language::Punjabi.to_string()),
        JsValue::from(Language::Romanian.to_string()),
        JsValue::from(Language::Russian.to_string()),
        JsValue::from(Language::Serbian.to_string()),
        JsValue::from(Language::Shona.to_string()),
        JsValue::from(Language::Slovak.to_string()),
        JsValue::from(Language::Slovene.to_string()),
        JsValue::from(Language::Somali.to_string()),
        JsValue::from(Language::Sotho.to_string()),
        JsValue::from(Language::Spanish.to_string()),
        JsValue::from(Language::Swahili.to_string()),
        JsValue::from(Language::Swedish.to_string()),
        JsValue::from(Language::Tagalog.to_string()),
        JsValue::from(Language::Tamil.to_string()),
        JsValue::from(Language::Telugu.to_string()),
        JsValue::from(Language::Thai.to_string()),
        JsValue::from(Language::Tsonga.to_string()),
        JsValue::from(Language::Tswana.to_string()),
        JsValue::from(Language::Turkish.to_string()),
        JsValue::from(Language::Ukrainian.to_string()),
        JsValue::from(Language::Urdu.to_string()),
        JsValue::from(Language::Vietnamese.to_string()),
        JsValue::from(Language::Welsh.to_string()),
        JsValue::from(Language::Xhosa.to_string()),
        JsValue::from(Language::Yoruba.to_string()),
        JsValue::from(Language::Zulu.to_string()),
    ]);
    let result = WasmLanguageDetectorBuilder::fromAllLanguagesWithout(languages);
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "LanguageDetector needs at least 2 languages to choose from"
        ))
    );
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_whitelist() {
    let result = WasmLanguageDetectorBuilder::fromLanguages(Box::new([
        JsValue::from(Language::German.to_string()),
        JsValue::from(Language::English.to_string()),
    ]));
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_short_whitelist() {
    let result = WasmLanguageDetectorBuilder::fromLanguages(Box::new([JsValue::from(
        Language::German.to_string(),
    )]));
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "LanguageDetector needs at least 2 languages to choose from"
        ))
    );
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_iso_639_1_codes() {
    let result = WasmLanguageDetectorBuilder::fromISOCodes6391(Box::new([
        JsValue::from(IsoCode639_1::DE.to_string()),
        JsValue::from(IsoCode639_1::ZU.to_string()),
    ]));
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_few_iso_639_1_codes() {
    let result = WasmLanguageDetectorBuilder::fromISOCodes6391(Box::new([JsValue::from(
        IsoCode639_1::DE.to_string(),
    )]));
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "LanguageDetector needs at least 2 languages to choose from"
        ))
    );
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_iso_639_3_codes() {
    let result = WasmLanguageDetectorBuilder::fromISOCodes6393(Box::new([
        JsValue::from(IsoCode639_3::DEU.to_string()),
        JsValue::from(IsoCode639_3::ZUL.to_string()),
    ]));
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_few_iso_639_3_codes() {
    let result = WasmLanguageDetectorBuilder::fromISOCodes6393(Box::new([JsValue::from(
        IsoCode639_3::DEU.to_string(),
    )]));
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "LanguageDetector needs at least 2 languages to choose from"
        ))
    );
}

#[wasm_bindgen_test]
fn assert_detector_can_be_built_from_minimum_relative_distance() {
    let mut builder = WasmLanguageDetectorBuilder::fromAllLanguages();
    let result = builder.withMinimumRelativeDistance(0.25);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_small_minimum_relative_distance() {
    let mut builder = WasmLanguageDetectorBuilder::fromAllLanguages();
    let result = builder.withMinimumRelativeDistance(-2.3);
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "Minimum relative distance must lie in between 0.0 and 0.99"
        ))
    );
}

#[wasm_bindgen_test]
fn assert_detector_cannot_be_built_from_too_large_minimum_relative_distance() {
    let mut builder = WasmLanguageDetectorBuilder::fromAllLanguages();
    let result = builder.withMinimumRelativeDistance(1.7);
    assert_eq!(
        result.err(),
        Some(JsValue::from(
            "Minimum relative distance must lie in between 0.0 and 0.99"
        ))
    );
}

#[wasm_bindgen_test]
fn test_detect_language() {
    let detector = WasmLanguageDetectorBuilder::fromLanguages(Box::new([
        JsValue::from(Language::German.to_string()),
        JsValue::from(Language::English.to_string()),
        JsValue::from(Language::French.to_string()),
    ]))
    .unwrap()
    .build();

    let mut language = detector.detectLanguageOf("mein Haus ist groß");
    assert_eq!(language, Some(Language::German.to_string()));

    language = detector.detectLanguageOf("my house is big");
    assert_eq!(language, Some(Language::English.to_string()));

    language = detector.detectLanguageOf("ma maison est grande");
    assert_eq!(language, Some(Language::French.to_string()));
}

#[wasm_bindgen_test]
fn test_detect_multiple_languages() {
    let detector = WasmLanguageDetectorBuilder::fromLanguages(Box::new([
        JsValue::from(Language::German.to_string()),
        JsValue::from(Language::English.to_string()),
        JsValue::from(Language::French.to_string()),
    ]))
    .unwrap()
    .build();

    let sentence = "Parlez-vous français? Ich spreche Französisch nur ein bisschen. A little bit is better than nothing.";

    let results: Vec<WasmDetectionResult> =
        serde_wasm_bindgen::from_value(detector.detectMultipleLanguagesOf(sentence)).unwrap();

    assert_eq!(results.len(), 3);

    let first_result = &results[0];
    let first_substring = &sentence[first_result.startIndex..first_result.endIndex];
    assert_eq!(first_substring, "Parlez-vous français? ");
    assert_eq!(first_result.language, Language::French.to_string());

    let second_result = &results[1];
    let second_substring = &sentence[second_result.startIndex..second_result.endIndex];
    assert_eq!(
        second_substring,
        "Ich spreche Französisch nur ein bisschen. "
    );
    assert_eq!(second_result.language, Language::German.to_string());

    let third_result = &results[2];
    let third_substring = &sentence[third_result.startIndex..third_result.endIndex];
    assert_eq!(third_substring, "A little bit is better than nothing.");
    assert_eq!(third_result.language, Language::English.to_string());
}

#[wasm_bindgen_test]
fn test_compute_language_confidence_values() {
    let detector = WasmLanguageDetectorBuilder::fromLanguages(Box::new([
        JsValue::from(Language::German.to_string()),
        JsValue::from(Language::English.to_string()),
        JsValue::from(Language::French.to_string()),
    ]))
    .unwrap()
    .build();

    let confidence_values: Vec<ConfidenceValue> = serde_wasm_bindgen::from_value(
        detector.computeLanguageConfidenceValues("mein Haus ist groß"),
    )
    .unwrap();

    let rounded_confidence_values = confidence_values
        .iter()
        .map(|value| ConfidenceValue {
            language: value.language.clone(),
            confidence: (value.confidence * 100000.0).round() / 100000.0,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rounded_confidence_values,
        vec![
            ConfidenceValue {
                language: Language::German.to_string(),
                confidence: 0.9698
            },
            ConfidenceValue {
                language: Language::French.to_string(),
                confidence: 0.01534
            },
            ConfidenceValue {
                language: Language::English.to_string(),
                confidence: 0.01486
            }
        ]
    );
}

#[wasm_bindgen_test]
fn test_compute_language_confidence() {
    let detector = WasmLanguageDetectorBuilder::fromLanguages(Box::new([
        JsValue::from(Language::German.to_string()),
        JsValue::from(Language::English.to_string()),
        JsValue::from(Language::French.to_string()),
    ]))
    .unwrap()
    .build();

    let confidence = detector
        .computeLanguageConfidence("mein Haus ist groß", &Language::German.to_string())
        .unwrap();
    let rounded_confidence = (confidence * 100000.0).round() / 100000.0;
    assert_eq!(rounded_confidence, 0.9698);

    let result = detector.computeLanguageConfidence("mein Haus ist groß", "Sorbian");
    assert_eq!(
        result.err(),
        Some(JsValue::from("Language 'Sorbian' is not supported"))
    );
}
