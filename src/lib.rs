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

//! ## 1. What does this library do?
//!
//! Its task is simple: It tells you which language some text is written in.
//! This is very useful as a preprocessing step for linguistic data in natural language
//! processing applications such as text classification and spell checking.
//! Other use cases, for instance, might include routing e-mails to the right geographically
//! located customer service department, based on the e-mails' languages.
//!
//! ## 2. Why does this library exist?
//!
//! Language detection is often done as part of large machine learning frameworks or natural
//! language processing applications. In cases where you don't need the full-fledged
//! functionality of those systems or don't want to learn the ropes of those,
//! a small flexible library comes in handy.
//!
//! So far, other comprehensive open source libraries in the Rust ecosystem for
//! this task are [*CLD2*](https://github.com/emk/rust-cld2),
//! [*Whatlang*](https://github.com/greyblake/whatlang-rs) and
//! [*Whichlang*](https://github.com/quickwit-oss/whichlang).
//! Unfortunately, most of them have two major drawbacks:
//!
//! 1. Detection only works with quite lengthy text fragments. For very short text snippets
//!    such as Twitter messages, it does not provide adequate results.
//! 2. The more languages take part in the decision process, the less accurate are the
//!    detection results.
//!
//! *Lingua* aims at eliminating these problems. She nearly does not need any configuration and
//! yields pretty accurate results on both long and short text, even on single words and phrases.
//! She draws on both rule-based and statistical methods but does not use any dictionaries of words.
//! She does not need a connection to any external API or service either.
//! Once the library has been downloaded, it can be used completely offline.
//!
//! ## 3. Which languages are supported?
//!
//! Compared to other language detection libraries, *Lingua's* focus is on *quality over quantity*,
//! that is, getting detection right for a small set of languages first before adding new ones.
//! Currently, 75 languages are supported. They are listed as variants in the [Language] enum.
//!
//! ## 4. How good is it?
//!
//! *Lingua* is able to report accuracy statistics for some bundled test data available for each
//! supported language. The test data for each language is split into three parts:
//!
//! 1. a list of single words with a minimum length of 5 characters
//! 2. a list of word pairs with a minimum length of 10 characters
//! 3. a list of complete grammatical sentences of various lengths
//!
//! Both the language models and the test data have been created from separate documents of the
//! [Wortschatz corpora](https://wortschatz.uni-leipzig.de) offered by Leipzig University, Germany.
//! Data crawled from various news websites have been used for training, each corpus comprising one
//! million sentences. For testing, corpora made of arbitrarily chosen websites have been used,
//! each comprising ten thousand sentences. From each test corpus, a random unsorted subset of
//! 1000 single words, 1000 word pairs and 1000 sentences has been extracted, respectively.
//!
//! Given the generated test data, I have compared the detection results of *Lingua*, *CLD2*,
//! *Whatlang* and *Whichlang* running over the data of *Lingua's* supported 75 languages.
//! Languages that are not supported by the other classifiers are simply ignored for the
//! respective library during the detection process.
//!
//! The results of this comparison are available
//! [here](https://github.com/pemistahl/lingua-rs#4-how-accurate-is-it).
//!
//! ## 5. Why is it better than other libraries?
//!
//! Every language detector uses a probabilistic [n-gram](https://en.wikipedia.org/wiki/N-gram)
//! model trained on the character distribution in some training corpus. Most libraries only use
//! n-grams of size 3 (trigrams) which is satisfactory for detecting the language of longer text
//! fragments consisting of multiple sentences. For short phrases or single words, however,
//! trigrams are not enough. The shorter the input text is, the less n-grams are available.
//! The probabilities estimated from such few n-grams are not reliable. This is why *Lingua* makes
//! use of n-grams of sizes 1 up to 5 which results in much more accurate prediction of the correct
//! language.
//!
//! A second important difference is that *Lingua* does not only use such a statistical model, but
//! also a rule-based engine. This engine first determines the alphabet of the input text and
//! searches for characters which are unique in one or more languages. If exactly one language can
//! be reliably chosen this way, the statistical model is not necessary anymore. In any case, the
//! rule-based engine filters out languages that do not satisfy the conditions of the input text.
//! Only then, in a second step, the probabilistic n-gram model is taken into consideration.
//! This makes sense because loading less language models means less memory consumption and better
//! runtime performance.
//!
//! In general, it is always a good idea to restrict the set of languages to be considered in the
//! classification process using the respective api methods. If you know beforehand that certain
//! languages are never to occur in an input text, do not let those take part in the classification
//! process. The filtering mechanism of the rule-based engine is quite good, however, filtering
//! based on your own knowledge of the input text is always preferable.
//!
//! ## 6. How to add it to your project?
//!
//! Add *Lingua* to your `Cargo.toml` file like so:
//!
//! ```toml
//! [dependencies]
//! lingua = "1.6.2"
//! ```
//!
//! By default, this will download the language model dependencies for all 75 supported languages,
//! a total of approximately 90 MB. If your bandwidth or hard drive space is limited, or you simply
//! do not need all languages, you can specify a subset of the language models to be downloaded as
//! separate features in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lingua = { version = "1.6.2", default-features = false, features = ["french", "italian", "spanish"] }
//! ```
//!
//! ## 7. How to use?
//!
//! ### 7.1 Basic usage
//!
//! ```
//! use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
//! use lingua::Language::{English, French, German, Spanish};
//!
//! let languages = vec![English, French, German, Spanish];
//! let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
//! let detected_language: Option<Language> = detector.detect_language_of("languages are awesome");
//!
//! assert_eq!(detected_language, Some(English));
//! ```
//!
//! ### 7.2 Minimum relative distance
//!
//! By default, *Lingua* returns the most likely language for a given input text. However, there are
//! certain words that are spelled the same in more than one language. The word *prologue*, for
//! instance, is both a valid English and French word. *Lingua* would output either English or
//! French which might be wrong in the given context. For cases like that, it is possible to
//! specify a minimum relative distance that the logarithmized and summed up probabilities for
//! each possible language have to satisfy. It can be stated in the following way:
//!
//! ```
//! use lingua::LanguageDetectorBuilder;
//! use lingua::Language::{English, French, German, Spanish};
//!
//! let detector = LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
//!     .with_minimum_relative_distance(0.9)
//!     .build();
//! let detected_language = detector.detect_language_of("languages are awesome");
//!
//! assert_eq!(detected_language, None);
//! ```
//!
//! Be aware that the distance between the language probabilities is dependent on the length of the
//! input text. The longer the input text, the larger the distance between the languages. So if you
//! want to classify very short text phrases, do not set the minimum relative distance too high.
//! Otherwise [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) will be
//! returned most of the time as in the example above. This is the return value for cases where
//! language detection is not reliably possible.
//!
//! ### 7.3 Confidence values
//!
//! Knowing about the most likely language is nice but how reliable is the computed likelihood?
//! And how less likely are the other examined languages in comparison to the most likely one?
//! These questions can be answered as well:
//!
//! ```
//! use lingua::Language::{English, French, German, Spanish};
//! use lingua::{Language, LanguageDetectorBuilder};
//!
//! let languages = vec![English, French, German, Spanish];
//! let detector = LanguageDetectorBuilder::from_languages(&languages).build();
//! let confidence_values: Vec<(Language, f64)> = detector
//!     .compute_language_confidence_values("languages are awesome")
//!     .into_iter()
//!     // Let's round the values to two decimal places for easier assertions
//!     .map(|(language, confidence)| (language, (confidence * 100.0).round() / 100.0))
//!     .collect();
//!
//! assert_eq!(
//!     confidence_values,
//!     vec![(English, 0.93), (French, 0.04), (German, 0.02), (Spanish, 0.01)]
//! );
//! ```
//!
//! In the example above, a vector of two-element tuples is returned containing all possible
//! languages sorted by their confidence value in descending order. Each value is a probability
//! between 0.0 and 1.0. The probabilities of all languages will sum to 1.0. If the language is
//! unambiguously identified by the rule engine, the value 1.0 will always be returned for this
//! language. The other languages will receive a value of 0.0.
//!
//! There is also a method for returning the confidence value for one specific language only:
//!
//! ```
//! use lingua::Language::{English, French, German, Spanish};
//! use lingua::LanguageDetectorBuilder;
//!
//! let languages = vec![English, French, German, Spanish];
//! let detector = LanguageDetectorBuilder::from_languages(&languages).build();
//! let confidence = detector.compute_language_confidence("languages are awesome", French);
//! let rounded_confidence = (confidence * 100.0).round() / 100.0;
//!
//! assert_eq!(rounded_confidence, 0.04);
//! ```
//!
//! The value that this method computes is a number between 0.0 and 1.0.
//! If the language is unambiguously identified by the rule engine, the value
//! 1.0 will always be returned. If the given language is not supported by
//! this detector instance, the value 0.0 will always be returned.
//!
//! ### 7.4 Eager loading versus lazy loading
//!
//! By default, *Lingua* uses lazy-loading to load only those language models on demand which are
//! considered relevant by the rule-based filter engine. For web services, for instance, it is
//! rather beneficial to preload all language models into memory to avoid unexpected latency while
//! waiting for the service response. If you want to enable the eager-loading mode, you can do it
//! like this:
//!
//! ```
//! use lingua::LanguageDetectorBuilder;
//!
//! LanguageDetectorBuilder::from_all_languages().with_preloaded_language_models().build();
//! ```
//!
//! Multiple instances of `LanguageDetector` share the same language models in memory which are
//! accessed asynchronously by the instances.
//!
//! ### 7.5 Low accuracy mode versus high accuracy mode
//!
//! *Lingua's* high detection accuracy comes at the cost of being noticeably slower
//! than other language detectors. The large language models also consume significant
//! amounts of memory. These requirements might not be feasible for systems running low
//! on resources. If you want to classify mostly long texts or need to save resources,
//! you can enable a *low accuracy mode* that loads only a small subset of the language
//! models into memory:
//!
//! ```
//! use lingua::LanguageDetectorBuilder;
//!
//! LanguageDetectorBuilder::from_all_languages().with_low_accuracy_mode().build();
//! ```
//!
//! The downside of this approach is that detection accuracy for short texts consisting
//! of less than 120 characters will drop significantly. However, detection accuracy for
//! texts which are longer than 120 characters will remain mostly unaffected.
//!
//! In high accuracy mode (the default), the language detector consumes approximately
//! 1,200 MB of memory if all language models are loaded. In low accuracy mode, memory
//! consumption is reduced to approximately 90 MB. The goal is to further reduce memory
//! consumption in later releases.
//!
//! An alternative for a smaller memory footprint and faster performance is to reduce the set
//! of languages when building the language detector. In most cases, it is not advisable to
//! build the detector from all supported languages. When you have knowledge about
//! the texts you want to classify you can almost always rule out certain languages as impossible
//! or unlikely to occur.
//!
//! ### 7.6 Detection of multiple languages in mixed-language texts
//!
//! In contrast to most other language detectors, *Lingua* is able to detect multiple languages
//! in mixed-language texts. This feature can yield quite reasonable results, but it is still
//! in an experimental state and therefore the detection result is highly dependent on the input
//! text. It works best in high-accuracy mode with multiple long words for each language.
//! The shorter the phrases and their words are, the less accurate are the results. Reducing the
//! set of languages when building the language detector can also improve accuracy for this task
//! if the languages occurring in the text are equal to the languages supported by the respective
//! language detector instance.
//!
//! ```
//! use lingua::DetectionResult;
//! use lingua::Language::{English, French, German};
//! use lingua::LanguageDetectorBuilder;
//!
//! let languages = vec![English, French, German];
//! let detector = LanguageDetectorBuilder::from_languages(&languages).build();
//! let sentence = "Parlez-vous français? \
//!     Ich spreche Französisch nur ein bisschen. \
//!     A little bit is better than nothing.";
//!
//! let results: Vec<DetectionResult> = detector.detect_multiple_languages_of(sentence);
//!
//! if let [first, second, third] = &results[..] {
//!     assert_eq!(first.language(), French);
//!     assert_eq!(
//!         &sentence[first.start_index()..first.end_index()],
//!         "Parlez-vous français? "
//!     );
//!
//!     assert_eq!(second.language(), German);
//!     assert_eq!(
//!         &sentence[second.start_index()..second.end_index()],
//!         "Ich spreche Französisch nur ein bisschen. "
//!     );
//!
//!     assert_eq!(third.language(), English);
//!     assert_eq!(
//!         &sentence[third.start_index()..third.end_index()],
//!         "A little bit is better than nothing."
//!     );
//! }
//! ```
//!
//! In the example above, a vector of [DetectionResult] is returned. Each entry in the vector
//! describes a contiguous single-language text section, providing start and end indices of the
//! respective substring.
//!
//! ### 7.7 Single-threaded versus multi-threaded language detection
//!
//! The `LanguageDetector` methods explained above all operate in a single thread.
//! If you want to classify a very large set of texts, you will probably want to
//! use all available CPU cores efficiently in multiple threads for maximum performance.
//!
//! Every single-threaded method has a multi-threaded equivalent that accepts a list of texts
//! and returns a list of results.
//!
//! | Single-threaded                      | Multi-threaded                                   |
//! |--------------------------------------|--------------------------------------------------|
//! | `detect_language_of`                 | `detect_languages_in_parallel_of`                |
//! | `detect_multiple_languages_of`       | `detect_multiple_languages_in_parallel_of`       |
//! | `compute_language_confidence_values` | `compute_language_confidence_values_in_parallel` |
//! | `compute_language_confidence`        | `compute_language_confidence_in_parallel`        |
//!
//! ### 7.8 Methods to build the LanguageDetector
//!
//! There might be classification tasks where you know beforehand that your language data is
//! definitely not written in Latin, for instance (what a surprise :-). The detection accuracy can
//! become better in such cases if you exclude certain languages from the decision process or just
//! explicitly include relevant languages:
//!
//! ```
//! use lingua::{LanguageDetectorBuilder, Language, IsoCode639_1, IsoCode639_3};
//!
//! // Include all languages available in the library.
//! LanguageDetectorBuilder::from_all_languages();
//!
//! // Include only languages that are not yet extinct (= currently excludes Latin).
//! LanguageDetectorBuilder::from_all_spoken_languages();
//!
//! // Include only languages written with Cyrillic script.
//! LanguageDetectorBuilder::from_all_languages_with_cyrillic_script();
//!
//! // Exclude only the Spanish language from the decision algorithm.
//! LanguageDetectorBuilder::from_all_languages_without(&[Language::Spanish]);
//!
//! // Only decide between English and German.
//! LanguageDetectorBuilder::from_languages(&[Language::English, Language::German]);
//!
//! // Select languages by ISO 639-1 code.
//! LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::EN, IsoCode639_1::DE]);
//!
//! // Select languages by ISO 639-3 code.
//! LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::ENG, IsoCode639_3::DEU]);
//! ```
//!
//! ## 8. WebAssembly support
//!
//! This library can be compiled to [WebAssembly (WASM)](https://webassembly.org) which allows to
//! use *Lingua* in any JavaScript-based project, be it in the browser or in the back end running on
//! [Node.js](https://nodejs.org).
//!
//! The easiest way to compile is to use [`wasm-pack`](https://rustwasm.github.io/wasm-pack).
//! After the installation, you can, for instance, build the library with the web target so that it
//! can be directly used in the browser:
//!
//! ```shell
//! wasm-pack build --target web
//! ```
//!
//! By default, all 75 supported languages are included in the compiled wasm file which has a size
//! of 74 MB, approximately. If you only need a subset of certain languages, you can tell `wasm-pack`
//! which ones to include:
//!
//! ```shell
//! wasm-pack build --target web -- --no-default-features --features "french,italian,spanish"
//! ```
//!
//! The output of `wasm-pack` will be hosted in a
//! [separate repository](https://github.com/pemistahl/lingua-js) which allows to add further
//! JavaScript-related configuration, tests and documentation. *Lingua* will then be added to the
//! [npm registry](https://www.npmjs.com) as well, allowing for an easy download and installation
//! within every JavaScript or TypeScript project.

