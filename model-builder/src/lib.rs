use common::{fraction::Fraction, ngram::Ngram};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};
use zip::ZipArchive;

/// Generates rkyv files from the JSON language models in the given directory.
pub fn build_frequencies(models_dir: impl AsRef<Path>) {
    let models_dir = models_dir.as_ref();

    for n in 1..=5 {
        let ngram_name = Ngram::find_ngram_name_by_length(n);

        let json = load_json(models_dir, ngram_name);
        let frequencies = extract_frequencies(&json);
        let bytes = rkyv::to_bytes::<_, 256>(&frequencies).unwrap();

        let file_path = models_dir.join(format!("{}s.bin", ngram_name));
        let mut file = BufWriter::new(File::create(file_path).unwrap());
        file.write_all(&bytes).unwrap();
    }
}

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

/// Reads the JSON language model for the given n-gram and directory.
fn load_json(models_dir: &Path, ngram_name: &str) -> String {
    let file_path = models_dir.join(format!("{}s.json.zip", ngram_name));
    let file_reader = BufReader::new(File::open(file_path).unwrap());

    let mut archive = ZipArchive::new(file_reader).unwrap();
    let mut json_file = archive.by_index(0).unwrap();

    let mut json = String::new();
    json_file.read_to_string(&mut json).unwrap();

    json
}

/// Turns the language model JSON into the `HashMap` of frequencies that
/// `TrainingDataLanguageModel` uses internally.
fn extract_frequencies(json: &str) -> HashMap<Ngram, f64> {
    let json_language_model = serde_json::from_str::<JsonLanguageModel>(json).unwrap();
    let mut json_relative_frequencies = HashMap::new();

    for (fraction, ngrams) in json_language_model.ngrams {
        let floating_point_value = fraction.to_f64();
        for ngram in ngrams.split(' ') {
            json_relative_frequencies.insert(Ngram::new(ngram), floating_point_value);
        }
    }

    json_relative_frequencies
}

/// A copy of the original JSON language model with just the parts that we use here.
#[derive(Deserialize)]
struct JsonLanguageModel {
    ngrams: BTreeMap<Fraction, String>,
}
