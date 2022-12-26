use crate::fs::*;
use grimoire2::grimoire::versioned::GrimoireVersioned;
use grimoire2::grimoire::Grimoire;
use grimoire2::modify::command::Commands;
use grimoire_serde::modify::GrimoireUpdateSerializable;
use std::path::Path;
use error_stack::Result;

pub fn update_grimoire(mut grimoire: Grimoire, from: &Path, to: &Path) -> Result<(), FSOperationError> {
    let from: GrimoireUpdateSerializable = load(from)?;

    from.to_update().update(&mut grimoire);

    let output_versioned: GrimoireVersioned = grimoire.into();

    save(to, &output_versioned)
}