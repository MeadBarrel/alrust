use std::sync::{Arc, Mutex};

use evalexpr::{context_map, Node};
use rand::{rngs::ThreadRng, thread_rng};
use geneticalchemy::{prelude::*};
use grimoire2::prelude::*;
use tracing::info;
use crate::fs::save;
use std::sync::mpsc::Receiver;
use crossterm::event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};

use genetic::{
    operators::{
        crossover::PrecedencePreservativeCrossover, reinsert::ElitistReinserter,
        select::TournamentSelector,
    },
    prelude::Algorithm,
};

use super::{
    config::OptimizatorConfig,
    eexpr::EvalExpressionFitnessElement,
    error::{OptimizationError, Result},
    printer::PopulationsSerializable, message::Message,
};

pub struct Optimizator {
    grimoire: Grimoire,
    optimized_grimoire: OptimizedGrimoire,
    config: OptimizatorConfig,
    pub populations: Arc<Mutex<PopulationsSerializable>>,
}

impl Optimizator {
    pub fn run(&mut self, receiver: Receiver<Message>) -> Result<()> {
        let mut ga = self.algorithm();
        //let mut printer =  PopulationsSerializable::new(self.optimized_grimoire.clone());
        let mut generation = 0;

        loop {
            generation += 1;

            ga.advance_evolution()?;

            if let Ok(Message::Stop) = receiver.try_recv() {
                return Ok(())
            }

            // match receiver.try_recv() {
            //     Ok(value) => match value {
            //         Message::Stop => return Ok(()),
            //         _ => {},
            //     }
            //     Err(_) => {}
            // };

            // let poll_result = poll(Duration::ZERO)
            //     .into_report()
            //     .change_context(OptimizationError::OutputError)
            //     .attach_printable("Error while reading terminal event")?;

            // if poll_result {                

            //     let read_result = read()
            //     .into_report()
            //     .change_context(OptimizationError::OutputError)
            //     .attach_printable("Error while reading terminal event")?;

            //     if let Event::Key(
            //         KeyEvent {
            //             code: KeyCode::Esc,
            //             modifiers: KeyModifiers::NONE,
            //             kind: KeyEventKind::Press,
            //             state: KeyEventState::NONE,
            //         }
            //     ) = read_result {
            //         info!("ESC pressed, stopping");
            //         return Ok(())
            //     }

            // }

            if generation % self.config.output_every != 0 {
                continue;
            }

            // let fitnesses: Vec<ParettoFitness> = ga.last_population().clone().into_iter().map(|x| x.fitness).collect();
            // let best = fitnesses
            //     .into_iter()
            //     .map(|x| x.into_iter().sum::<genetic::NotNan<f64>>())
            //     .max();

            // best.into_iter().for_each(|x| {
            //     println!("Generation: {generation}; Best: {x}")
            // });

            let population = ga.last_population();

            self.populations.lock().unwrap().add_population(population.clone(), generation);

            // save(std::path::Path::new(&output_filename), &printer)
            //     .change_context(OptimizationError::OutputError)?;
        }
    }

    fn algorithm(&self) -> 
        AlchemyGA<PrecedencePreservativeCrossover, TournamentSelector, ElitistReinserter, ThreadRng> 
    {
        let mut rng = thread_rng();
        let initial_pool = self.initial_pool(&mut rng);

        create_alchemy_ga(
            self.fitness_function(),
            self.mutator(),
            self.crossover(),
            self.config.select.clone(),
            self.reinsert(),
            rng,
            initial_pool,
        )
    }

    fn initial_pool<R: rand::Rng>(&self, rng: &mut R) -> Vec<AlchemyGenome> {
        (0..self.config.population_size)
            .into_iter()
            .map(|_| AlchemyGenome::create_random(rng, self.optimized_grimoire.ingredients.len()))
            .collect()
    }

    fn reinsert(&self) -> ElitistReinserter {
        ElitistReinserter::default()        
    }

    fn crossover(&self) -> PrecedencePreservativeCrossover {
        PrecedencePreservativeCrossover::new(self.config.num_children)
    }

    fn mutator(&self) -> AlchemyMutator {
        AlchemyMutator::new(
            self.grimoire.ingredients.len(),
            self.config.mutate.amount_grow_ratio,
            self.config.mutate.min_amount_grow,
            self.config.mutate.num_mutations_amt,
            self.config.mutate.num_mutations_ing,
        )
    }

    fn fitness_function(&self) -> AlchemyFitnessFunction {
        let fitness_elements = self.config
            .effects
            .iter()
            .map(|x| {
                Box::new(EvalExpressionFitnessElement::new(
                    x.clone(),
                    self.config.unknown_multiplier,
                )) as Box<dyn AlchemyFitnessElement>
            })
            .collect();

        AlchemyFitnessFunction::new(
            self.optimized_grimoire.clone(),
            fitness_elements,
            self.config.volume,
        )
    }

    pub fn new(mut grimoire: Grimoire, character: Character, config: OptimizatorConfig) -> Self {
        if let Some(node) = &config.include_ingredients {
            grimoire
                .ingredients
                .retain(|_, ingredient| Self::should_include_ingredient(node, ingredient).unwrap())
        }

        grimoire.ingredients.retain(|name, _| !config.exclude_ingredients.contains(name));

        let optimized_grimoire: OptimizedGrimoire = (&character, &grimoire).into();

        let populations = Arc::new(
            Mutex::new(
                PopulationsSerializable::new(optimized_grimoire.clone())
            )
        );

        Self {
            grimoire,
            optimized_grimoire,
            config,
            populations,
        }
    }

    fn should_include_ingredient(node: &Node, ingredient: &Ingredient) -> Result<bool> {
        let context = context_map! {
            "dh" => ingredient.modifiers[Effect::DirectHealing].term.inner(),
            "mdh" => ingredient.modifiers[Effect::DirectHealing].multiplier.inner(),

            "dp" => ingredient.modifiers[Effect::DirectPoison].term.inner(),
            "mdp" => ingredient.modifiers[Effect::DirectPoison].multiplier.inner(),

            "hot" => ingredient.modifiers[Effect::HealingOverTime].term.inner(),
            "mhot" => ingredient.modifiers[Effect::HealingOverTime].multiplier.inner(),

            "pot" => ingredient.modifiers[Effect::PoisonOverTime].term.inner(),
            "mpot" => ingredient.modifiers[Effect::PoisonOverTime].multiplier.inner(),

            "hl" => ingredient.modifiers[Effect::HealingLength].term.inner(),
            "mhl" => ingredient.modifiers[Effect::HealingLength].multiplier.inner(),

            "pl" => ingredient.modifiers[Effect::PoisonLength].term.inner(),
            "mpl" => ingredient.modifiers[Effect::PoisonLength].multiplier.inner(),

            "a" => ingredient.modifiers[Effect::Alcohol].term.inner(),
            "ma" => ingredient.modifiers[Effect::Alcohol].multiplier.inner(),

            "w" => ingredient.weight as i64,
        }?;

        Ok(node.eval_boolean_with_context(&context)?)
    }
}