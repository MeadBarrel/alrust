use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Effect {
    DirectHealing,
    DirectPoison,
    HealingOverTime,
    PoisonOverTime,
    HealingLength,
    PoisonLength,
    Alcohol,
}
