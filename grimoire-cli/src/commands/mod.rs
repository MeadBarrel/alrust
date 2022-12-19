pub mod mix;


use clap::{Command, ArgMatches};
use error_stack::Result;

use grimoire_serde::history::History;


#[derive(Debug, thiserror::Error)]
#[error("Error executing command")]
pub struct CommandError();


pub trait SubCommand {
    fn create(&self) -> Command;
    fn accepts(&self, name: &str) -> bool;
    fn run(&self, history: History, matches: &ArgMatches) -> Result<(), CommandError>;
}
