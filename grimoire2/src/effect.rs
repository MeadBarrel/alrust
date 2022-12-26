use serde::{Serialize, Deserialize};
use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Effect {
    DirectHealing,
    DirectPoison,
    HealingOverTime,
    PoisonOverTime,
    HealingLength,
    PoisonLength,
    Alcohol,
}
