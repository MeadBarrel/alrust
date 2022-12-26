use clap::*;
use evalexpr::*;
use grimoire2::grimoire::{Grimoire, Skill};

pub fn command() -> Command {
    Command::new("skills")
        .arg(
            Arg::new("filter")
            .index(1)
            .trailing_var_arg(true)
            .required(false)
            .value_name("filter")
        )        
}

pub fn matched_command(grimoire: Grimoire, args: &ArgMatches) {
    list_skills(grimoire, args.get_one::<String>("filter").cloned())
}

pub fn list_skills(grimoire: Grimoire, filter: Option<String>) {
    let filter_ = |skill: &Skill| {
        match &filter {
            Some(x) => filter_skill(&grimoire, skill.clone(), x),
            None => true
        }
    };

    for (name, skill) in grimoire.skills.iter().filter(|(_, skill)| filter_(skill)) {
        println!("{}", name);
    }   
}

fn filter_skill(grimoire: &Grimoire, skill: Skill, filter: &str) -> bool {
    let grimoire_1 = grimoire.clone();
    let context = context_map! {
        "is_child" => Function::new(move |arg| {
            let arg_str = arg.as_string()?;
            let result = skill.is_child(&grimoire_1, &arg_str);
            Ok(result.into())
        }),
    }.unwrap();
    eval_boolean_with_context(filter, &context).unwrap()
}