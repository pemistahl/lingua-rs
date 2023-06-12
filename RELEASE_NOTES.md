## Lingua 1.5.0 (released on 13 Jun 2023)

### Features

- The new method `LanguageDetector.detect_multiple_languages_of()` has been
  introduced. It allows to detect multiple languages in mixed-language text. (#1)

- The new method `LanguageDetectorBuilder.with_low_accuracy_mode()` has been
  introduced. By activating it, detection accuracy for short text is reduced
  in favor of a smaller memory footprint and faster detection performance. (#119)

- The new method `LanguageDetector.compute_language_confidence()` has been
  introduced. It allows to retrieve the confidence value for one specific
  language only, given the input text. (#102)

### Improvements

- The computation of the confidence values has been revised and the softmax function
  is now applied to the values, making them better comparable by behaving more like 
  real probabilities. (#120)

- The WASM API has been revised. Now it makes use of the same builder pattern
  as the Rust API. (#122)

- The language model files are now compressed with the Brotli algorithm which
  reduces the file size by 15 %, on average. (#189)

- Several performance optimizations have been applied which makes the library
  nearly twice as fast as the previous version. Big thanks go out to @serega
  and @koute for their help. (#82, #148, #177)

- The enums `IsoCode639_1` and `IsoCode639_3` now implement some new traits
  such as `Copy`, `Hash` and Serde's `Serialize` and `Deserialize`. The enum
  `Language` now implements `Copy` as well. (#175)

## Lingua 1.4.0 (released on 08 Apr 2022)

### Features

- The library can now be compiled to WebAssembly and be used in any
  JavaScript project. Big thanks to @martindisch for bringing this
  forward. (#14)

### Improvements

- Some minor performance tweaks have been applied to the rule engine.

## Lingua 1.3.3 (released on 22 Feb 2022)

### Bug Fixes

- This release updates outdated dependencies and fixes an incompatibility
  between different versions of the `include_dir` crate which are used
  in the main `lingua` crate and the language model crates.

## Lingua 1.3.2 (released on 19 Oct 2021)

### Bug Fixes

- Another compilation error has been fixed which occurred when the
  Latin language was left out as Cargo feature.

## Lingua 1.3.1 (released on 19 Oct 2021)

### Bug Fixes

- When Chinese, Japanese or Korean were left out as Cargo features,
  there were compilation errors. This has been fixed.

## Lingua 1.3.0 (released on 19 Oct 2021)

### Features

- The language model dependencies are separate Cargo features now.
  Users can decide which languages shall be downloaded and used in
  the library. (#12)

### Improvements

- The code that does the lazy-loading of the language models has been
  refactored significantly, making the code more stable and less error-prone.

### Bug Fixes

- In very rare cases, the language returned by the detector was non-deterministic. 
  This has been fixed. Big thanks to @asg0451 for identifying this problem. (#17)

## Lingua 1.2.2 (released on 2 Jun 2021)

### Features

- The enums `Language`, `IsoCode639_1` and `IsoCode639_3` now implement
  [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
  in order to instantiate enum variants by string values. This comes in
  handy for JavaScript bindings and the like. (#15)
  
### Improvements

- The performance of preloading the language models has been improved.

### Bug Fixes

- Language detection for sentences with more than 120 characters
  was supposed to be done by iterating through trigrams only but
  this was never the case. This has been corrected.

## Lingua 1.2.1 (released on 8 May 2021)

### Improvements

- Language detection for sentences with more than 120 characters now
  performs more quickly by iterating through trigrams only which is
  enough to achieve high detection accuracy.
- Textual input that includes logograms from Chinese, Japanese or Korean
  is now split at each logogram and not only at whitespace. This
  provides for more reliable language detection for sentences that
  include multi-language content.
  
### Bug Fixes

- Errors in the rule engine for the Latvian language have been resolved.
- Corrupted characters in the Latvian test data have been corrected.

## Lingua 1.2.0 (released on 8 Apr 2021)

### Features

- A `LanguageDetector` can now be built with lazy-loading required language models
on demand (default) or with preloading all language models at once by calling
`LanguageDetectorBuilder.with_preloaded_language_models()`. (#10)  

## Lingua 1.1.0 (released on 31 Jan 2021)

### Languages

- The Maori language is now supported.
Thanks to @eekkaiia for the contribution. (#5)

### Performance

- Loading and searching the language models has been quite slow so far.
Using parallel iterators from the [Rayon](https://github.com/rayon-rs/rayon)
library, this process is now at least 50% faster, depending on how many
CPU cores are available. (#8)
  
### Accuracy Reports

- Accuracy reports are now also generated for the [*CLD2*](https://github.com/emk/rust-cld2)
library and included in the language detector comparison plots. (#6)
  
## Lingua 1.0.3 (released on 15 Jan 2021)

### Bug Fixes

- Lingua could not be used within other projects because of a private
serde module that was accidentally tried to be exposed.
Thanks to @luananama for reporting this bug. (#9)  

## Lingua 1.0.2 (released on 22 Nov 2020)

### Bug Fixes

- Accidentally, bug #3 was only partially fixed. This has been corrected.

## Lingua 1.0.1 (released on 22 Nov 2020)

### Bug Fixes

- When trying to create new language models, the `LanguageModelFilesWriter` panicked
when it recognized characters in a text corpus that consist of multiple bytes.
Thanks to @eekkaiia for reporting this bug. (#3)

## Lingua 1.0.0 (released on 21 Nov 2020)

This is the very first release of Lingua for Rust. 
Took me 5 months of hard work in my free time. 
Hope you find it useful. :)
