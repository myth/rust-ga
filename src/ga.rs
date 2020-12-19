use super::cli::Cli;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

/// Interface for working with various populations
pub trait Population {
    fn evolve(&mut self, args: &Cli) -> f64;
}

pub trait Genotype {
    /// Mutate this genotype
    fn mutate(&mut self, rng: &mut impl Rng);
    /// Perform crossover and produce a new offspring
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self;
}

pub trait Phenotype {
    /// Evaluate the fitness of this Phenotype
    fn evaluate(&self) -> f64;
}

pub struct EightQueens {
    genome: [i32; 8],
}

impl Genotype for EightQueens {
    /// Mutate this genomy by swapping two random positions
    fn mutate(&mut self, rng: &mut impl Rng) {
        let a = rng.gen_range(0, 8);
        let b = rng.gen_range(0, 8);
        self.genome[a] = self.genome[b];
    }

    /// Create a new specimen by performing crossover with other at random index
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        let mut genome = [0; 8];
        let index = rng.gen_range(0, 8);

        for i in 0..index {
            genome[i] = self.genome[i];
        }
        for i in index..8 {
            genome[i] = other.genome[i];
        }

        Self { genome }
    }
}

impl Phenotype for EightQueens {
    fn evaluate(&self) -> f64 {
        0.0
    }
}

/// Create a new, empty population
pub fn create_eight_queens(population_size: i32) -> StandardPopulation<EightQueens> {
    println!(
        "Creating StandardPopulation with {} individuals",
        population_size
    );

    let mut rng = thread_rng();
    let mut genotypes = vec![];

    for _ in 0..population_size {
        genotypes.push(EightQueens { genome: rng.gen() });
    }

    StandardPopulation {
        rng,
        epoch: 0,
        history: vec![],
        genotypes,
    }
}

struct EvolutionStats {
    fitness: f64,
}

/// Simple sandbox population
pub struct StandardPopulation<G> {
    rng: rand::rngs::ThreadRng,
    epoch: i32,
    history: Vec<EvolutionStats>,
    genotypes: Vec<G>,
}

impl<G> StandardPopulation<G>
where
    G: Genotype + Phenotype,
{
    /// Performs mutation in the population
    fn mutate(&mut self, rate: f64) {
        let mut count = 0;

        for g in &mut self.genotypes {
            if self.rng.gen_bool(rate) {
                g.mutate(&mut self.rng);
                count += 1;
            }
        }

        if count > 0 {
            println!("Performed {} mutations this epoch", count);
        }
    }

    /// Performs crossover in the population
    fn crossover(&mut self, rate: f64) {
        let mut count = 0;

        for _g in &mut self.genotypes {
            if self.rng.gen_bool(rate) {
                count += 1;
            }
        }

        if count > 0 {
            println!("Performed {} crossovers this epoch", count);
        }
    }

    /// Perform one epoch cycle
    fn next(&mut self, args: &Cli, start: &SystemTime) -> f64 {
        self.epoch += 1;

        self.mutate(args.mutation_rate);
        self.crossover(args.crossover_rate);

        let fitness = self.rng.gen_range(0.0, 0.9);
        let stats = EvolutionStats { fitness };

        self.history.push(stats);

        if self.epoch % 10 == 0 {
            println!(
                "[{}/{}] F: {:.2} (Elapsed {:.4}s)",
                self.epoch,
                args.max_epochs,
                fitness,
                start.elapsed().unwrap().as_secs_f32()
            );
        }

        fitness
    }
}

/// Implementation of the Population trait for the simple sandbox population
impl<G> Population for StandardPopulation<G>
where
    G: Genotype + Phenotype,
{
    /// Evolve this population based on the given command line arguments
    fn evolve(&mut self, args: &Cli) -> f64 {
        println!(
            "Attempting to evolve Standard population to target fitness {} in maximum {} epochs",
            args.target_fitness, args.max_epochs
        );

        let start = SystemTime::now();
        let mut best_fitness = 0.0;

        for _ in 1..args.max_epochs + 1 {
            let epoch_fitness = self.next(args, &start);

            if epoch_fitness > best_fitness {
                best_fitness = epoch_fitness;
            }
        }

        println!(
            "Completed after {:.4}s with {} as best candidate",
            start.elapsed().unwrap().as_secs_f32(),
            best_fitness
        );

        best_fitness
    }
}
