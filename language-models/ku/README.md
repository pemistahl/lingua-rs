## Kurdish language model for Lingua

This is the language model for the Kurdish language which is used by
[*Lingua*](https://github.com/pemistahl/lingua-rs),
the most accurate natural language detection library in the Rust ecosystem.

### Changelog

#### Version 1.3.0

- Initial release of the Kurdish language model. The language model files
  are stored as finite-state transducers (FSTs) which reduces memory
  consumption drastically at the cost of a slightly slower runtime performance.
