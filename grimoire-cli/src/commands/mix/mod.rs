mod experiment;

use clap::{Command, Arg};
use grimoire_serde::history::History;

use crate::commands::{SubCommand, CommandError};
use error_stack::{Report, ResultExt};
use crate::fs::load;
use experiment::ExperimentConfig;
use std::path::Path;


pub struct MixCommand();


impl SubCommand for MixCommand {
    fn accepts(&self, name: &str) -> bool {
        name == "mix"
    }

    fn create(&self) -> Command {
        Command::new("mix")
            .arg(
                Arg::new("config")
                .short('c')
                .long("config")
                .value_name("config")                
            )
    }

    fn run(&self, history: grimoire_serde::history::History, matches: &clap::ArgMatches) -> error_stack::Result<(), CommandError> {
        let config = matches.get_one::<String>("config");
        if config.is_none() {
            return Err(Report::new(CommandError())).attach_printable("Config file not set");
        }

        let experiment_config: ExperimentConfig = load(Path::new(config.unwrap())).unwrap();

        let grimoire = history.to_grimoire();

        experiment_config.run(grimoire).unwrap();

        Ok(())
    }
}