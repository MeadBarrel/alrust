mod eexpr;
mod config;
mod print;
mod error;
mod build;

use clap::*;
use grimoire2::grimoire::Grimoire;

const CONFIG_HELP: &str = "
Config file format:

grimoire: (see help for `update` command)

character: String  # name of the character 

population_size: int  # size of population

output_every: int  # how often to print the population, i.e. if it's 100, every 100th population will be printed

volume: float  # desired volume; NOTE: it doesn't take alvarin clade into account

effects:  # what will the algorithm optimize for
    - <expression using dh, mdh, dp, mdp, hot, mhot, pot, mpot, hl, mhl, pl, mpl, a, ma>
    - ...

include_ingredients: expression  # Not required, expression that returns bool to determine whether ingredient will be included

unknown_multiplier: float  # Theoretical values will be multiplied by this factor during evaluation

num_children: int  # Number of children

print:
    folder: <folder to write to - default is `output`>
    subfolder: <subfolder to write to - default is based on current date and time>
    config: <potion serialization config (see help for `mix`)>

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
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    let config_filename = std::path::Path::new(args.get_one::<String>("config").unwrap());
    let config: config::OptimizatorConfig = crate::fs::load(config_filename).unwrap();
    let mut optimizator = build::Optimizator::new(grimoire, config).unwrap();
    optimizator.run().unwrap();
}