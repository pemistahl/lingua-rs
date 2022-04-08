## You want to contribute to Lingua? That's great!

In case you want to contribute something to *Lingua*, then I encourage you to do so. Do you have ideas for 
improving the API? Are there some specific languages that you want to have supported early? Or have you 
found any bugs so far? Feel free to open an issue or send a pull request. It's very much appreciated.

For pull requests, please make sure that all unit tests pass and that the code is formatted according to
the official Rust style guide with `cargo fmt`.

All kinds of pull requests are welcome. The pull requests I favor the most are new language additions. If you want
to contribute new languages to *Lingua*, here comes a detailed manual explaining how to accomplish that.

Thank you very much in advance for all contributions, however small they may be.

### How to add new languages?

1. Clone *Lingua's* repository to your own computer as described in README's [section 8][library build url].
2. Open enums [`IsoCode639_1`][isocode639_1 url] and [`IsoCode639_3`][isocode639_3 url] and add the 
language's iso codes. Among other sites, Wikipedia provides a [comprehensive list][wikipedia isocodes list].
3. Open enum [`Language`][language url] and add a new entry for your language. If the language is written
with a script that is not yet supported by *Lingua's* [`Alphabet`][alphabet url] enum, then add a new entry
for it there as well.
4. If your language's script contains characters that are completely unique to it, then add them to the
respective method in the [`Language`][language method url] enum. However, if the characters occur in more 
than one language **but** not in all languages, then add them to the 
[`CHARS_TO_LANGUAGES_MAPPING`][chars to languages mapping url] constant instead.
5. Use [`LanguageModelFilesWriter`][language model files writer url] to create the language model files.
The training data file used for ngram probability estimation is not required to have a specific format
other than to be a valid txt file with UTF-8 encoding. Do **not** rename the language model files.
6. Use [`TestDataFilesWriter`][test data files writer url] to create the test data files used for
accuracy report generation. The input file from which to create the test data should have each
sentence on a separate line. Do **not** rename the test data files.
7. Create a new crate in [`/language-models`][language models directory url]. Add a subdirectory
named after the new language's ISO 639-1 code and put the crate's content in there, including
the language model files and the test data files. Look at the other languages' crates to see
how it looks like. It should be pretty self-explanatory.
8. Add the new crate as an optional dependency to the main [`Cargo.toml`][cargo toml url].
Do not forget to add a separate Cargo feature for the new language as well.
9. Add the new language to the functions in [`/src/json.rs`][json rs url] and 
[`/src/bin/accuracy_reports.rs`][accuracy reports url] so that *Lingua* can find the language
model and test data directories.
10. Fix the existing unit tests by adding your new language.
11. For accuracy report generation, run `cargo run --release --bin accuracy_reports --features accuracy-reports`.
12. Be happy! :-) You have successfully contributed a new language and have thereby significantly widened
this library's fields of application. 

[library build url]: https://github.com/pemistahl/lingua-rs#8-how-to-build
[isocode639_1 url]: https://github.com/pemistahl/lingua-rs/blob/main/src/isocode.rs#L26
[isocode639_3 url]: https://github.com/pemistahl/lingua-rs/blob/main/src/isocode.rs#L334
[wikipedia isocodes list]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[language url]: https://github.com/pemistahl/lingua-rs/blob/main/src/language.rs#L30
[language method url]: https://github.com/pemistahl/lingua-rs/blob/main/src/language.rs#L777
[alphabet url]: https://github.com/pemistahl/lingua-rs/blob/main/src/alphabet.rs#L25
[chars to languages mapping url]: https://github.com/pemistahl/lingua-rs/blob/main/src/constant.rs#L44
[language model files writer url]: https://github.com/pemistahl/lingua-rs/blob/main/src/writer.rs#L38
[language models directory url]: https://github.com/pemistahl/lingua-rs/tree/main/language-models
[test data files writer url]: https://github.com/pemistahl/lingua-rs/blob/main/src/writer.rs#L172
[test data directory url]: https://github.com/pemistahl/lingua-rs/tree/main/assets/test/language-testdata
[cargo toml url]: https://github.com/pemistahl/lingua-rs/blob/main/Cargo.toml
[json rs url]: https://github.com/pemistahl/lingua-rs/blob/main/src/json.rs#L262
[accuracy reports url]: https://github.com/pemistahl/lingua-rs/blob/main/src/bin/accuracy_reports.rs
