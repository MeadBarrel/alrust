mod app;
mod error;
mod toppanel;
mod editors;
mod id;
mod widgets;
mod global;

use tracing_subscriber::*;

pub fn main() {
    let subs = fmt()
        .with_env_filter(EnvFilter::new("alrust=debug"))
        .finish();
    tracing::subscriber::set_global_default(subs).unwrap();

    app::main()
}
