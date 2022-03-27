use common::ngram::Ngram;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

/// Generates `generated_statics.rs` in the given directory containing includes of the rkyv data.
pub fn generate_statics(models_dir: impl AsRef<Path>, src_dir: impl AsRef<Path>) {
    let file_path = src_dir.as_ref().join("generated_statics.rs");
    let mut file = BufWriter::new(File::create(file_path).unwrap());

    writeln!(
        &mut file,
        "//! Auto-generated static data containing n-gram hash maps.

/// Pseudo-struct used for aligning bytes to a certain type.
#[repr(C)]
pub struct AlignedTo<Align, Bytes: ?Sized> {{
    _align: [Align; 0],
    bytes: Bytes,
}}
"
    )
    .unwrap();

    let mut ngram_names = [None; 5];
    for n in 1..=5 {
        let ngram_name = Ngram::find_ngram_name_by_length(n);
        let file_path = models_dir.as_ref().join(format!("{}s.bin", ngram_name));

        if file_path.exists() {
            ngram_names[n - 1] = Some(ngram_name);

            writeln!(
                &mut file,
                r#"static {}S: &AlignedTo<u128, [u8]> = &AlignedTo {{
    _align: [],
    bytes: *include_bytes!("{}"),
}};"#,
                ngram_name.to_uppercase(),
                fs::canonicalize(file_path).unwrap().display(),
            )
            .unwrap();
        }
    }

    writeln!(&mut file, "\npub static NGRAMS: [Option<&[u8]>; 5] = [").unwrap();
    for ngram_name in ngram_names {
        let line = ngram_name
            .map(|name| format!("Some(&{}S.bytes),", name.to_uppercase()))
            .unwrap_or_else(|| String::from("None,"));
        writeln!(&mut file, "    {}", line).unwrap()
    }
    writeln!(&mut file, "];").unwrap();
}
