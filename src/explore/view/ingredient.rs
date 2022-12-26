use std::io::stdout;

use grimoire2::grimoire::Grimoire;
use grimoire_serde::grimoire::ingredient::IngredientHumanReadable;
use serde_yaml::to_writer;
use clap::*;
use tracing::info;

pub fn command() -> Command {
    Command::new("ingredient")
        .arg(
            Arg::new("name")
            .index(1)
            .required(true)
            .value_name("name")
        )
}

pub fn matched_command(mut grimoire: Grimoire, args: &ArgMatches) {
    match grimoire.ingredients.remove(args.get_one::<String>("name").unwrap()) {
        Some(x) => {
            let hr: IngredientHumanReadable = x.into();
            to_writer(stdout(), &hr).unwrap();
        },
        None => {
            info!("Ingredient not found");
        }
    }
}