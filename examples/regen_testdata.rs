use std::path::PathBuf;

use lingua::TestDataFilesWriter;

fn main() {
    let input = PathBuf::from("/tmp/venetian_clean.txt");
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("language-models/vec/testdata");

    println!("Generating test data from cleaned corpus...");
    TestDataFilesWriter::create_and_write_test_data_files(
        &input,
        &output,
        r"\p{L}",
        1000,
    )
    .expect("Failed to create test data");

    println!("Done! Created sentences.txt, single-words.txt, word-pairs.txt");
}
