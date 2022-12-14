use std::borrow::Borrow;
use std::fmt::format;
use std::fs::File;
use std::path::Path;
use std::borrow::BorrowMut;
use std::cmp::min;
use std::cell::RefCell;
use std::fs::create_dir_all;
use error_stack::ResultExt;
use evalexpr::Node;
use evalexpr::context_map;
use genetic::alias::{Individuals, RankedIndividuals};
use rand::prelude::*;
use anyhow::Result;
use serde_yaml::{from_reader, to_writer};

use genetic::prelude::*;
use serde::Deserialize;
use grimoire::prelude::*;
use grimoire::data::Ingredient;
use crate::eexpr::*;
use crate::algorithm::*;
use crate::fitness::*;
use crate::genetic::*;
use crate::mutate::*;
use crate::incubator::*;
use crate::genetic::*;



#[derive(Deserialize, Clone)]
pub enum DesiredEffects {
    MaximizeDH,
    MaximizeDP,
    MaximizeHOT,
    MaximizePOT,
    MaximizeHL,
    MaximizePL,
    MaximizeA,    
}


#[derive(Deserialize)]
#[serde(default)]
pub struct MutatorConfig {
    amount_grow_ratio: f64,
    min_amount_grow: u64,
    num_mutations_amt: usize,    
    num_mutations_ing: usize,
}

impl Default for MutatorConfig {
    fn default() -> Self {
        Self {
            amount_grow_ratio: 0.1,
            min_amount_grow: 1,
            num_mutations_amt: 4,
            num_mutations_ing: 2,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct GAConfig {
    pub db_name: String,
    pub character_name: String,
    pub mutate: MutatorConfig,
    pub tournament: TournamentSelector,
    pub population_size: u64,
    pub output_every: usize,
    pub output_folder: String,
    pub desired_volume: f64,
    pub desired_effects: Vec<Node>,
    pub include_ingredients: Option<Node>,
    pub num_children: usize,
}


impl Default for GAConfig {
    fn default() -> Self {
        Self {
            db_name: "db.sqlite".to_string(),
            character_name: "default".to_string(),
            mutate: MutatorConfig::default(),
            tournament: TournamentSelector::default(),
            population_size: 100,
            output_every: 1000,
            output_folder: "output".to_string(),
            desired_volume: 40.,
            desired_effects: vec![],
            include_ingredients: None,
            num_children: 2,
        }
    }
}


fn random_genome<R:Rng>(rng: &mut R, grimoire: &OptimizedGrimoir) -> AlchemyGenome {
    let genome_len = min(grimoire.ingredients.len(), 16);
    let grimoire_size = grimoire.ingredients.len();
    let selected_ingredients = (0..genome_len).choose_multiple(rng, grimoire_size);
    selected_ingredients.into_iter().map(
        |x| AlchemyGene { amount: rng.gen_range(0..10), ingredient_index: x} ).collect()
}


impl GAConfig {
    pub fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        from_reader(f).unwrap()
    }


    pub fn build(&self, grimoire: OptimizedGrimoir) -> Result<Box<dyn Algorithm<Population=ParettoPopulation<AlchemyIndividual>>>> {
        let mut rng = thread_rng();

        let mutate = AlchemyMutator::new(
            grimoire.ingredients.len(),
            self.mutate.amount_grow_ratio, 
            self.mutate.min_amount_grow,
            self.mutate.num_mutations_amt,
            self.mutate.num_mutations_ing,
        );

        let fitness_elements = self.desired_effects.iter().map(
            |x| Box::new(EvalExpressionFitnessElement::new(x.clone())) 
                    as Box<dyn AlchemyFitnessElement>
        ).collect();
            

        let fitness_function = AlchemyFitnessFunction::new(
            grimoire.clone(),
            fitness_elements,
            self.desired_volume,
        );

        let crossover = PrecedencePreservativeCrossover::new(self.num_children);
        let reinsert = ElitistReinserter::default();

        let initial_pool = (0..self.population_size).into_iter()
            .map(|_| random_genome(&mut rng, &grimoire)).collect();

        Ok(
            Box::new(
                create_alchemy_ga(
                    fitness_function, 
                    mutate, 
                    crossover, 
                    self.tournament.clone(), 
                    reinsert, 
                    rng, 
                    initial_pool
                )
            )
        )
    }

    pub fn should_include_ingredient(&self, ingredient: &Ingredient) -> Result<bool> {
        if self.include_ingredients.is_none() { return Ok(true) };
        
        let context = context_map! {
            "dh" => ingredient.get_modifier(Property::DirectHealing).modifier,
            "mdh" => ingredient.get_modifier(Property::DirectHealing).multiplier,

            "dp" => ingredient.get_modifier(Property::DirectPoison).modifier,
            "mdp" => ingredient.get_modifier(Property::DirectPoison).multiplier,

            "hot" => ingredient.get_modifier(Property::HealingOverTime).modifier,
            "mhot" => ingredient.get_modifier(Property::HealingOverTime).multiplier,

            "pot" => ingredient.get_modifier(Property::PoisonOverTime).modifier,
            "mpot" => ingredient.get_modifier(Property::PoisonOverTime).multiplier,

            "hl" => ingredient.get_modifier(Property::HealingLength).modifier,
            "mhl" => ingredient.get_modifier(Property::HealingLength).multiplier,

            "pl" => ingredient.get_modifier(Property::PoisonLength).modifier,
            "mpl" => ingredient.get_modifier(Property::PoisonLength).multiplier,

            "a" => ingredient.get_modifier(Property::Alcohol).modifier,
            "ma" => ingredient.get_modifier(Property::Alcohol).multiplier,

            "w" => ingredient.alchemical_weight as i64,
        }?;

        Ok(self.include_ingredients.as_ref().unwrap().eval_boolean_with_context(&context)?)
    }

    pub fn run(&self) -> Result<()> {
        let mut grimoire_long = load_from_db(Path::new(&self.db_name).to_str().unwrap())?;
        grimoire_long.ingredients.retain(|_, x| self.should_include_ingredient(x).unwrap());
        let character = &grimoire_long.characters[&self.character_name];
        let grimoire = grimoire_long.create_reference(character);

        let mut ga = self.build(grimoire.clone())?;
        let incubator = AlchemyIncubator::new(grimoire);

        create_dir_all("output")?;

        let mut i = 0;

        loop {
            i += 1;
            ga.advance_evolution().unwrap();

            if i % self.output_every != 0 { continue; }
            println!("{}", i);
            
            let mut population = ga.last_population();
            population.sort();

            let phenotypes: Vec<PotionSerializable> = population.into_iter().map(|i| incubator.grow(i.genotype())).collect();

            let folder = Path::new(&self.output_folder);
            let filename = format!("{}.yaml", i);
            let mut file = File::create(folder.join(&filename))?;
            to_writer(&mut file, &phenotypes)?;            
        }   

        Ok(())

    }

}