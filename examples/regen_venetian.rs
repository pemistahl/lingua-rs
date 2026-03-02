use std::path::PathBuf;

use lingua::{Language, LanguageModelFilesWriter};

fn main() {
    let input = PathBuf::from("/tmp/venetian_clean.txt");
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("language-models/vec/models");

    println!("Input: {:?}", input);
    println!("Output: {:?}", output);

    LanguageModelFilesWriter::create_and_write_language_model_files(
        &input,
        &output,
        Language::Venetian,
        r"\p{L}",
    )
    .expect("Failed to create models");

    println!("Done! Created ngrams.fst");
}
