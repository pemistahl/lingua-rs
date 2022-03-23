fn main() {
    model_builder::build_frequencies("models");
    model_builder::generate_statics("../models", "src");
}
