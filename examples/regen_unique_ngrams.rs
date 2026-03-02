use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use lingua::{Language, MostCommonNgramsWriter, UniqueNgramsWriter};

fn main() {
    // UniqueNgramsWriter creates a subdirectory named by the language's ISO 639-1 code.
    // Venetian has no ISO 639-1 code, so it gets "zz/". We use a staging directory
    // and then copy the files into our models directory.
    let staging = PathBuf::from("/tmp/lingua-vec-staging");
    let models_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("language-models/vec/models");

    // Prepare staging directory
    if staging.exists() {
        fs::remove_dir_all(&staging).expect("Failed to clean staging dir");
    }
    fs::create_dir_all(&staging).expect("Failed to create staging dir");

    println!("Generating unique ngrams for all languages (this compares Venetian against all others)...");
    UniqueNgramsWriter::create_and_write_unique_ngram_files(&staging)
        .expect("Failed to create unique ngrams");

    // Copy zz/unique-ngrams.fst to models dir
    let src = staging.join("zz/unique-ngrams.fst");
    let dst = models_dir.join("unique-ngrams.fst");
    fs::copy(&src, &dst).unwrap_or_else(|e| panic!("Failed to copy {:?} -> {:?}: {}", src, dst, e));
    println!("Copied {:?} -> {:?}", src, dst);

    println!("Generating most common ngrams for Venetian...");
    let languages: HashSet<Language> = [Language::Venetian].into_iter().collect();
    MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
        &staging,
        &languages,
        10,
    )
    .expect("Failed to create most common ngrams");

    // Copy zz/mostcommon-ngrams.fst to models dir
    let src = staging.join("zz/mostcommon-ngrams.fst");
    let dst = models_dir.join("mostcommon-ngrams.fst");
    fs::copy(&src, &dst).unwrap_or_else(|e| panic!("Failed to copy {:?} -> {:?}: {}", src, dst, e));
    println!("Copied {:?} -> {:?}", src, dst);

    // Clean up staging
    fs::remove_dir_all(&staging).ok();

    println!("Done!");
}
