#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate core;

use grimoire2::prelude::Grimoire;
use grimoire2::grimoire::versioned::GrimoireVersioned;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_grimoire_dialog])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn open_grimoire_dialog() -> Result<Option<Grimoire>, String> {
  let dialog = rfd::FileDialog::new().add_filter("Grimoire as JSON", &["json"]);
  let path = if let Some(path) = dialog.pick_file() { path } else { return Ok(None); };
  Ok(Some(load_grimoire(path)?))
}

fn load_grimoire(path: std::path::PathBuf) -> Result<Grimoire, String> {
    let f = match std::fs::File::open(path) {
        Ok(x) => x,
        Err(_) => Err("oops".to_string())?
    };
    let grimoire_ver: GrimoireVersioned = match serde_json::from_reader(f) {
        Ok(x) => x,
        Err(_) => Err("Deserialization failed")?,
    };
    Ok(grimoire_ver.into())
}