use crate::{alias::*, genetic::*};

// Individual --------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
/// Represents an individual in a population.
pub struct IndividualStruct<G, F> {
    /// The genotype of the individual.
    pub genotype: G,
    /// The fitness of the individual.
    pub fitness: F,
    /// The constraints of the individual.
    pub constraint: Constraint,
}

impl<G, F> IndividualStruct<G, F>
where
    G: Genotype,
    F: Fitness,
{
    /// Creates a new `IndividualStruct` with the given `genotype`, `fitness`, and `constraints`.
    pub fn new(genotype: G, fitness: F, constraint: Constraint) -> Self {
        Self {
            genotype,
            fitness,
            constraint,
        }
    }

    /// Creates a new `IndividualStruct` with the given `genome` and `fitness_function`. The `genotype` is
    /// set to the value of the `genome`, the `fitness` is set to the result of calling the `fitness` method
    /// of `fitness_function` on the `genome`, and the `constraints` are set to the result of calling the
    /// `constraint` method of `fitness_function` on the `genome`.
    pub fn from_genome(genome: G, fitness_function: &FitnessFunctionAlias<G, F>) -> Self {
        let fitness = fitness_function.fitness(&genome);
        let constraint = fitness_function.constraint(&genome);
        Self::new(genome, fitness, constraint)
    }
}

/// Represents an individual in a population.
pub trait Individual: Clone {
    /// The genotype type associated with this individual.
    type Genotype: Genotype;
    /// The fitness type associated with this individual.
    type Fitness: Fitness;

    /// Returns a reference to the genotype of this individual.
    fn genotype(&self) -> &Self::Genotype;

    /// Returns a reference to the fitness of this individual.
    fn fitness(&self) -> &Self::Fitness;

    /// Returns a reference to the constraints of this individual.
    fn constraint(&self) -> Constraint;

    /// Consumes the individual and returns its genotype
    fn into_genotype(self) -> Self::Genotype;

    /// Creates a new instance of this individual with the given `genome` and `fitness_function`. The `genotype` is
    /// set to the value of the `genome`, the `fitness` is set to the result of calling the `fitness` method
    /// of `fitness_function` on the `genome`, and the `constraints` are set to the result of calling the
    /// `constraint` method of `fitness_function` on the `genome`.
    fn from_genome(
        genome: Self::Genotype,
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness>,
    ) -> Self;
}

impl<G, F> Individual for IndividualStruct<G, F>
where
    G: Genotype,
    F: Fitness,
{
    type Genotype = G;
    type Fitness = F;

    fn genotype(&self) -> &Self::Genotype {
        &self.genotype
    }

    fn fitness(&self) -> &Self::Fitness {
        &self.fitness
    }

    fn constraint(&self) -> Constraint {
        self.constraint
    }

    fn into_genotype(self) -> Self::Genotype {
        self.genotype
    }

    fn from_genome(
        genome: Self::Genotype,
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness>,
    ) -> Self {
        IndividualStruct::<G, F>::from_genome(genome, fitness_function)
    }
}
