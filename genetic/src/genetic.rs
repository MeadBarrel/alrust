pub trait Genotype: Clone {}
pub trait Fitness: Clone {}
pub trait Constraint: Clone + Ord {}
pub trait Advantage: Clone + Ord {}
pub trait Locus: Clone {}

pub type VectorEncoded<L> = Vec<L>;
impl<L: Locus> Genotype for VectorEncoded<L> {}


pub trait FitnessFunction {
    type Genotype: Genotype;
    type Fitness: Fitness;
    type Constraint: Constraint;

    fn fitness(&self, genome: &Self::Genotype) -> Self::Fitness;
    fn constraint(&self, genome: &Self::Genotype) -> Self::Constraint;
}

pub trait AdvantageFunction {
    type Fitness: Fitness;
    type Advantage: Advantage;

    fn call(&self, fitnesses: Vec<&Self::Fitness>) -> Vec<Self::Advantage>;
}
