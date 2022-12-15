use evalexpr::*;
use std::error::Error;
use std::fmt::Display;

use serde::Deserialize;
use grimoire::types::Property;
use grimoire::mix::*;
use geneticalchemy::incubator::AlchemyFitnessElement;


#[derive(Debug)]
pub struct UnknownIdentifierError {
    identifier: String
}


impl UnknownIdentifierError {
    pub fn new(identifier: &str) -> Self {
        Self { identifier: identifier.to_string() }
    }
}


impl Display for UnknownIdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Unknown identifier: {}", self.identifier))
    }
}


impl Error for UnknownIdentifierError {}

#[derive(Deserialize, Clone)]
pub struct EvalExpressionFitnessElement {
    expression: Node,
    unknown_multiplier: f64,
}


impl EvalExpressionFitnessElement {
    pub fn new(expression: Node, unknown_multiplier: f64) -> Self {
        Self { expression, unknown_multiplier }
    }

    fn get_identifier_value(&self, identifier: &str, mix: &Mix) -> Result<f64, UnknownIdentifierError> {
        match identifier {
            "dh" => Ok(mix_effect(mix, Property::DirectHealing).known_or(|x| x * self.unknown_multiplier)),
            "dp" => Ok(mix_effect(mix, Property::DirectPoison).known_or(|x| x * self.unknown_multiplier)),
            "hot" => Ok(mix_effect(mix, Property::HealingOverTime).known_or(|x| x * self.unknown_multiplier)),
            "pot" => Ok(mix_effect(mix, Property::PoisonOverTime).known_or(|x| x * self.unknown_multiplier)),
            "hl" => Ok(mix_effect(mix, Property::HealingLength).known_or(|x| x * self.unknown_multiplier)),
            "pl" => Ok(mix_effect(mix, Property::PoisonLength).known_or(|x| x * self.unknown_multiplier)),
            "a" => Ok(mix_effect(mix, Property::Alcohol).known_or(|x| x * self.unknown_multiplier)),
            "volume" => Ok(mix_volume(mix)),
            _ => Err(UnknownIdentifierError::new(identifier))
        }
    }
}


impl AlchemyFitnessElement for EvalExpressionFitnessElement {
    fn fitness(&self, mix: &grimoire::prelude::Mix) -> f64 {        
        let mut context = HashMapContext::new();
        for identifier in self.expression.iter_identifiers() {
            context.set_value(
                identifier.to_string(), 
                Value::Float(self.get_identifier_value(identifier, mix).unwrap())
            ).unwrap();
        };

        self.expression.eval_float_with_context(&context).unwrap()
    }
}