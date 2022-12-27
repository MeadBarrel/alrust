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

unknown_multiplier: float  # Theoretical values will be multiplied by this factor during evaluation

num_children: int  # Number of children

";

pub fn command() -> Command {
    Command::new("optimize")
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
        // .arg(
        //     Arg::new("output")
        //         .short('o')
        //         .long("output")
        //         .env("ALRUST_OPTIMIZE_OUTPUT")
        //         .required(true)
        // )
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    let config_filename = std::path::Path::new(args.get_one::<String>("config").unwrap());
    let config: config::OptimizatorConfig = crate::fs::load(config_filename).unwrap();

    let character_name = args.get_one::<String>("character").unwrap();
    let character = grimoire.characters.get(character_name.as_str()).expect("Character not found").clone();

    let optimizator = build::Optimizator::new(grimoire, character, config);
    // let populations = optimizator.populations.clone();

    // //let output_filename = args.get_one::<String>("output").unwrap();
    // let (sender, receiver) = mpsc::channel();

    // thread::spawn(move || optimizator.run(receiver).unwrap());

    repl::run_repl(optimizator);
    
}