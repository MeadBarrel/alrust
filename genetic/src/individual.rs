use crate::alias::*;
use crate::genetic::*;

// Individual --------------------------------------------------------------------------------------


#[derive(Debug, Clone)]
/// Represents an individual in a population.
pub struct IndividualStruct<G, F, C> {
    /// The genotype of the individual.
    pub genotype: G,
    /// The fitness of the individual.
    pub fitness: F,
    /// The constraints of the individual.
    pub constraints: C,
}

impl<G, F, C> IndividualStruct<G, F, C>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint,
{
    /// Creates a new `IndividualStruct` with the given `genotype`, `fitness`, and `constraints`.
    pub fn new(genotype: G, fitness: F, constraints: C) -> Self {
        Self {
            genotype,
            fitness,
            constraints,
        }
    }

    /// Creates a new `IndividualStruct` with the given `genome` and `fitness_function`. The `genotype` is
    /// set to the value of the `genome`, the `fitness` is set to the result of calling the `fitness` method
    /// of `fitness_function` on the `genome`, and the `constraints` are set to the result of calling the
    /// `constraint` method of `fitness_function` on the `genome`.
    pub fn from_genome(
        genome: G,
        fitness_function: &FitnessFunctionAlias<G, F,  C>
    ) -> Self {
        let fitness = fitness_function.fitness(&genome);
        let constraint = fitness_function.constraint(&genome);
        Self::new(genome, fitness, constraint )
    }
}


/// Represents an individual in a population.
pub trait Individual: Clone {
    /// The genotype type associated with this individual.
    type Genotype: Genotype;
    /// The fitness type associated with this individual.
    type Fitness: Fitness;
    /// The constraint type associated with this individual.
    type Constraint: Constraint;

    /// Returns a reference to the genotype of this individual.
    fn genotype(&self) -> &Self::Genotype;

    /// Returns a reference to the fitness of this individual.
    fn fitness(&self) -> &Self::Fitness;

    /// Returns a reference to the constraints of this individual.
    fn constraint(&self) -> &Self::Constraint;

    /// Creates a new instance of this individual with the given `genome` and `fitness_function`. The `genotype` is
    /// set to the value of the `genome`, the `fitness` is set to the result of calling the `fitness` method
    /// of `fitness_function` on the `genome`, and the `constraints` are set to the result of calling the
    /// `constraint` method of `fitness_function` on the `genome`.
    fn from_genome(
        genome: Self::Genotype,
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self;
}

impl<G, F, C> Individual for IndividualStruct<G, F, C>
    where
        G: Genotype,
        F: Fitness,
        C: Constraint
{
    type Genotype = G;
    type Fitness = F;
    type Constraint = C;

    fn genotype(&self) -> &Self::Genotype {
        &self.genotype
    }

    fn fitness(&self) -> &Self::Fitness {
        &self.fitness
    }

    fn constraint(&self) -> &Self::Constraint {
        &self.constraints
    }

    fn from_genome(genome: Self::Genotype, fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>) -> Self {
        IndividualStruct::<G, F, C>::from_genome(genome, fitness_function)
    }
}


// RankedIndividual --------------------------------------------------------------------------------

/// `RankedIndividualStruct` is used to store an individual in a population along with its
/// `advantage` which indicates how well-suited the individual is for a certain objective compared
/// to other individuals in the population.
#[derive(Debug, Clone)]
pub struct RankedIndividualStruct<I, A>
{
    /// The individual in the population.
    pub individual: I,
    /// The advantage of the individual compared to others in the population.
    pub advantage: A,
}

impl<I, A> RankedIndividualStruct<I, A>
    where
        I: Individual,
        A: Advantage,
{
    /// Creates a new instance of `RankedIndividualStruct` with the given individual and advantage.
    pub fn new(individual: I, advantage: A) -> Self {
        Self {
            individual,
            advantage,
        }
    }
}

pub trait RankedIndividual: Clone {
    /// The type of the individual.
    type Individual: Individual<Genotype=Self::Genotype, Fitness=Self::Fitness, Constraint=Self::Constraint>;
    /// The type of the advantage value.
    type Advantage: Advantage;
    /// The type of the genotype of the individual.
    type Genotype: Genotype;
    /// The type of the fitness value of the individual.
    type Fitness: Fitness;
    /// The type of the constraint value of the individual.
    type Constraint: Constraint;

    /// Returns a reference to the individual in the `RankedIndividual` struct.
    fn individual(&self) -> &Self::Individual;
    /// Returns a reference to the advantage value in the `RankedIndividual` struct.
    fn advantage(&self) -> &Self::Advantage;

    /// Converts the `RankedIndividual` struct into its individual component.
    fn into_individual(self) -> Self::Individual;
    /// Converts an individual and an advantage value into a `RankedIndividual` struct.
    fn from_individual(individual: Self::Individual, advantage: Self::Advantage) -> Self;
}

impl<I, A> RankedIndividual for RankedIndividualStruct<I, A>
    where
        I: Individual,
        A: Advantage,
{
    type Individual = I;
    type Advantage = A;
    type Genotype = I::Genotype;
    type Fitness = I::Fitness;
    type Constraint = I::Constraint;

    fn individual(&self) -> &Self::Individual {
        &self.individual
    }

    fn advantage(&self) -> &Self::Advantage {
        &self.advantage
    }

    fn into_individual(self) -> Self::Individual {
        self.individual
    }

    fn from_individual(individual: Self::Individual, advantage: Self::Advantage) -> Self {
        Self {
            individual, advantage
        }
    }

}


#[cfg(test)]
pub mod tests {
    use super::*;

    impl Constraint for Vec<u32> {}
    impl Fitness for u32 {}
    impl Locus for u32 {}

    struct TestFitnessFunction {
        expected_fitness: u32,
        expected_constraints: Vec<u32>,
    }

    impl FitnessFunction for TestFitnessFunction {
        type Genotype = Vec<u32>;
        type Constraint = Vec<u32>;
        type Fitness = u32;
        
        fn fitness(&self, _genome: &Vec<u32>) -> u32 {
            self.expected_fitness
        }

        fn constraint(&self, _genome: &Vec<u32>) -> Vec<u32> {
            self.expected_constraints.clone()
        }
    }

    #[test]
    fn test_individual_struct_new() {
        let genotype = vec![1, 2, 3];
        let fitness = 10;
        let constraints = vec![1, 0, 1];

        let individual = IndividualStruct::new(genotype.clone(), fitness, constraints.clone());

        assert_eq!(individual.genotype, genotype);
        assert_eq!(individual.fitness, fitness);
        assert_eq!(individual.constraints, constraints);
    }

    #[test]
    fn test_individual_struct_from_genome() {
        let genotype = vec![1, 2, 3];
        let fitness = 10;
        let constraints = vec![1, 0, 1];
        let fitness_function = TestFitnessFunction {
            expected_fitness: fitness,
            expected_constraints: constraints.clone(),
        };

        let individual = IndividualStruct::from_genome(
            genotype.clone(), &fitness_function);

        assert_eq!(individual.genotype, genotype);
        assert_eq!(individual.fitness, fitness);
        assert_eq!(individual.constraints, constraints);
    }

}