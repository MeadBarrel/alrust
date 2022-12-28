use std::path::Path;
use error_stack::{Report, Result, IntoReport, ResultExt};
use grimoire2::modify::command::Commands;
use grimoire2::prelude::{Grimoire, Character};
use grimoire2::standalone::OptimizedGrimoire;
use grimoire2::standalone::Mix;
use serde::Deserialize;
use grimoire_serde::potion::{PotionSerializableConfig, PotionSerializable};
use grimoire_serde::modify::GrimoireUpdateSerializable;
use grimoire_serde::mix::MixIngredients;
use crate::fs::load;
use clap::*;
use thiserror::Error;

#[derive(Deserialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MixConfig {
    potion: PotionSerializableConfig,
    grimoire: GrimoireUpdateSerializable,
    mix: MixIngredients
}

#[derive(Error, Debug)]
pub enum MixError {
    #[error("Ingredient not found: {0}")]
    IngredientNotFound(String),
    #[error("Character not found: {0}")]
    CharacterNotFound(String),
}

pub fn command() -> Command {
    Command::new("mix")
        .before_help("Calculate a potion")
        .arg(
            Arg::new("character")
                .short('c')
                .long("character")
                .required(true)
                .help("Character name")
                .env("ALRUST_CHARACTER")            
        )
        .arg(
            Arg::new("mixfile")
                .index(2)
                .help("Mix configuration file")
                .env("ALRUST_MIX")
                .required(true)
                .long_help(
                    "Path to mix configuration file\n\
                    \n\
                    Configuration file format:\n\
                    \n\
                    potion: (optional)  # configure potion output\n\
                    \tvolume: bool (default true)  # show volume\n\
                    \teffects: bool (default true)  # show effects\n\
                    \tingredients: bool (default true)  # show ingredients\n\
                    \ttotal_healing_raw: bool (default false)  # healing stats\n\
                    \ttotal_poison_raw: bool (default false)  # poison stats\n\
                    \ttotal_healing: bool (default false)  # healing-poison stats\n\
                    \ttotal_poison: bool (default false)  # poison-healing stats\n\n\
                    grimoire: grimoire update configuration (see help for `update` command)\n\n\
                    mix:\n\
                    \t<name of ingredient>: <amount>\n\
                    \t<name of ingredient>: <amount>\n\
                    \t..."
                )
        )
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    let character_name = args.get_one::<String>("character").unwrap();
    let config: MixConfig = load(Path::new(args.get_one::<String>("mixfile").unwrap())).unwrap();

    let character = grimoire.characters.get(character_name.as_str()).ok_or(
        Report::new(MixError::CharacterNotFound(character_name.clone()))
    ).unwrap().clone();

    let potion = config.run(grimoire, character).unwrap();
    serde_yaml::to_writer(std::io::stdout(), &potion).unwrap();
}

impl MixConfig {
    pub fn run(&self, mut grimoire: Grimoire, character: Character) -> Result<PotionSerializable, MixError> {
        self.grimoire.to_update().update(&mut grimoire);
      
        let optimized = OptimizedGrimoire::from((&character, &grimoire));

        let mut ingredients: Vec<(usize, u64)> = Vec::default();

        for (name, value) in &self.mix {
            let index = optimized
                .ingredients
                .by_name(name)
                .into_report()
                .change_context(MixError::IngredientNotFound(name.to_string()))?;

            ingredients.push((index, *value))
        };

        let mix = Mix::new(&optimized, ingredients);

        Ok(self.potion.serialize_mix(&mix))
    }
}