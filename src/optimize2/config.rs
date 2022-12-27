use serde::Deserialize;
use grimoire_serde::modify::GrimoireUpdateSerializable;
use genetic::operators::TournamentSelector;
use evalexpr::Node;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct OptimizatorConfig {
    pub grimoire: GrimoireUpdateSerializable,
    
    // Printing parameters
    pub output_every: usize,

    // Population parameters
    pub population_size: usize,
    pub num_children: usize,

    // Operators parameters
    pub select: TournamentSelector,
    pub mutate: MutatorConfig,

    // Desired potion parameters
    pub volume: f64,
    pub effects: Vec<Node>,
    pub include_ingredients: Option<Node>,
    pub unknown_multiplier: f64,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct MutatorConfig {
    pub amount_grow_ratio: f64,
    pub min_amount_grow: u64,
    pub num_mutations_amt: usize,
    pub num_mutations_ing: usize,
}

impl Default for OptimizatorConfig {
    fn default() -> Self {
        Self {
            grimoire: GrimoireUpdateSerializable::default(),
            mutate: MutatorConfig::default(),
            select: TournamentSelector::default(),
            population_size: 200,
            output_every: 10000,
            volume: 40.,
            effects: Vec::default(),
            include_ingredients: None,
            unknown_multiplier: 1.,
            num_children: 2,
        }
    }    
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