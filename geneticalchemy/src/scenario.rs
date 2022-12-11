use grimoire::data::Ingredient;
use grimoire::types::Property;
use crate::genetic::AlchemyFitnessElement;
use crate::fitness::AlchemyEffectFitness;


pub trait Scenario {
    fn should_include_ingredient(&self, ingredient: &Ingredient) -> bool;
    fn fitness_element(&self) -> Box<dyn AlchemyFitnessElement>;
}


pub struct Scenarios {
    scenarios: Vec<Box<dyn Scenario>>
}


impl Scenarios {
    pub fn new(scenarios: Vec<Box<dyn Scenario>>) -> Self {
        Self {
            scenarios
        }
    }

    pub fn fitness_functions(&self) -> Vec<Box<dyn AlchemyFitnessElement>> {
        self.scenarios.iter().map(|x| x.fitness_element()).collect()
    }

    pub fn should_include_ingredient(&self, ingredient: &Ingredient) -> bool {
        self.scenarios.iter().any(|x| x.should_include_ingredient(ingredient))
    }

    pub fn filter_ingredients(&self, ingredients: Vec<Ingredient>) -> Vec<Ingredient> {
        ingredients.into_iter().filter(|x| self.should_include_ingredient(x)).collect()
    }
}

pub struct EffectScenario {
    effect: Property,
}


impl EffectScenario {
    pub fn new(effect: Property) -> Self {
        Self { effect }
    }
}


impl Scenario for EffectScenario {
    fn fitness_element(&self) -> Box<dyn AlchemyFitnessElement> {
        Box::new(AlchemyEffectFitness::new(self.effect, false))
    }

    fn should_include_ingredient(&self, ingredient: &Ingredient) -> bool {
        let modifier = ingredient.modifiers.iter().find(|(x, _)| *x == self.effect);
        match modifier {
            None => false,
            Some((_, x)) => x.modifier > 0. || x.multiplier > 0.
        }
    }
}