use common::ngram::Ngram;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

/// Generates `generated_statics.rs` in the given directory containing includes of the rkyv data.
pub fn generate_statics(model_dir_relative: &str, src_dir: impl AsRef<Path>) {
    let file_path = src_dir.as_ref().join("generated_statics.rs");
    let mut file = BufWriter::new(File::create(file_path).unwrap());

    writeln!(
        &mut file,
        "//! Auto-generated static data containing n-gram hash maps.\n"
    )
    .unwrap();

    for n in 1..=5 {
        let ngram_name = Ngram::find_ngram_name_by_length(n);

        writeln!(
            &mut file,
            r#"static {}S: &[u8] = include_bytes!("{}/{}s.bin");"#,
            ngram_name.to_uppercase(),
            model_dir_relative,
            ngram_name
        )
        .unwrap();
    }

    writeln!(
        &mut file,
        "\npub static NGRAMS: [&[u8]; 5] = [UNIGRAMS, BIGRAMS, TRIGRAMS, QUADRIGRAMS, FIVEGRAMS];"
    )
    .unwrap();
}
