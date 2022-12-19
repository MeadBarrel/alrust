mod fs;
mod commands;

use std::path::Path;

use clap::{Command, Arg, ArgAction};

use grimoire_serde::history::History;


fn main() {

    let commands: Vec<Box<dyn commands::SubCommand>> = vec![
        Box::new(commands::mix::MixCommand())
    ];

    let grimoire_arg = Arg::new("grimoire")
        .short('g')
        .long("grimoire")
        .value_name("grimoire");

    let mut app = Command::new("Alrust")
        .arg_required_else_help(true);

    app = app.arg(grimoire_arg);

    for command in commands.iter() {
        app = app.subcommand(
            command.create()
        );
    }

    let matches = app.get_matches();

    let grimoire_name = matches.get_one::<String>("grimoire");

    let history: History = match grimoire_name {
        Some(x) => { fs::load(Path::new(x)).unwrap() },
        None => { println!("File not set"); return }
    };

    if let Some((name, sub_matches)) = matches.subcommand() {
        let command = commands.iter().find(|x| x.accepts(name));
        if let Some(com) = command {
            com.run(history, sub_matches).unwrap()
        }
    }



    // let json_arg = Arg::new("json")
    //     .short('j')
    //     .long("json")
    //     .help("JSON file to load")
    //     .conflicts_with("db");

    // let db_arg = Arg::new("db")
    //     .short('d')
    //     .long("db")
    //     .help("Database file to load")
    //     .conflicts_with("json");


    // let app = Command::new("Alrust")
    //     .version("0.1")
    //     .arg_required_else_help(true)
    //     .arg(db_arg)
    //     .arg(json_arg);

    // app.get_matches();
}
