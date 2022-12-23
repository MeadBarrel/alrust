mod app;
mod error;
mod toppanel;
mod editor;
mod id;
mod grimoire;


use tracing_subscriber::*;




pub fn main() {
    let subs = fmt()
        .with_env_filter(EnvFilter::new("alrust=debug"))
        .finish();
    tracing::subscriber::set_global_default(subs).unwrap();

    app::main()
}
