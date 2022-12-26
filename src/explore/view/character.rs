use std::io::stdout;

use grimoire2::grimoire::Grimoire;
use grimoire_serde::grimoire::character::CharacterHumanReadable;
use serde_yaml::to_writer;
use clap::*;
use tracing::info;

pub fn command() -> Command {
    Command::new("character")
        .arg(
            Arg::new("name")
            .index(1)
            .required(true)
            .value_name("name")
        )
}

pub fn matched_command(mut grimoire: Grimoire, args: &ArgMatches) {
    match grimoire.characters.remove(args.get_one::<String>("name").unwrap()) {
        Some(x) => {
            let hr: CharacterHumanReadable = x.into();
            to_writer(stdout(), &hr).unwrap();
        },
        None => {
            info!("Character not found");
        }
    }
}