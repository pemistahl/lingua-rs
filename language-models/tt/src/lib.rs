use include_dir::{include_dir, Dir};

pub const TATAR_MODELS_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/models");

pub const TATAR_TESTDATA_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/testdata");