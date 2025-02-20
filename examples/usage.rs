/*
This example demonstrates language detection using lingua-rs.
To run this example, use the following command:

cargo run --example usage
*/

fn main() {
    let detector = lingua::LanguageDetectorBuilder::from_all_languages().build();
    let language = detector.detect_language_of("你好，世界!"); // Chinese
    match language {
        Some(language) => println!("Detected language: {}", language),
        None => println!("Could not detect the language."),
    }
}
