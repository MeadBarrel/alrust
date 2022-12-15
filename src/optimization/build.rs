use error_stack::*;

use rand::{
    thread_rng,
    rngs::ThreadRng,
};

use geneticalchemy::{
    algorithm::{AlchemyGA, create_alchemy_ga},
    genetic::{random_genome, AlchemyFitnessElement, AlchemyFitnessFunction},
    mutate::AlchemyMutator,
};

use grimoire::{
    data::Compendium,
    optimized::OptimizedGrimoir,
};

use genetic:: {
    operators:: {
        select::TournamentSelector,
        reinsert::ElitistReinserter,
        crossover::PrecedencePreservativeCrossover,
    }, prelude::Algorithm
};

use crate::optimization::print::PopulationPrinter;

use super::config::OptimizatorConfig;
use super::eexpr::EvalExpressionFitnessElement;
use super::error::{Result, OptimizationError};
use super::print::ToYaml;

pub struct Optimizator {
    grimoire: Compendium,
    optimized_grimoire: OptimizedGrimoir,
    ga: AlchemyGA<PrecedencePreservativeCrossover, TournamentSelector, ElitistReinserter, ThreadRng>,
    printer: ToYaml,
    output_every: usize,
}


impl Optimizator {
    pub fn run(&mut self) -> Result<Self> {
        let mut generation = 0;
        loop {
            generation += 1;
            
            self.ga.advance_evolution().change_context(OptimizationError::OptimizationError)?;

            if generation % self.output_every != 0 { continue; }

            println!("Generation: {}", generation);
            
            let population = self.ga.last_population();

            self.printer.print(&self.optimized_grimoire, &population, generation)?;
        }
    }

    pub fn load(filename: &str) -> Result<Self> {
        use serde_yaml::from_reader;
        use std::fs::File;

        let file = File::open(filename).into_report().change_context(OptimizationError::LoadError)?;
        let config = from_reader(file).into_report().change_context(OptimizationError::LoadError)?;
        Self::new(config)
    }

    pub fn new(config: OptimizatorConfig) -> Result<Self> {
        let mut rng = thread_rng();
        let grimoire = config.grimoire.build().change_context(OptimizationError::LoadError)?;
        let character = grimoire.characters.get(&config.character).ok_or_else(
            || Report::new(OptimizationError::LoadError)
                .attach_printable(format!("Character not found: {}", config.character))
        )?;
        let optimized_grimoire = grimoire.create_reference(character);

        let mutate = AlchemyMutator::new(
            grimoire.ingredients.len(),
            config.mutate.amount_grow_ratio,
            config.mutate.min_amount_grow,
            config.mutate.num_mutations_amt,
            config.mutate.num_mutations_ing
        );

        let fitness_elements = config.effects.iter().map(
            |x| Box::new(EvalExpressionFitnessElement::new(x.clone())) 
                    as Box<dyn AlchemyFitnessElement>
        ).collect();

        let fitness_function = AlchemyFitnessFunction::new(
            optimized_grimoire.clone(),
            fitness_elements,
            config.volume,
        );

        let crossover = PrecedencePreservativeCrossover::new(config.num_children);
        let reinsert = ElitistReinserter::default();
        
        let initial_pool = (0..config.population_size).into_iter()
            .map(|_| random_genome(&mut rng, &optimized_grimoire)).collect();

        let ga = create_alchemy_ga(
            fitness_function, 
            mutate, 
            crossover, 
            config.select, 
            reinsert, 
            rng, 
            initial_pool
        );

        Ok (
            Self { 
                grimoire, 
                optimized_grimoire, 
                ga, 
                printer: 
                config.printer, 
                output_every: 
                config.output_every 
            }
        )
    }
}
