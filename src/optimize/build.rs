use error_stack::*;
use evalexpr::{context_map, Node};

use rand::{rngs::ThreadRng, thread_rng};

use geneticalchemy::prelude::*;

use grimoire2::prelude::*;

use genetic::{
    operators::{
        crossover::PrecedencePreservativeCrossover, reinsert::ElitistReinserter,
        select::TournamentSelector,
    },
    prelude::Algorithm,
};

use super::print::PopulationPrinter;

use super::{
    config::OptimizatorConfig,
    eexpr::EvalExpressionFitnessElement,
    error::{OptimizationError, Result},
    print::ToJson,
};

pub struct Optimizator {
    grimoire: Grimoire,
    optimized_grimoire: OptimizedGrimoire,
    ga: AlchemyGA<
        PrecedencePreservativeCrossover,
        TournamentSelector,
        ElitistReinserter,
        ThreadRng,
    >,
    printer: ToJson,
    output_every: usize,
}

impl Optimizator {
    pub fn run(&mut self) -> Result<Self> {
        let mut generation = 0;
        loop {
            generation += 1;

            self.ga
                .advance_evolution()
                .change_context(OptimizationError::OptimizationError)?;

            if generation % self.output_every != 0 {
                continue;
            }

            println!("Generation: {}", generation);

            let population = self.ga.last_population();

            self.printer
                .print(&self.optimized_grimoire, &population, generation)?;
        }
    }

    pub fn new(mut grimoire: Grimoire, config: OptimizatorConfig) -> Result<Self> {
        let mut rng = thread_rng();

        if let Some(node) = config.include_ingredients {
            grimoire
                .ingredients
                .retain(|_, ingredient| Self::should_include_ingredient(&node, ingredient).unwrap())
        }

        let character = grimoire.characters.get(&config.character).ok_or_else(|| {
            Report::new(OptimizationError::LoadError)
                .attach_printable(format!("Character not found: {}", config.character))
        })?;
        let optimized_grimoire: OptimizedGrimoire = (character, &grimoire).into();

        let mutate = AlchemyMutator::new(
            grimoire.ingredients.len(),
            config.mutate.amount_grow_ratio,
            config.mutate.min_amount_grow,
            config.mutate.num_mutations_amt,
            config.mutate.num_mutations_ing,
        );

        let fitness_elements = config
            .effects
            .iter()
            .map(|x| {
                Box::new(EvalExpressionFitnessElement::new(
                    x.clone(),
                    config.unknown_multiplier,
                )) as Box<dyn AlchemyFitnessElement>
            })
            .collect();

        let fitness_function = AlchemyFitnessFunction::new(
            optimized_grimoire.clone(),
            fitness_elements,
            config.volume,
        );

        let crossover = PrecedencePreservativeCrossover::new(config.num_children);
        let reinsert = ElitistReinserter::default();

        let initial_pool = (0..config.population_size)
            .into_iter()
            .map(|_| AlchemyGenome::create_random(&mut rng, optimized_grimoire.ingredients.len()))
            .collect();

        let ga = create_alchemy_ga(
            fitness_function,
            mutate,
            crossover,
            config.select,
            reinsert,
            rng,
            initial_pool,
        );

        Ok(Self {
            grimoire,
            optimized_grimoire,
            ga,
            printer: config.print,
            output_every: config.output_every,
        })
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
        }
        .into_report()
        .change_context(OptimizationError::LoadError)
        .attach_printable("Failed to determine wether to include an ingredient")?;

        node.eval_boolean_with_context(&context)
            .into_report()
            .change_context(OptimizationError::LoadError)
            .attach_printable("Failed to determine wether to include an ingredient")
    }
}