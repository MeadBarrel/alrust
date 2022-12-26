use clap::{Command, ArgMatches};
use grimoire2::grimoire::Grimoire;

pub mod list;
pub mod view;

pub fn command() -> Command {
    
    list::command()
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    list::matched_command(grimoire, args)
}