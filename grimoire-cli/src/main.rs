mod fs;
mod experiment;
mod grimoire;

use clap::{Command, Arg};


fn main() {
    use grimoire_sqlite::GrimoireSqlite;
    use grimoire_serde::history::History;
    use grimoire2::modify::skill::SkillUpdate;
    use grimoire2::theoretical::Theoretical;
    use crate::fs::save;
    use std::path::Path;

    let mut grimoire = GrimoireSqlite::connect("db.sqlite").unwrap().load().unwrap();

    let skill_update = SkillUpdate::default().set_effectiveness(Theoretical::Unknown).clone();

    grimoire.skills.iter_mut().for_each(|x| skill_update.update(x.1));

    let history = History::from_grimoire(&grimoire, 0);
    save(Path::new("history2.json"), &history).unwrap();

    // println!("Hello, world!");

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
