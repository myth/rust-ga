use rand::Rng;
use std::fmt;
use std::fmt::Display;

/// TODO: Possibly add Phenotype as associated type and do some Into/From trait magic in Population bounds
/// TODO: Make this into a struct generic over T where T has bounds without Self
pub trait Genotype {
    /// Create a new Genotype
    fn new(rng: &mut impl Rng) -> Self;
    /// Mutate this genotype
    fn mutate(&mut self, rng: &mut impl Rng);
    /// Perform crossover and produce a new offspring
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self;
}

/// Putting fitness into different Phenotype trait for future separation of decode
pub trait Phenotype {
    /// Evaluate the fitness of this Phenotype
    fn fitness(&self) -> f64;
}

/// Individual wraps the T: Genotype + Phenotype with additional metadata
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Individual<T>
where
    T: Genotype + Phenotype + PartialOrd,
{
    pub fitness: f64,
    pub generation: i32,
    pub genotype: T,
}

/// Convenience method to evaluate the fitness of a genotype
impl<T> Individual<T>
where
    T: Genotype + Phenotype + PartialOrd,
{
    pub fn evaluate(&mut self) {
        self.fitness = self.genotype.fitness();
    }
}

/// Convenience method to perform crossover on the underlying genotype
impl<T> Individual<T>
where
    T: Genotype + Phenotype + PartialOrd,
{
    pub fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        Individual {
            generation: self.generation + 1,
            fitness: 0.0,
            genotype: self.genotype.crossover(&other.genotype, rng),
        }
    }
}

/// String representation of an indididual
impl<T> Display for Individual<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Individual {{ F: {:.3}, G: {} }}",
            self.fitness, self.generation
        )
    }
}
