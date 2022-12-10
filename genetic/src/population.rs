use crate::genetic::*;
use crate::individual::*;
use crate::alias::*;


pub trait Population {
    type Genotype;
    type Fitness;
    type Constraint;

    fn from_genomes(
        genomes: Vec<Self::Genotype>, 
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self;

    fn fitnesses(&self) -> Vec<&Self::Fitness>;
}


pub trait RankedPopulation {
    type Advantage;
    type Genotype;
    type Fitness;
    type Constraint;
    type Item; 

    fn from_population(
        population: Individuals<Self::Genotype, Self::Fitness, Self::Constraint>, 
        advantage_function: 
            &Box<dyn AdvantageFunction<Fitness = Self::Fitness, Advantage = Self::Advantage>>)
            -> Self;

    fn best(&self) -> Option<&Self::Item>;
    fn to_individuals(self) -> Individuals<Self::Genotype, Self::Fitness, Self::Constraint>;
}


impl<G, F, C, A> RankedPopulation for RankedIndividuals<G, F, C, A>
    where 
        G: Genotype,
        F: Fitness,
        C: Constraint,
        A: Advantage
{
    type Genotype = G;
    type Fitness = F;
    type Constraint = C;
    type Advantage = A;
    type Item = RankedIndividual<Self::Genotype, Self::Fitness, Self::Constraint, Self::Advantage>;

    fn best(&self) -> Option<&Self::Item> {
        self.iter().max_by_key(|x| &x.advantage)
    }

    fn from_population(
            population: Individuals<Self::Genotype, Self::Fitness, Self::Constraint>, 
            advantage_function: 
                &Box<dyn AdvantageFunction<Fitness = Self::Fitness, Advantage = Self::Advantage>>)
                -> Self {
        let fitnesses = population.fitnesses();
        let advantages = advantage_function.call(fitnesses);
        population.into_iter().zip(advantages.into_iter())
            .map(|(individual, advantage)| RankedIndividual::new(individual, advantage))
            .collect()
                
    }

    fn to_individuals(self) -> Individuals<Self::Genotype, Self::Fitness, Self::Constraint> {
        self.into_iter().map(|x| x.individual).collect()
    }
}


impl<G, F, C> Population for Individuals<G, F, C> 
    where 
        G: Genotype,
        F: Fitness,
        C: Constraint
{
    type Genotype = G;
    type Fitness = F;
    type Constraint = C;

    fn from_genomes(
        genomes: Vec<Self::Genotype>, 
        fitness_function: &FitnessFunctionAlias<Self::Genotype, Self::Fitness, Self::Constraint>
    ) -> Self {
        genomes.into_iter().map(|genome| Individual::from_genome(genome, fitness_function)).collect()
    }

    fn fitnesses(&self) -> Vec<&Self::Fitness> {
        self.iter().map(|x| &x.fitness).collect()
    }
}