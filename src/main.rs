mod app;

use grimoire_sqlite::GrimoireSqlite;
use grimoire2::grimoire::versioned::GrimoireVersioned;
use std::fs::File;
use serde_json::to_writer_pretty;



fn main() {
    let grimoire = GrimoireSqlite::connect("../backups/db.sqlite").unwrap().load().unwrap();
    let grimoire_ser: GrimoireVersioned = grimoire.into();
    let f = File::create("grimoire.json").unwrap();
    to_writer_pretty(f, &grimoire_ser).unwrap();
    //app::main()
}