use std::io::stdout;
use evalexpr::*;
use grimoire2::effect::Effect;
use grimoire2::grimoire::{Ingredient, Grimoire};
use grimoire_serde::grimoire::ingredient::IngredientHumanReadable;
use indexmap::IndexMap;
use serde_yaml::to_writer;
use clap::*;

use crate::explore::view::ingredient;

pub fn command() -> Command {
    Command::new("ingredients")
        .arg(
            Arg::new("filter")
            .index(1)
            .trailing_var_arg(true)
            .required(false)
            .value_name("filter")
        )
        .arg(
            Arg::new("detailed")
            .help("Print detailed info")
            .short('d')
            .long("detailed")
            .action(ArgAction::SetTrue)
        )
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    list_ingredients(
        grimoire, 
        args.get_one::<String>("filter").cloned(), 
        args.get_flag("detailed")
    )
}

pub fn list_ingredients(grimoire: Grimoire, filter: Option<String>, detailed: bool) {
    let filtered = match filter {
        Some(filter) => {
            grimoire.ingredients.into_iter().filter(|(_, ingredient)| {
                filter_ingredient(ingredient, &filter)
            }).collect()
        }
        None => grimoire.ingredients,
    };

    if detailed {
        let filtered: IndexMap<String, IngredientHumanReadable> = filtered.into_iter()
            .map(|(n, v)| (n, v.into()))
            .collect();
        return to_writer(stdout(), &filtered).unwrap();
    } 
    for name in filtered.keys() {
        println!("{name}");
    }
}

fn filter_ingredient(ingredient: &Ingredient, filter: &str) -> bool {
    let mut context = HashMapContext::new();

    context.set_value("weight".to_string(), ingredient.weight.into()).unwrap();

    {
        let effect = Effect::DirectHealing;
        let term = "dh";

        context.set_value("dh".to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };

    {
        let effect = Effect::DirectPoison;
        let term = "dp";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    {
        let effect = Effect::HealingOverTime;
        let term = "hot";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    {
        let effect = Effect::PoisonOverTime;
        let term = "pot";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    {
        let effect = Effect::HealingLength;
        let term = "hl";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    {
        let effect = Effect::PoisonLength;
        let term = "pl";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    {
        let effect = Effect::Alcohol;
        let term = "a";

        context.set_value(term.to_string(), ingredient.modifiers[effect].term.inner().into()).unwrap();
        context.set_value(format!("{term}_known"), ingredient.modifiers[effect].term.is_known().into()).unwrap();
        context.set_value(format!("{term}_theory"), ingredient.modifiers[effect].term.is_theory().into()).unwrap();
        context.set_value(format!("{term}_unknown"), ingredient.modifiers[effect].term.is_unknown().into()).unwrap();

        context.set_value(format!("m{term}"), ingredient.modifiers[effect].multiplier.inner().into()).unwrap();
        context.set_value(format!("m{term}_known"), ingredient.modifiers[effect].multiplier.is_known().into()).unwrap();
        context.set_value(format!("m{term}_theory"), ingredient.modifiers[effect].multiplier.is_theory().into()).unwrap();
        context.set_value(format!("m{term}_unknown"), ingredient.modifiers[effect].multiplier.is_unknown().into()).unwrap();
    };    

    eval_boolean_with_context(filter, &context).unwrap()
}