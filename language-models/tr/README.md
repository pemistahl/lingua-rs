## Turkish language model for Lingua

This is the language model for the Turkish language which is used by 
[*Lingua*](https://github.com/pemistahl/lingua-rs), 
the most accurate natural language detection library in the Rust ecosystem.

### Changelog

#### Version 1.2.0

- The language model has been enhanced by including unique and most common
  ngrams to support an absolute confidence metric which is independent of
  other languages.

#### Version 1.1.0

- The language model files are now compressed with the Brotli algorithm which
  reduces the file size by 15 %, on average.
