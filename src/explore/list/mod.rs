mod characters;
mod skills;
mod ingredients;

use clap::*;
use grimoire2::grimoire::Grimoire;

pub fn command() -> Command {
    Command::new("list")
        .subcommand(characters::command())
        .subcommand(skills::command())
        .subcommand(ingredients::command())
        .subcommand_required(true)
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    match args.subcommand() {
        Some(("characters", args)) => {
            characters::matched_command(grimoire, args)
        },
        Some(("skills", args)) => {
            skills::matched_command(grimoire, args)
        },
        Some(("ingredients", args)) => {
            ingredients::matched_command(grimoire, args)
        }
        None | Some(_) => {
        }
    }
}