#[macro_use]
extern crate maplit;

#[cfg(test)]
use regex::Regex;

pub use builder::LanguageDetectorBuilder;
pub use detector::LanguageDetector;
pub use isocode::{IsoCode639_1, IsoCode639_3};
pub use language::Language;
pub use result::DetectionResult;
#[cfg(target_family = "wasm")]
pub use wasm::{
    ConfidenceValue, DetectionResult as WasmDetectionResult,
    LanguageDetectorBuilder as WasmLanguageDetectorBuilder,
};
pub use writer::{LanguageModelFilesWriter, TestDataFilesWriter};

mod alphabet;
mod builder;
mod constant;
mod detector;
mod fraction;
mod isocode;
mod json;
mod language;
mod model;
mod ngram;
mod result;
mod script;
mod writer;

#[cfg(feature = "python")]
mod python;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(any(target_family = "wasm", feature = "python"))]
pub(crate) fn convert_byte_indices_to_char_indices(
    results: &Vec<DetectionResult>,
    text: &str,
) -> Vec<DetectionResult> {
    let mut converted_results: Vec<DetectionResult> = vec![];

    for i in 0..results.len() {
        let result = results[i];
        let chars_count = text[result.start_index..result.end_index].chars().count();
        let start_index = if i == 0 {
            0
        } else {
            converted_results[i - 1].end_index
        };
        let end_index = start_index + chars_count;
        converted_results.push(DetectionResult {
            start_index,
            end_index,
            word_count: result.word_count,
            language: result.language,
        });
    }

    converted_results
}

#[cfg(test)]
pub(crate) fn minify(json: &str) -> String {
    let re = Regex::new("\n\\s*").unwrap();
    re.replace_all(json, "").to_string()
}
