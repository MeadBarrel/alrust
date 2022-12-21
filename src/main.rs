mod app;
mod error;
mod wishes;
mod toppanel;
mod editor;
mod widget;
mod id;

pub fn main() {
    app::main()
}

// let grimoire = GrimoireSqlite::connect("../backups/db.sqlite").unwrap().load().unwrap();
// let grimoire_ser: GrimoireVersioned = grimoire.into();
// let f = File::create("grimoire.json").unwrap();
// to_writer_pretty(f, &grimoire_ser).unwrap();
