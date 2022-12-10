use std::fmt::format;
use std::fs::File;
use std::path::Path;
use std::borrow::BorrowMut;
use std::cmp::min;
use std::cell::RefCell;
use std::fs::create_dir_all;
use genetic::alias::RankedIndividuals;
use rand::prelude::*;
use anyhow::Result;
use serde_yaml::{from_reader, to_writer};

use genetic::prelude::*;
use serde::Deserialize;
use grimoire::prelude::*;
use crate::algorithm::*;
use crate::fitness::*;
use crate::genetic::*;
use crate::mutate::*;
use crate::incubator::*;



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
    pub tournament: TournamentConfig,
    pub population_size: u64,
    pub output_every: usize,
    pub output_folder: String,
    pub desired_volume: f64,
    pub desired_effects: Vec<DesiredEffects>,
    pub num_children: usize,
}


impl Default for GAConfig {
    fn default() -> Self {
        Self {
            db_name: "db.sqlite".to_string(),
            character_name: "default".to_string(),
            mutate: MutatorConfig::default(),
            tournament: TournamentConfig::default(),
            population_size: 100,
            output_every: 1000,
            output_folder: "output".to_string(),
            desired_volume: 40.,
            desired_effects: vec![DesiredEffects::MaximizeDH],
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


    pub fn build(&self, grimoire: OptimizedGrimoir) -> Result<AlchemyGA> {
        let mut rng_ = thread_rng();
        let rng = RefCell::new(thread_rng());

        let mutate = AlchemyMutator::new(
            rng.clone(), 
            grimoire.ingredients.len(), 
            self.mutate.amount_grow_ratio, 
            self.mutate.min_amount_grow,
            self.mutate.num_mutations_amt,
            self.mutate.num_mutations_ing,
        );

        let constraint_function = Box::new(DesiredVolumeConstraint::new(self.desired_volume));
        let fitness_elements = self.desired_effects.iter().map(
            |x| self.fitness_element_from_de(x.clone())
        ).collect();

        let fitness_function = AlchemyFitnessFunction::new(
            grimoire.clone(),
            fitness_elements,
            vec![constraint_function],
        );

        let crossover = PrecedencePreservativeCrossover::new(self.num_children, rng.clone());
        let select = TournamentSelector::new(self.tournament.clone(), rng.clone());
        let reinsert = ElitistReinserter::new(Box::new(ParettoAdvantageFunction::default()));

        let initial_pool = (0..self.population_size).into_iter()
            .map(|_| random_genome(&mut rng_, &grimoire)).collect();

        Ok(create_alchemy_ga(fitness_function, mutate, crossover, select, reinsert, initial_pool))
    }

    fn fitness_element_from_de(&self, desired_effect: DesiredEffects) -> Box<dyn AlchemyFitnessElement> {
        match desired_effect {
            DesiredEffects::MaximizeDH => Box::new(AlchemyEffectFitness::new(Property::DirectHealing, false)),
            DesiredEffects::MaximizeDP => Box::new(AlchemyEffectFitness::new(Property::DirectPoison, false)),
            DesiredEffects::MaximizeHOT => Box::new(AlchemyEffectFitness::new(Property::HealingOverTime, false)),
            DesiredEffects::MaximizePOT => Box::new(AlchemyEffectFitness::new(Property::PoisonOverTime, false)),
            DesiredEffects::MaximizeHL => Box::new(AlchemyEffectFitness::new(Property::HealingLength, false)),
            DesiredEffects::MaximizePL => Box::new(AlchemyEffectFitness::new(Property::PoisonLength, false)),
            DesiredEffects::MaximizeA => Box::new(AlchemyEffectFitness::new(Property::Alcohol, false)),
        }
    }

    pub fn run(&self) -> Result<()> {
        let grimoire_long = load_from_db(Path::new(&self.db_name).to_str().unwrap())?;
        let character = &grimoire_long.characters[&self.character_name];
        let grimoire = grimoire_long.create_reference(character);

        let ga = self.build(grimoire.clone())?;
        let advantage_function = ParettoAdvantageFunction::default();
        let incubator = AlchemyIncubator::new(grimoire);

        create_dir_all("output")?;

        for (i, population) in ga.enumerate() {
            if !(i % self.output_every == 0) { continue; }
            println!("{}", i);

            let mut ranked = RankedIndividuals::from_population(population, &advantage_function);
            ranked.sort_by_key(|x| x.advantage.clone());
            ranked.reverse();
            println!("{:?}", ranked[0].clone());
            println!("{:?}", incubator.grow(&ranked[0].individual.genotype));
            let printable: Vec<PrintableIndividual<ParettoAdvantage, AlchemyPhenotype>> = ranked.into_iter().map(
                |x| PrintableIndividual::new(
                    incubator.grow(&x.individual.genotype), x.advantage.clone()
                )
            ).collect();
            
            let mut folder = Path::new(&self.output_folder);
            let filename = format!("{}.yaml", i);
            let mut file = File::create(folder.join(&filename))?;
            to_writer(&mut file, &printable)?;
        };

        Ok(())
    }
}