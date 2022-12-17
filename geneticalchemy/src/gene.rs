use genetic::genetic::Locus;

#[derive(Eq, Clone, Debug)]
pub struct AlchemyGene {
    pub ingredient_index: usize,
    pub amount: u64,
}

impl Into<(usize, u64)> for AlchemyGene {
    fn into(self) -> (usize, u64) {
        (self.ingredient_index, self.amount)
    }
}

impl Locus for AlchemyGene {}

impl PartialEq for AlchemyGene {
    fn eq(&self, other: &AlchemyGene) -> bool {
        self.ingredient_index == other.ingredient_index
    }
}
