use clap::*;
use genetic::op;
use serde_yaml::to_writer;
use std::io::stdout;

mod experiment;
mod guess;
mod grimoiredb;
mod models;
mod schema;
mod optimization;

use geneticalchemy;
use grimoire::serializable::PotionSerializable;

use crate::grimoiredb::run_migrations;


fn run_genetic(config: &str) {
    // geneticalchemy::builder::GAConfig::load(config).run().unwrap();
    optimization::build::Optimizator::load(config).unwrap().run().unwrap();
}


fn run_experiment(config: &str) {
    let mix = experiment::ExperimentConfig::load(config).unwrap().mix().unwrap();
    let potion = PotionSerializable::from_mix(&mix);
    to_writer(stdout(), &potion).unwrap();
}


fn run_db(filename: &str) {
    use grimoiredb::load_grimoire_from_db;

    load_grimoire_from_db(filename).unwrap();
}


fn run_update(from: &str, to: &str) {
    use diesel::prelude::{SqliteConnection, Connection};
    use models::write_compendium;
    use grimoiredb::GrimoireConfig;

    let grimoire = GrimoireConfig::load(from).unwrap().build().unwrap();

    let mut connection = SqliteConnection::establish(to).unwrap();
    run_migrations(&mut connection).unwrap();

    write_compendium(&mut connection, &grimoire).unwrap();
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
        .subcommand(
            Command::new("db")
                .arg(
                    arg!(--filename <VALUE>).required(true)
                )
        )
        .subcommand(
            Command::new("update")
                .arg(arg!(--from <VALUE>).required(true))
                .arg(arg!(--to <VALUE>).required(true))
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
