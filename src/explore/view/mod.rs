pub mod character;
pub mod skill;
pub mod ingredient;

use clap::{Command, ArgMatches};
use grimoire2::grimoire::Grimoire;

pub fn command() -> Command {
    Command::new("view")
        .subcommand(character::command())
        .subcommand(skill::command())
        .subcommand(ingredient::command())
        .subcommand_required(true)
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    match args.subcommand() {
        Some(("character", args)) => character::matched_command(grimoire, args),
        Some(("skill", args)) => skill::matched_command(grimoire, args),
        Some(("ingredient", args)) => ingredient::matched_command(grimoire, args),
        None | Some(_) => {}
    }
}