use clap::*;
use serde_yaml::to_writer;
use std::io::stdout;

mod experiment;
mod guess;

use geneticalchemy;
use grimoire::serializable::PotionSerializable;


fn run_genetic(config: &str) {
    geneticalchemy::builder::GAConfig::load(config).run().unwrap();
}


fn run_experiment(config: &str) {
    let mix = experiment::ExperimentConfig::from_file(config).unwrap().mix().unwrap();
    let potion = PotionSerializable::from_mix(&mix);
    to_writer(stdout(), &potion).unwrap();
}


fn main() {
    let matches = Command::new("MO2 Alchemy Tools")
        .subcommand(
            Command::new("genetic")
                .arg(
                    arg!(--config <VALUE>).default_value("config.yaml")
                )
        )
        .subcommand(
            Command::new("experiment")
                .arg(
                    arg!(--config <VALUE>).required(true)
                )
        )
        .subcommand(
            Command::new("guess")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("genetic", args)) => {
            let config_fn = args.get_one::<String>("config").unwrap();
            run_genetic(config_fn);
        },
        Some(("experiment", args)) => {
            let config_fn = args.get_one::<String>("config").unwrap();
            run_experiment(config_fn);
        }
        Some(("guess", args)) => { guess::greet() }
        Some((_, _)) => {}
        None => {}
    }
    //geneticalchemy::builder::GAConfig::load("config.yaml").run().unwrap();
}
