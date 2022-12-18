use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Clone, Copy, Debug)]
pub enum Effect {
    DirectHealing,
    DirectPoison,
    HealingOverTime,
    PoisonOverTime,
    HealingLength,
    PoisonLength,
    Alcohol,
}
