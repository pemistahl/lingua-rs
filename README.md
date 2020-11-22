![lingua](logo.png)

<br>

[![Build Status](https://github.com/pemistahl/lingua-rs/workflows/Lingua%20CI/badge.svg?branch=master)](https://github.com/pemistahl/lingua-rs/actions?query=workflow%3A%22Lingua+CI%22+branch%3Amaster)
[![dependency status](https://deps.rs/crate/lingua/1.0.2/status.svg)](https://deps.rs/crate/lingua/1.0.2)
[![codecov](https://codecov.io/gh/pemistahl/lingua-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/pemistahl/lingua-rs)
[![supported languages](https://img.shields.io/badge/supported%20languages-74-green.svg)](#supported-languages)
[![Downloads](https://img.shields.io/crates/d/lingua.svg)](https://crates.io/crates/lingua)

[![Docs.rs](https://docs.rs/lingua/badge.svg)](https://docs.rs/lingua)
[![Crates.io](https://img.shields.io/crates/v/lingua.svg)](https://crates.io/crates/lingua)
[![Lib.rs](https://img.shields.io/badge/lib.rs-v1.0.2-blue)](https://lib.rs/crates/lingua)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

## <a name="table-of-contents"></a> Table of Contents

1. [What does this library do?](#library-purpose)
2. [Why does this library exist?](#library-reason)
3. [Which languages are supported?](#supported-languages)
4. [How good is it?](#library-accuracy)
5. [Why is it better than other libraries?](#why-is-it-better)   
6. [Test report generation](#report-generation)
7. [How to add it to your project?](#library-dependency)  
8. [How to build?](#library-build)
9. [How to use?](#library-use)  
10. [What's next for version 1.1.0?](#whats-next) 

## 1. <a name="library-purpose"></a> What does this library do? <sup>[Top ▲](#table-of-contents)</sup>

Its task is simple: It tells you which language some provided textual data is written in.
This is very useful as a preprocessing step for linguistic data in natural language
processing applications such as text classification and spell checking.
Other use cases, for instance, might include routing e-mails to the right geographically
located customer service department, based on the e-mails' languages.

## 2. <a name="library-reason"></a> Why does this library exist? <sup>[Top ▲](#table-of-contents)</sup>

Language detection is often done as part of large machine learning frameworks or natural
language processing applications. In cases where you don't need the full-fledged
functionality of those systems or don't want to learn the ropes of those,
a small flexible library comes in handy.

So far, the only other comprehensive open source library in the Rust ecosystem for
this task is [*Whatlang*](https://github.com/greyblake/whatlang-rs).
Unfortunately, it has two major drawbacks:

1. Detection only works with quite lengthy text fragments. For very short text snippets
such as Twitter messages, it does not provide adequate results.
2. The more languages take part in the decision process, the less accurate are the
detection results.

*Lingua* aims at eliminating these problems. It nearly does not need any configuration and
yields pretty accurate results on both long and short text, even on single words and phrases.
It draws on both rule-based and statistical methods but does not use any dictionaries of words.
It does not need a connection to any external API or service either.
Once the library has been downloaded, it can be used completely offline.

## 3. <a name="supported-languages"></a> Which languages are supported? <sup>[Top ▲](#table-of-contents)</sup>

Compared to other language detection libraries, *Lingua's* focus is on *quality over quantity*, that is, 
getting detection right for a small set of languages first before adding new ones. 
Currently, the following 74 languages are supported:

- A
  - Afrikaans
  - Albanian
  - Arabic
  - Armenian
  - Azerbaijani
- B
  - Basque
  - Belarusian
  - Bengali
  - Norwegian Bokmal
  - Bosnian
  - Bulgarian
- C
  - Catalan
  - Chinese
  - Croatian
  - Czech
- D
  - Danish
  - Dutch
- E
  - English
  - Esperanto
  - Estonian
- F
  - Finnish
  - French
- G
  - Ganda
  - Georgian
  - German
  - Greek
  - Gujarati
- H
  - Hebrew
  - Hindi
  - Hungarian
- I
  - Icelandic
  - Indonesian
  - Irish
  - Italian
- J
  - Japanese
- K
  - Kazakh
  - Korean
- L
  - Latin
  - Latvian
  - Lithuanian
- M
  - Macedonian
  - Malay
  - Marathi
  - Mongolian
- N
  - Norwegian Nynorsk
- P
  - Persian
  - Polish
  - Portuguese
  - Punjabi
- R
  - Romanian
  - Russian
- S
  - Serbian
  - Shona
  - Slovak
  - Slovene
  - Somali
  - Sotho
  - Spanish
  - Swahili
  - Swedish
- T
  - Tagalog
  - Tamil
  - Telugu
  - Thai
  - Tsonga
  - Tswana
  - Turkish
- U
  - Ukrainian
  - Urdu
- V
  - Vietnamese
- W
  - Welsh
- X
  - Xhosa
- Y
  - Yoruba
- Z
  - Zulu
  
## 4. <a name="library-accuracy"></a> How good is it? <sup>[Top ▲](#table-of-contents)</sup>

*Lingua* is able to report accuracy statistics for some bundled test data available for each
supported language. The test data for each language is split into three parts:

1. a list of single words with a minimum length of 5 characters
2. a list of word pairs with a minimum length of 10 characters
3. a list of complete grammatical sentences of various lengths

Both the language models and the test data have been created from separate documents of the
[Wortschatz corpora](https://wortschatz.uni-leipzig.de) offered by Leipzig University, Germany.
Data crawled from various news websites have been used for training, each corpus comprising one
million sentences. For testing, corpora made of arbitrarily chosen websites have been used,
each comprising ten thousand sentences. From each test corpus, a random unsorted subset of
1000 single words, 1000 word pairs and 1000 sentences has been extracted, respectively.

Given the generated test data, I have compared the detection results of *Lingua* and *Whatlang*
running over the data of *Lingua's* supported 74 languages. Languages that are not supported
by *Whatlang* are simply ignored for this library during the detection process.

The box plot below shows the distribution of the averaged accuracy values for all three performed tasks: 
Single word detection, word pair detection and sentence detection. *Lingua* clearly outperforms its contender.
Bar plots for each language and further box plots for the separate detection tasks can be found in the file 
[ACCURACY_PLOTS.md](https://github.com/pemistahl/lingua-rs/blob/master/ACCURACY_PLOTS.md). 
Detailed statistics including mean, median and standard deviation values for each language and classifier are 
available in the file 
[ACCURACY_TABLE.md](https://github.com/pemistahl/lingua-rs/blob/master/ACCURACY_TABLE.md).

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/master/images/plots/boxplot-average.png" alt="Average Detection Performance" />

## 5. <a name="why-is-it-better"></a> Why is it better than other libraries? <sup>[Top ▲](#table-of-contents)</sup>

Every language detector uses a probabilistic [n-gram](https://en.wikipedia.org/wiki/N-gram) model trained on the 
character distribution in some training corpus. Most libraries only use n-grams of size 3 (trigrams) which is 
satisfactory for detecting the language of longer text fragments consisting of multiple sentences. For short 
phrases or single words, however, trigrams are not enough. The shorter the input text is, the less n-grams are 
available. The probabilities estimated from such few n-grams are not reliable. This is why *Lingua* makes use 
of n-grams of sizes 1 up to 5 which results in much more accurate prediction of the correct language.  

A second important difference is that *Lingua* does not only use such a statistical model, but also a rule-based 
engine. This engine first determines the alphabet of the input text and searches for characters which are unique 
in one or more languages. If exactly one language can be reliably chosen this way, the statistical model is not 
necessary anymore. In any case, the rule-based engine filters out languages that do not satisfy the conditions 
of the input text. Only then, in a second step, the probabilistic n-gram model is taken into consideration. 
This makes sense because loading less language models means less memory consumption and better runtime performance.

In general, it is always a good idea to restrict the set of languages to be considered in the classification process 
using the respective [api methods](#library-use). If you know beforehand that certain languages are 
never to occur in an input text, do not let those take part in the classifcation process. The filtering mechanism 
of the rule-based engine is quite good, however, filtering based on your own knowledge of the input text is always preferable.

## 6. <a name="report-generation"></a> Test report generation <sup>[Top ▲](#table-of-contents)</sup>

If you want to reproduce the accuracy results above, you can generate the test reports yourself for both classifiers 
and all languages by doing:

    cargo run --release --example accuracy_reports
    
It is important to use the `--release` flag here because loading the language models in debug mode takes too much time. 
For each detector and language, a test report file is then written into 
[`/accuracy-reports`](https://github.com/pemistahl/lingua-rs/tree/master/accuracy-reports), 
to be found next to the `src` directory. As an example, here is the current output of the *Lingua* German report:

```
##### German #####

>>> Accuracy on average: 89.1%

>> Detection of 1000 single words (average length: 9 chars)
Accuracy: 73.6%
Erroneously classified as Dutch: 2.3%, Danish: 2.1%, English: 2.1%, Latin: 2%, Bokmal: 1.6%, Basque: 1.2%, French: 1.2%, Italian: 1.2%, Esperanto: 1.1%, Swedish: 1%, Afrikaans: 0.8%, Tsonga: 0.7%, Nynorsk: 0.6%, Portuguese: 0.6%, Estonian: 0.5%, Finnish: 0.5%, Sotho: 0.5%, Welsh: 0.5%, Yoruba: 0.5%, Icelandic: 0.4%, Irish: 0.4%, Polish: 0.4%, Spanish: 0.4%, Swahili: 0.4%, Tswana: 0.4%, Bosnian: 0.3%, Catalan: 0.3%, Tagalog: 0.3%, Albanian: 0.2%, Croatian: 0.2%, Indonesian: 0.2%, Lithuanian: 0.2%, Romanian: 0.2%, Slovak: 0.2%, Xhosa: 0.2%, Zulu: 0.2%, Latvian: 0.1%, Malay: 0.1%, Slovene: 0.1%, Somali: 0.1%, Turkish: 0.1%

>> Detection of 1000 word pairs (average length: 18 chars)
Accuracy: 94%
Erroneously classified as Dutch: 0.9%, Latin: 0.8%, English: 0.7%, Swedish: 0.6%, Danish: 0.5%, French: 0.4%, Bokmal: 0.3%, Irish: 0.2%, Swahili: 0.2%, Tagalog: 0.2%, Afrikaans: 0.1%, Esperanto: 0.1%, Estonian: 0.1%, Finnish: 0.1%, Icelandic: 0.1%, Italian: 0.1%, Nynorsk: 0.1%, Somali: 0.1%, Tsonga: 0.1%, Turkish: 0.1%, Welsh: 0.1%, Zulu: 0.1%

>> Detection of 1000 sentences (average length: 112 chars)
Accuracy: 99.7%
Erroneously classified as Dutch: 0.2%, Latin: 0.1%
```

## 7. <a name="library-dependency"></a> How to add it to your project? <sup>[Top ▲](#table-of-contents)</sup>

Add *Lingua* to your `Cargo.toml` file like so:

```toml
[dependencies]
lingua = "1.0.2"
```

## 8. <a name="library-build"></a> How to build? <sup>[Top ▲](#table-of-contents)</sup>

In order to build the source code yourself, you need the 
[stable Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine 
so that [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager is available.

```
git clone https://github.com/pemistahl/lingua-rs.git
cd lingua-rs
cargo build
```

The source code is accompanied by an extensive unit test suite. To run them, simply say:

    cargo test --lib
    
## 9. <a name="library-use"></a> How to use? <sup>[Top ▲](#table-of-contents)</sup>

### 9.1 Basic usage

```rust
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, French, German, Spanish};

let languages = vec![English, French, German, Spanish];
let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
let detected_language: Option<Language> = detector.detect_language_of("languages are awesome");

assert_eq!(detected_language, Some(English));
```

All instances of `LanguageDetector` within a single application share the same language models 
and have synchronized access to them. So you can safely have multiple instances without worrying
about consuming too much memory.

### 9.2 Minimum relative distance

By default, *Lingua* returns the most likely language for a given input text. However, there are
certain words that are spelled the same in more than one language. The word *prologue*, for
instance, is both a valid English and French word. *Lingua* would output either English or
French which might be wrong in the given context. For cases like that, it is possible to
specify a minimum relative distance that the logarithmized and summed up probabilities for
each possible language have to satisfy. It can be stated in the following way:

```rust
use lingua::LanguageDetectorBuilder;
use lingua::Language::{English, French, German, Spanish};

let detector = LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
    .with_minimum_relative_distance(0.25) // minimum: 0.00 maximum: 0.99 default: 0.00
    .build();
let detected_language = detector.detect_language_of("languages are awesome");

assert_eq!(detected_language, None);
```

Be aware that the distance between the language probabilities is dependent on the length of the
input text. The longer the input text, the larger the distance between the languages. So if you
want to classify very short text phrases, do not set the minimum relative distance too high.
Otherwise [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) will be
returned most of the time as in the example above. This is the return value for cases where
language detection is not reliably possible.

### 9.3 Confidence values

Knowing about the most likely language is nice but how reliable is the computed likelihood?
And how less likely are the other examined languages in comparison to the most likely one?
These questions can be answered as well:

```rust
use lingua::{LanguageDetectorBuilder, Language};
use lingua::Language::{English, French, German, Spanish};
use float_cmp::approx_eq;

let languages = vec![English, French, German, Spanish];
let detector = LanguageDetectorBuilder::from_languages(&languages).build();
let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(
    "languages are awesome"
);

// The more readable version of the assertions below:
// assert_eq!(
//     confidence_values,
//     vec![(English, 1.0), (French, 0.79), (German, 0.75), (Spanish, 0.72)]
// );

assert_eq!(confidence_values[0], (English, 1.0_f64));

assert_eq!(confidence_values[1].0, French);
assert!(approx_eq!(f64, confidence_values[1].1, 0.7917282993701181, ulps = 2));

assert_eq!(confidence_values[2].0, German);
assert!(approx_eq!(f64, confidence_values[2].1, 0.7532048914992281, ulps = 2));

assert_eq!(confidence_values[3].0, Spanish);
assert!(approx_eq!(f64, confidence_values[3].1, 0.7229637749926444, ulps = 2));
```

In the example above, a vector of all possible languages is returned, sorted by their confidence
value in descending order. The values that the detector computes are part of a **relative**
confidence metric, not of an absolute one. Each value is a number between 0.0 and 1.0.
The most likely language is always returned with value 1.0. All other languages get values
assigned which are lower than 1.0, denoting how less likely those languages are in comparison
to the most likely language.

The vector returned by this method does not necessarily contain all languages which the calling
instance of `LanguageDetector` was built from.
If the rule-based engine decides that a specific language is truly impossible, then it will not
be part of the returned vector. Likewise, if no ngram probabilities can be found within the
detector's languages for the given input text, the returned vector will be empty. The confidence
value for each language not being part of the returned vector is assumed to be 0.0.

### 9.4 Methods to build the LanguageDetector

There might be classification tasks where you know beforehand that your language data is
definitely not written in Latin, for instance (what a surprise :-). The detection accuracy can
become better in such cases if you exclude certain languages from the decision process or just
explicitly include relevant languages:

```rust
use lingua::{LanguageDetectorBuilder, Language, IsoCode639_1, IsoCode639_3};

// Including all languages available in the library
// consumes approximately 2GB of memory and might
// lead to slow runtime performance.
LanguageDetectorBuilder::from_all_languages();

// Include only languages that are not yet extinct (= currently excludes Latin).
LanguageDetectorBuilder::from_all_spoken_languages();

// Include only languages written with Cyrillic script.
LanguageDetectorBuilder::from_all_languages_with_cyrillic_script();

// Exclude only the Spanish language from the decision algorithm.
LanguageDetectorBuilder::from_all_languages_without(&[Language::Spanish]);

// Only decide between English and German.
LanguageDetectorBuilder::from_languages(&[Language::English, Language::German]);

// Select languages by ISO 639-1 code.
LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::EN, IsoCode639_1::DE]);

// Select languages by ISO 639-3 code.
LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::ENG, IsoCode639_3::DEU]);
```

## 10. <a name="whats-next"></a> What's next for version 1.1.0? <sup>[Top ▲](#table-of-contents)</sup>

Take a look at the [planned issues](https://github.com/pemistahl/lingua-rs/milestone/1).