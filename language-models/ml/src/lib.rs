use include_dir::{include_dir, Dir};

pub const MALAYALAM_MODELS_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/models");

pub const MALAYALAM_TESTDATA_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/testdata");