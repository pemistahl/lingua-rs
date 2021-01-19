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