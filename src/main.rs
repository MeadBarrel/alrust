mod fs;
mod update;
mod explore;
mod mix;

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
        }
        None | Some(_) => {}
    }
        
}
