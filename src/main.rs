mod app;
mod error;
mod toppanel;
mod editor;
mod widget;
mod id;


use tracing::{self, instrument::WithSubscriber};
use tracing_subscriber::*;




pub fn main() {
    let subs = fmt()
        .with_env_filter(EnvFilter::new("alrust=debug"))
        .finish();
    tracing::subscriber::set_global_default(subs);
        

    //simple_logger::SimpleLogger::default().init().unwrap();


    app::main()
}

// let grimoire = GrimoireSqlite::connect("../backups/db.sqlite").unwrap().load().unwrap();
// let grimoire_ser: GrimoireVersioned = grimoire.into();
// let f = File::create("grimoire.json").unwrap();
// to_writer_pretty(f, &grimoire_ser).unwrap();
