use include_dir::{include_dir, Dir};

pub const BURMESE_MODELS_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/models");

pub const BURMESE_TESTDATA_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/testdata");