use genetic::NotNan;
use geneticalchemy::prelude::Mix;
use grimoire2::prelude::Effect;
use grimoire2::theoretical::Theoretical;
use reedline_repl_rs::*;
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use super::printer::*;
use super::error::{OptimizationError, Result};
use std::ops::Neg;
use std::sync::{Arc, Mutex};
use super::eexpr::EvalExpressionFitnessElement;
use geneticalchemy::prelude::AlchemyFitnessElement;



struct Context {
    populations: Arc<Mutex<PopulationsSerializable>>,
    selected_generation: Option<usize>,
    last_table: Option<PopulationSerializable>,
    sort: Option<EvalExpressionFitnessElement>,
    truncate: Option<usize>,
}

impl Context {
    pub fn population(&self) -> Result<PopulationSerializable> {
        match self.selected_generation {
            Some(x) => match self.populations.lock().unwrap().populations.get(x) {
                    Some(x) => Ok(x.clone()),
                    None => Err(OptimizationError::GenericError("The index is greater than the number of populations".to_string()))
            }
            None => match self.populations.lock().unwrap().populations.last() {
                    Some(x) => Ok(x.clone()),
                    None => Err(OptimizationError::GenericError("No populations yet".to_string()))
                }
            }
    }
}

pub fn run_repl(populations: Arc<Mutex<PopulationsSerializable>>) {

    let context = Context {
        populations,
        selected_generation: None,
        last_table: None,
        sort: None,
        truncate: None,
    };

    Repl::new(context)
        .with_name("Alrust genetic optimization")
        .with_stop_on_ctrl_d(true)
        .with_command(
            Command::new("generations"), 
            generations
        )
        .with_command(
            Command::new("save")
                .arg(
                    Arg::new("filename")
                        .index(1)
                        .required(true)
                ), 
            save
        )
        .with_command(
            Command::new("select")
                .arg(
                    Arg::new("index")
                        .index(1)
                        .required(true)
                ), 
            select
        )
        .with_command(
            Command::new("last"), 
            select_last
        )
        .with_command(Command::new("table"), table)
        .with_command(Command::new("sort").arg(Arg::new("value").index(1)), set_sort)
        .with_command(Command::new("truncate").arg(Arg::new("value").index(1)), truncate)
        .with_command(Command::new("show").arg(Arg::new("index").index(1)), show)
        .run().unwrap();
}

fn save(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let filename = args.get_one::<String>("filename").unwrap();
    let populations = context_.populations.as_ref();
    let result = crate::fs::save(std::path::Path::new(filename), populations);
    if let Err(x) = result {
        return Ok(Some(x.to_string()))
    }

    Ok(Some(format!("Saved as {filename}")))
}

fn select(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let index: usize = args
        .get_one::<String>("index")
        .ok_or(OptimizationError::GenericError("Argument not provided".to_string()))?
        .parse::<usize>()
        .map_err(
            |_| OptimizationError::GenericError("Could not parse argument as integer".to_string())
        )?;
    
    context_.selected_generation = Some(index);
    Ok(Some("Selected".to_string()))
}

fn select_last(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    context_.selected_generation = None;
    Ok(Some("Tracking last generation".to_string()))
}

fn generations(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let result = context_.populations.lock().unwrap().populations.len();
    Ok(Some(result.to_string()))
}

fn table(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    use cli_table::{format::Justify, Cell, Style, Table, CellStruct};

    let grimoire = context_.populations.lock().unwrap().grimoire.clone();

    let mut population = context_.population()?;

    if let Some(sort) = &context_.sort {
        population.individuals.sort_by_key(|individual| {
            let mix = Mix::new(&grimoire, individual.genome.clone());
            NotNan::new(sort.fitness(&mix)).unwrap_or(NotNan::default()).neg()
        });    
    }

    if let Some(truncate) = &context_.truncate {
        population.individuals.truncate(*truncate);
    }

    context_.last_table = Some(population.clone());

    println!("{}", population.individuals.len());

    let cells: Vec<Vec<CellStruct>> = population.individuals.into_iter().enumerate().map(|(i, individual)| {
        let mix = Mix::new(&grimoire, individual.genome);
        vec![
            i.cell(),
            mix.volume().cell(),
            theoretical_to_str(mix.effect(Effect::DirectHealing)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::DirectPoison)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::HealingOverTime)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::PoisonOverTime)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::HealingLength)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::PoisonLength)).cell().justify(Justify::Right),
            theoretical_to_str(mix.effect(Effect::Alcohol)).cell().justify(Justify::Right),
        ]
    }).collect();
    let table = cells.table().title(vec![
        "volume".cell().bold(true), 
        "idx".cell().bold(true), 
        "dh".cell().bold(true),
        "dp".cell().bold(true),
        "hot".cell().bold(true),
        "pot".cell().bold(true),
        "hl".cell().bold(true),
        "pl".cell().bold(true),
        "a".cell().bold(true),
    ])
    .bold(true);

    Ok(Some(format!("{}", table.display().unwrap())))
}

fn theoretical_to_str(value: Theoretical<f64>) -> String {
    match value {
        Theoretical::Known(x) => format!("{x:.3}"),
        Theoretical::Theory(x) => format!("!?{x:.3}"),
        Theoretical::Unknown => "??".to_string(),
    }
}

fn set_sort(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let value = args.get_one::<String>("value").cloned();
    context_.sort = match value {
        Some(x) => Some(EvalExpressionFitnessElement::new(evalexpr::build_operator_tree(&x)?, 1.)),
        None => None
    };
    Ok(Some("Sorting parameter set".to_string()))
}

fn truncate(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    let value = args.get_one::<String>("value").cloned();
    context_.truncate = match value {
        Some(x) => match x.parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => return Err(OptimizationError::GenericError("Could not convert to integer".to_string()))
        }
        None => None
    };

    Ok(Some("Truncate length set".to_string()))
}

fn show(args: ArgMatches, context_: &mut Context) -> Result<Option<String>> {
    use grimoire_serde::potion::PotionSerializableConfig;
    use serde_yaml::to_string;

    let value = args.get_one::<String>("index").cloned().unwrap();
    let index = match value.parse::<usize>() {
        Ok(x) => x,
        Err(_) => return Err(OptimizationError::GenericError("Could not convert to integer".to_string()))
    };
    let population = context_.last_table.clone()
        .ok_or(
            OptimizationError::GenericError(
                "You did not open any tables yet (use table command)".to_string())
        )?;
    let individual = population.individuals.get(index).ok_or(
        OptimizationError::GenericError(
            "The index exceeds population size".to_string()
        )
    )?;

    let grimoire = context_.populations.lock().unwrap().grimoire.clone();

    let mix = Mix::new(&grimoire, individual.genome.clone());

    let config = PotionSerializableConfig::default();
    let potion = config.serialize_mix(&mix);
    let result = to_string(&potion)
        .map_err(|_| OptimizationError::GenericError("Serialization failed".to_string()))?;
    Ok(Some(result))
}
