use clap::*;
use serde_yaml::to_writer;
use std::io::stdout;

mod experiment;
mod grimoiredb;
mod guess;
mod optimization;
mod serializable;
mod theoretical;

use crate::serializable::PotionSerializable;


fn run_genetic(config: &str) {
    // geneticalchemy::builder::GAConfig::load(config).run().unwrap();
    optimization::build::Optimizator::load(config)
        .unwrap()
        .run()
        .unwrap();
}

fn run_experiment(config: &str) {
    let config = experiment::ExperimentConfig::load(config).unwrap();
    let grimoire = config.grimoire().unwrap();
    let mix = config.mix(&grimoire).unwrap();
    let potion = PotionSerializable::from_mix(&mix);
    to_writer(stdout(), &potion).unwrap();
}

fn run_db(filename: &str) {
    use grimoire_sqlite::GrimoireSqlite;
    GrimoireSqlite::connect(filename).unwrap().load().unwrap();
}

fn run_update(from: &str, to: &str) {
    use grimoiredb::GrimoireConfig;
    use grimoire_sqlite::GrimoireSqlite;

    let grimoire = GrimoireConfig::load(from).unwrap().build().unwrap();
    GrimoireSqlite::connect(to).unwrap().write(&grimoire).unwrap();
}

fn main() {
    let matches = Command::new("MO2 Alchemy Tools")
        .subcommand(
            Command::new("genetic").arg(arg!(--config <VALUE>).default_value("config.yaml")),
        )
        .subcommand(Command::new("experiment").arg(arg!(--config <VALUE>).required(true)))
        .subcommand(Command::new("guess"))
        .subcommand(Command::new("db").arg(arg!(--filename <VALUE>).required(true)))
        .subcommand(
            Command::new("update")
                .arg(arg!(--from <VALUE>).required(true))
                .arg(arg!(--to <VALUE>).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("genetic", args)) => {
            let config_fn = args.get_one::<String>("config").unwrap();
            run_genetic(config_fn);
        }
        Some(("experiment", args)) => {
            let config_fn = args.get_one::<String>("config").unwrap();
            run_experiment(config_fn);
        }
        Some(("guess", _)) => guess::greet(),
        Some(("db", args)) => {
            let filename = args.get_one::<String>("filename").unwrap();
            run_db(filename);
        }
        Some(("update", args)) => {
            let from = args.get_one::<String>("from").unwrap();
            let to = args.get_one::<String>("to").unwrap();
            run_update(from, to);
        }
        Some((_, _)) => {}
        None => {}
    }
    //geneticalchemy::builder::GAConfig::load("config.yaml").run().unwrap();
}
