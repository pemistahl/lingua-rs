use std::path::PathBuf;

use lingua::{
    Language, LanguageModelFilesWriter, MostCommonNgramsWriter, TestDataFilesWriter,
    UniqueNgramsWriter,
};

fn lang_meta(code: &str) -> (Language, &'static str, &'static str) {
    match code {
        "as" => (Language::Assamese, "as", "\\p{Bengali}"),
        "kn" => (Language::Kannada, "kn", "\\p{Kannada}"),
        // Kurdish: Latin (Kurmanji) + Arabic (Sorani / Southern Kurdish)
        "ku" => (Language::Kurdish, "ku", "\\p{L}"),
        "lo" => (Language::Lao, "lo", "\\p{Lao}"),
        "ml" => (Language::Malayalam, "ml", "\\p{Malayalam}"),
        "uz" => (Language::Uzbek, "uz", "\\p{L}"),
        other => panic!("unknown code: {other}"),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args
        .next()
        .expect("mode required: testdata | model | unique | mostcommon");
    let code = args.next().expect("language code required");
    let input_file = PathBuf::from(args.next().expect("input file required"));
    let repo_root = PathBuf::from(args.next().unwrap_or_else(|| {
        std::env::current_dir().unwrap().to_string_lossy().to_string()
    }));

    let (language, dir_code, char_class) = lang_meta(&code);
    let lang_dir = repo_root.join("language-models").join(dir_code);
    let models_dir = lang_dir.join("models");
    let testdata_dir = lang_dir.join("testdata");
    std::fs::create_dir_all(&models_dir).unwrap();
    std::fs::create_dir_all(&testdata_dir).unwrap();

    match mode.as_str() {
        "testdata" => {
            println!("Writing test data files for {code} to {}", testdata_dir.display());
            TestDataFilesWriter::create_and_write_test_data_files(
                input_file.as_path(),
                testdata_dir.as_path(),
                char_class,
                1000,
            )
            .expect("failed writing test data");
        }
        "model" => {
            println!("Writing language model file for {code} to {}", models_dir.display());
            LanguageModelFilesWriter::create_and_write_language_model_files(
                input_file.as_path(),
                models_dir.as_path(),
                language,
                char_class,
            )
            .expect("failed writing language model files");
        }
        "unique" => {
            let staging = std::env::temp_dir().join("lingua_unique_staging");
            if staging.exists() {
                std::fs::remove_dir_all(&staging).unwrap();
            }
            std::fs::create_dir_all(&staging).unwrap();
            println!("Writing unique ngram files to staging dir {}", staging.display());
            UniqueNgramsWriter::create_and_write_unique_ngram_files(staging.as_path())
                .expect("failed writing unique ngrams");

            let src = staging.join(language.iso_code_639_1().to_string()).join("unique-ngrams.fst");
            if src.exists() {
                std::fs::copy(&src, models_dir.join("unique-ngrams.fst")).unwrap();
                println!("Copied unique-ngrams.fst");
            } else {
                eprintln!("No unique ngrams produced for {code}");
            }
        }
        "mostcommon" => {
            let staging = std::env::temp_dir().join("lingua_mostcommon_staging");
            if staging.exists() {
                std::fs::remove_dir_all(&staging).unwrap();
            }
            std::fs::create_dir_all(&staging).unwrap();
            let languages = std::collections::HashSet::from([language]);
            println!("Writing most-common ngram files for {code}");
            MostCommonNgramsWriter::create_and_write_most_common_ngram_files(
                staging.as_path(),
                &languages,
                25,
            )
            .expect("failed writing most-common ngrams");
            let src = staging.join(language.iso_code_639_1().to_string()).join("mostcommon-ngrams.fst");
            if src.exists() {
                std::fs::copy(&src, models_dir.join("mostcommon-ngrams.fst")).unwrap();
                println!("Copied mostcommon-ngrams.fst");
            } else {
                eprintln!("No most-common ngrams produced for {code}");
            }
        }
        other => panic!("unknown mode: {other}"),
    }
    println!("Done.");
}
