use clap::*;
use evalexpr::context_map;
use grimoire2::grimoire::{Grimoire, Character};
use evalexpr::*;

pub fn command() -> Command {
    Command::new("characters")
        .arg(
            Arg::new("filter")
            .index(1)
            .trailing_var_arg(true)
            .required(false)
            .value_name("filter")
        )
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    list_characters(grimoire, args.get_one::<String>("filter").cloned())
}

pub fn list_characters(grimorie: Grimoire, filter: Option<String>) {
    for (name, _) in grimorie.characters.into_iter().filter(|(_, c)| {
        match &filter {
            Some(x) => filter_character(c.clone(), x),
            None => true,
        } 
    }) {
        println!("{}", name);
    }
}

pub fn filter_character(character: Character, filter: &str) -> bool {

    let clades = character.clades.clone();

    let context = context_map! {
        "raw_skill" => Function::new(move |skill| {
            let skill_str = skill.as_string()?;
            let result: i64 = character.raw_skill(&skill_str).into();
            Ok(result.into())
        }),
        "clade" => Function::new(move |clade| {
            let clade_str = clade.as_string()?;
            let result = clades.contains(&clade_str);
            Ok(result.into())
        }),
    }.unwrap();

    eval_boolean_with_context(filter, &context).unwrap()
}