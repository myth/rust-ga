use super::cli::Options;
use fmt::Display;
use rand::{thread_rng, Rng};
use std::fmt;
use std::time::SystemTime;
use structopt::clap::arg_enum;

/// Interface for working with various populations
pub trait Population {
    /// Create a new population based on the given options
    fn new(options: Options) -> Self;
    /// Remove this?
    fn evolve(&mut self);
}

/// TODO: Possibly add Phenotype as associated type and do some Into/From trait magic in Population bounds
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

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available parent selection strategies
    #[derive(Debug)]
    pub enum ParentSelection {
        RouletteWheel,
        StochasticUniversalSampling,
        TournamentSelection,
        RankSelection,
    }
}

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available survivor selection strategies
    #[derive(Debug)]
    pub enum SurvivorSelection {
        AgeBased,
        FitnessBased,
    }
}

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available population models
    #[derive(Debug)]
    pub enum PopulationModel {
        SteadyState,
        Generational,
    }
}

/// Basic statistics container
#[derive(Debug, Default)]
struct EvolutionStats {
    /// Which generation these stats represent
    generation: i32,
    /// Maximum number of generations in the evolution
    max_generations: i32,
    /// Best fitness achieved this generation
    fitness: f64,
    /// The total elapsed time at this generation
    elapsed: f32,
    /// The total number of mutations this generation
    mutations: i32,
    /// The total number of crossovers this generation
    crossovers: i32,
}

/// Individual wraps the T: Genotype + Phenotype with additional metadata
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Individual<T>
where
    T: Genotype + Phenotype + PartialOrd,
{
    fitness: f64,
    generation: i32,
    genotype: T,
}

/// Simple sandbox population
#[derive(Debug)]
pub struct StandardPopulation<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    options: Options,
    rng: rand::rngs::ThreadRng,
    stats: EvolutionStats,
    history: Vec<EvolutionStats>,
    population: Vec<Individual<T>>,
    started: SystemTime,
}

/// String representation of the statistics container
impl fmt::Display for EvolutionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}/{}] F: {:.3} (Elapsed {:.4}s) C: {} M: {}",
            self.generation,
            self.max_generations,
            self.fitness,
            self.elapsed,
            self.crossovers,
            self.mutations
        )
    }
}

impl<T> Individual<T>
where
    T: Genotype + Phenotype + PartialOrd,
{
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        Individual {
            generation: 0,
            fitness: 0.0,
            genotype: self.genotype.crossover(&other.genotype, rng),
        }
    }
}

/// Select a parent using roulette wheel selection
fn roulette_wheel_select<'a, T>(
    population: &'a Vec<Individual<T>>,
    s: f64,
    rng: &mut impl Rng,
) -> usize
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    let mut p = 0.0;
    let t = rng.gen_range(0.0, s);

    for (i, individual) in population.iter().enumerate() {
        p += individual.fitness;

        if p >= t {
            return i;
        }
    }

    0
}

/// Perform mutation on a population with a given mutation rate
fn mutate<'a, T>(population: &'a mut Vec<Individual<T>>, rate: f64, rng: &mut impl Rng) -> i32
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    let mut count = 0;

    for g in population.iter_mut() {
        if rng.gen_bool(rate) {
            g.genotype.mutate(rng);
            count += 1;
        }
    }

    count
}

/// Standard population implementation
impl<T> StandardPopulation<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    /// Evaluate the fitness of the entire population
    fn evaluate_fitness(&mut self) {
        for i in self.population.iter_mut() {
            i.fitness = i.genotype.fitness();
        }

        self.population.sort_by(|a, b| b.partial_cmp(a).unwrap());

        self.stats = EvolutionStats {
            generation: self.stats.generation + 1,
            max_generations: self.options.max_generations,
            fitness: self.population[0].fitness,
            elapsed: self.started.elapsed().unwrap().as_secs_f32(),
            ..Default::default()
        };
    }

    /// Select parents for crossover and mutation
    fn select_parents(&mut self) -> Vec<Individual<T>> {
        // TODO: Optimize, move chosen selector to struct member
        match self.options.parent_selection {
            _ => {
                println!("Selecting parents using {}", self.options.parent_selection);

                let mut new_population: Vec<Individual<T>> = vec![];

                while new_population.len() < self.options.max_generations as usize {
                    let a =
                        roulette_wheel_select(&new_population, self.stats.fitness, &mut self.rng);
                    let b =
                        roulette_wheel_select(&new_population, self.stats.fitness, &mut self.rng);

                    let individual_a = &self.population[a];
                    let individual_b = &self.population[b];

                    if self.rng.gen_bool(self.options.crossover_rate) {
                        new_population.push(individual_a.crossover(individual_b, &mut self.rng));
                        self.stats.crossovers += 1
                    } else {
                        // TODO: Impl copy or find a way to move through remove()
                        new_population.push(individual_a.crossover(individual_a, &mut self.rng));
                    }
                }

                new_population
            }
        }
    }

    /// Select survivors of this generation
    fn select_survivors(&self) -> Vec<&T> {
        match self.options.survivor_selection {
            _ => {
                println!("Selecting survivors ...");
                return vec![];
            }
        }
    }

    /// Advance to the next generation
    fn next(&mut self) {
        self.evaluate_fitness();

        self.population = self.select_parents();
        self.stats.mutations = mutate(
            &mut self.population,
            self.options.mutation_rate,
            &mut self.rng,
        );

        println!("{}", self.stats);

        if self.options.debug {
            println!("Best candidate:");
            println!("{}", &self.population[0].genotype);
        }
    }
}

/// Implementation of the Population trait for the simple sandbox population
impl<T> Population for StandardPopulation<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    /// Evolve this population based on the given command line arguments
    fn evolve(&mut self) {
        println!(
            "Attempting to evolve Standard population to target fitness {} in maximum {} generations",
            self.options.target_fitness, self.options.max_generations
        );

        for _ in 0..self.options.max_generations {
            // Advance to the next generation
            self.next();

            if self.stats.fitness >= self.options.target_fitness {
                break;
            }
        }

        println!(
            "Completed after {:.4}s",
            self.started.elapsed().unwrap().as_secs_f32(),
        );
    }

    /// Create a new standard population
    fn new(options: Options) -> Self {
        println!(
            "Creating StandardPopulation with {} individuals",
            options.population
        );

        let mut rng = thread_rng();
        let mut population: Vec<Individual<T>> = vec![];

        for _ in 0..options.population {
            population.push(Individual {
                generation: 0,
                fitness: 0.0,
                genotype: T::new(&mut rng),
            });
        }

        let stats = EvolutionStats {
            max_generations: options.max_generations,
            ..Default::default()
        };

        StandardPopulation {
            options,
            rng,
            stats,
            history: vec![],
            population,
            started: SystemTime::now(),
        }
    }
}
