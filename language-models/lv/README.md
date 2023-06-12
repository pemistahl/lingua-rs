## Latvian language model for Lingua

This is the language model for the Latvian language which is used by 
[*Lingua*](https://github.com/pemistahl/lingua-rs), 
the most accurate natural language detection library in the Rust ecosystem.

### Changelog

#### Version 1.1.0

- The language model files are now compressed with the Brotli algorithm which
  reduces the file size by 15 %, on average.

#### Version 1.0.1

- Some characters in the test data files were corrupted. This has been corrected.
