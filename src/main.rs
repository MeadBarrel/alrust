mod prediction;

use geneticalchemy;


fn   main() {
    geneticalchemy::builder::GAConfig::load("config.yaml").run().unwrap();
}
