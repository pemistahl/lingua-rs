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

//! ## 1. What does this library do?
//!
//! Its task is simple: It tells you which language some provided textual data is written in.
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
//! So far, the only other comprehensive open source library in the Rust ecosystem for
//! this task is [*Whatlang*](https://github.com/greyblake/whatlang-rs).
//! Unfortunately, it has two major drawbacks:
//!
//! 1. Detection only works with quite lengthy text fragments. For very short text snippets
//! such as Twitter messages, it does not provide adequate results.
//! 2. The more languages take part in the decision process, the less accurate are the
//! detection results.
//!
//! *Lingua* aims at eliminating these problems. It nearly does not need any configuration and
//! yields pretty accurate results on both long and short text, even on single words and phrases.
//! It draws on both rule-based and statistical methods but does not use any dictionaries of words.
//! It does not need a connection to any external API or service either.
//! Once the library has been downloaded, it can be used completely offline.
//!
//! ## 3. Which languages are supported?
//!
//! Compared to other language detection libraries, *Lingua's* focus is on *quality over quantity*,
//! that is, getting detection right for a small set of languages first before adding new ones.
//! Currently, 74 languages are supported. They are listed as variants in the
//! [`Language`](./enum.Language.html) enum.
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
//! Given the generated test data, I have compared the detection results of *Lingua* and *Whatlang*
//! running over the data of *Lingua's* supported 74 languages. Languages that are not supported
//! by *Whatlang* are simply ignored for this library during the detection process.
//!
//! The [bar and box plots](https://github.com/pemistahl/lingua-rs/blob/master/ACCURACY_PLOTS.md)
//! show the measured accuracy values for all three performed tasks: Single word detection,
//! word pair detection and sentence detection. *Lingua* clearly outperforms its contender.
//! Detailed statistics including mean, median and standard deviation values for each language
//! and classifier are available in
//! [tabular form](https://github.com/pemistahl/lingua-rs/blob/master/ACCURACY_TABLE.md) as well.
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
//! languages are never to occur in an input text, do not let those take part in the classifcation
//! process. The filtering mechanism of the rule-based engine is quite good, however, filtering
//! based on your own knowledge of the input text is always preferable.
//!
//! ## 6. How to use?
//!
//! ### 6.1 Basic usage
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
//! ### 6.2 Minimum relative distance
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
//!     .with_minimum_relative_distance(0.25) // minimum: 0.00 maximum: 0.99 default: 0.00
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
//! ### 6.3 Confidence values
//!
//! Knowing about the most likely language is nice but how reliable is the computed likelihood?
//! And how less likely are the other examined languages in comparison to the most likely one?
//! These questions can be answered as well:
//!
//! ```
//! use lingua::{LanguageDetectorBuilder, Language};
//! use lingua::Language::{English, French, German, Spanish};
//! use float_cmp::approx_eq;
//!
//! let languages = vec![English, French, German, Spanish];
//! let detector = LanguageDetectorBuilder::from_languages(&languages).build();
//! let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(
//!     "languages are awesome"
//! );
//!
//! // The more readable version of the assertions below:
//! // assert_eq!(
//! //     confidence_values,
//! //     vec![(English, 1.0), (French, 0.79), (German, 0.75), (Spanish, 0.72)]
//! // );
//!
//! assert_eq!(confidence_values[0], (English, 1.0_f64));
//!
//! assert_eq!(confidence_values[1].0, French);
//! assert!(approx_eq!(f64, confidence_values[1].1, 0.7917282993701181, ulps = 2));
//!
//! assert_eq!(confidence_values[2].0, German);
//! assert!(approx_eq!(f64, confidence_values[2].1, 0.7532048914992281, ulps = 2));
//!
//! assert_eq!(confidence_values[3].0, Spanish);
//! assert!(approx_eq!(f64, confidence_values[3].1, 0.7229637749926444, ulps = 2));
//! ```
//!
//! In the example above, a vector of all possible languages is returned, sorted by their confidence
//! value in descending order. The values that the detector computes are part of a **relative**
//! confidence metric, not of an absolute one. Each value is a number between 0.0 and 1.0.
//! The most likely language is always returned with value 1.0. All other languages get values
//! assigned which are lower than 1.0, denoting how less likely those languages are in comparison
//! to the most likely language.
//!
//! The vector returned by this method does not necessarily contain all languages which the calling
//! instance of [`LanguageDetector`](./struct.LanguageDetector.html) was built from.
//! If the rule-based engine decides that a specific language is truly impossible, then it will not
//! be part of the returned vector. Likewise, if no ngram probabilities can be found within the
//! detector's languages for the given input text, the returned vector will be empty. The confidence
//! value for each language not being part of the returned vector is assumed to be 0.0.
//!
//! ### 6.4 Methods to build the LanguageDetector
//!
//! There might be classification tasks where you know beforehand that your language data is
//! definitely not written in Latin, for instance (what a surprise :-). The detection accuracy can
//! become better in such cases if you exclude certain languages from the decision process or just
//! explicitly include relevant languages:
//!
//! ```
//! use lingua::{LanguageDetectorBuilder, Language, IsoCode639_1, IsoCode639_3};
//!
//! // Including all languages available in the library
//! // consumes approximately 2GB of memory and might
//! // lead to slow runtime performance.
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

#[macro_use]
extern crate maplit;

mod alphabet;
mod builder;
mod constant;
mod detector;
mod fraction;
mod isocode;
mod language;
mod model;
mod models;
mod ngram;
mod writer;

pub use builder::LanguageDetectorBuilder;
pub use detector::LanguageDetector;
pub use isocode::{IsoCode639_1, IsoCode639_3};
pub use language::Language;
pub use writer::{LanguageModelFilesWriter, TestDataFilesWriter};

#[cfg(test)]
use regex::Regex;

#[cfg(test)]
pub(crate) fn minify(json: &str) -> String {
    let re = Regex::new("\n\\s*").unwrap();
    re.replace_all(json, "").to_string()
}
