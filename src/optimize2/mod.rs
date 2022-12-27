mod config;
mod printer;
mod build;
mod error;
mod eexpr;
mod repl;
mod message;

use std::{sync::mpsc, thread};

use grimoire2::grimoire::Grimoire;
use clap::*;

const CONFIG_HELP: &str = "
Config file format:

grimoire: (see help for `update` command)

population_size: int  # size of population

output_every: int  # how often to print the population, i.e. if it's 100, every 100th population will be printed

volume: float  # desired volume; NOTE: it doesn't take alvarin clade into account

effects:  # what will the algorithm optimize for
    - <expression using dh, mdh, dp, mdp, hot, mhot, pot, mpot, hl, mhl, pl, mpl, a, ma>
    - ...

include_ingredients: expression  # Not required, expression that returns bool to determine whether ingredient will be included

exclude_ingredients:

    - <ingredient name>
    - <ingredient name>

unknown_multiplier: float  # Theoretical values will be multiplied by this factor during evaluation

num_children: int  # Number of children

";

pub fn command_run() -> Command {
    Command::new("optimize")
        .before_help("Run genetic optimization algorithm to find best potions")
        .arg(
            Arg::new("config")
                .index(1)
                .required(true)
                .help("Configuration file")
                .long_help(CONFIG_HELP)
        )
        .arg(
            Arg::new("character")
                .short('c')
                .long("character")
                .env("ALRUST_CHARACTER")
                .required(true)
        )
}

pub fn matched_command_run(grimoire: Grimoire, args: &ArgMatches) {
    let config_filename = std::path::Path::new(args.get_one::<String>("config").unwrap());
    let config: config::OptimizatorConfig = crate::fs::load(config_filename).unwrap();

    let character_name = args.get_one::<String>("character").unwrap();
    let character = grimoire.characters.get(character_name.as_str()).expect("Character not found").clone();

    let mut optimizator = build::Optimizator::new(grimoire, character, config);
    let populations = optimizator.populations.clone();

    //let output_filename = args.get_one::<String>("output").unwrap();
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || optimizator.run(receiver).unwrap());

    repl::run_repl(populations);

    sender.send(message::Message::Stop).unwrap();

    handle.join().unwrap();
}

pub fn command_explore() -> Command {
    Command::new("explore")
        .before_help("Explore the previously saved results of a genetic algorithm optimization")
        .arg(
            Arg::new("filename")
                .index(1)
                .required(true)
        )
}


pub fn matched_command_explore(args: &ArgMatches) {
    use std::path::Path;
    use crate::fs::load;
    use printer::PopulationsSerializable;
    use std::sync::{Arc, Mutex};

    let filename = args.get_one::<String>("filename").unwrap();
    let populations: PopulationsSerializable = load(Path::new(filename)).unwrap();
    let arched_populations = Arc::new(Mutex::new(populations));
    repl::run_repl(arched_populations);
}