mod fs;
mod update;
mod explore;
mod mix;
mod optimize;

use std::path::Path;
use tracing_subscriber::*;
use clap::*;
use grimoire2::grimoire::versioned::GrimoireVersioned;
use grimoire2::grimoire::Grimoire;


pub fn main() {
    let subs = fmt()
        .with_env_filter(EnvFilter::new("alrust=debug"))
        .finish();
    tracing::subscriber::set_global_default(subs).unwrap();

    let update_subcommand = Command::new("update")
        .arg(
            Arg::new("from")
                .index(1)
                .value_name("from")
                .required(true)
                .help("Grimoire update file")
                .long_help(
                    "Grimoire update file\n\
                     ====================\n\
                    Format of the file:\n\n\
                    remove_characters:\n\t<name>\n\t...\n\n\
                    remove_skills:\n\t<name>\n\t...\n\n\
                    remove_ingredients:\n\t<name>\n\t...\n\n\
                    characters:\n\
                    \t<character name>:\n\
                    \t\tremove_clades:\n\t\t\t- <clade>\n\t\t\t...\n\n\
                    \t\tremove_skills:\n\t\t\t- <skill>\n\t\t\t...\n\n\
                    \t\tadd_clades:\n\t\t\t- <clade>\n\t\t\t...\n\n\
                    \t\tskills:\n\t\t\t<skill>:<value>\n\t\t\t...\n\n\
                    skills:\n\
                    \t<skill name>:\n\
                    \t\teffectiveness: (effectiveness, theoretical*)\n\
                    \t\tparent: <name of parent 1>\n\
                    \t\tparent_2: <name of parent 2>\n\
                    \t\tremove_parent: bool\n\
                    \t\tremove_parent_2: bool\n\n\
                    ingredients:\n\
                    \t\t<name of ingredient>:\n\
                    \t\t\tskill: <lore of ingredient>\n\
                    \t\t\tremove_skill: bool  # remove lore\n\
                    \t\t\tweight: bool  # whether the ingredient has alchemical weight\n\
                    \t\t\tdh: (direct healing, theoretical*)\n\
                    \t\t\tmdh: (direct healing multiplier, theoretical*)\n\
                    \t\t\t<... dp, mdp, hot, mhot, pot, mpot, hl, mhl, pl, mpl, a, ma>\
                    \n\n\
                    * theoretical format is either:\n\
                    \t<floating point value (5, 0.4, 3.1 etc)> - Known value\n\
                    \t!?<floating point value> - Theoretical value\n\
                    \t?? - Value is unknown" 
                )
        )
        .arg(
            Arg::new("to")
                .index(2)
                .value_name("to")
                .required(true)
        )
        .arg_required_else_help(true);

    let grimoire_arg = Arg::new("grimoire")
        .index(1)
        .value_name("grimoire")
        .required(true);

    let app = Command::new("Alrust")
        .arg(grimoire_arg)
        .subcommand(update_subcommand)
        .subcommand(explore::list::command())
        .subcommand(explore::view::command())
        .subcommand(mix::command())
        .subcommand(optimize::command())
        .subcommand_required(true)
        .arg_required_else_help(true);


    let matches = app.get_matches();
    let grimoire_path = Path::new(matches.get_one::<String>("grimoire").unwrap());
    let grimoire_versioned: GrimoireVersioned = fs::load(grimoire_path).unwrap();
    let grimoire: Grimoire = grimoire_versioned.into();

    match matches.subcommand() {
        Some(("update", args)) => {
            update::update_grimoire(
                grimoire, 
                Path::new(args.get_one::<String>("from").unwrap()), 
                Path::new(args.get_one::<String>("to").unwrap()),
            ).unwrap();
        },
        Some(("list", args)) => {
            explore::list::matched_command(grimoire, args)
        },
        Some(("view", args)) => {
            explore::view::matched_command(grimoire, args)
        },
        Some(("mix", args)) => {
            mix::matched_command(grimoire, args)
        },
        Some(("optimize", args)) => {
            optimize::matched_command(grimoire, args)
        }
        None | Some(_) => {}
    }
        
}
