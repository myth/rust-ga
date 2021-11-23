use super::individual::{Genotype, Individual, Phenotype};
use crate::Options;
use rand::{thread_rng, Rng};
use std::time::SystemTime;
use std::{cmp::Ordering, fmt};
use std::{fmt::Display, slice::IterMut};
use structopt::clap::arg_enum;

/// Interface for working with various populations
pub trait Population {
    /// Create a new population based on the given options
    fn new(options: Options) -> Self;
    /// Remove this?
    fn evolve(&mut self);
}

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available population models
    #[derive(Copy, Clone, Debug)]
    pub enum PopulationModel {
        SteadyState,
        Generational,
    }
}

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available parent selection strategies
    #[derive(Copy, Clone, Debug)]
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
    #[derive(Copy, Clone, Debug)]
    pub enum SurvivorSelection {
        AgeBased,
        FitnessBased,
    }
}

/// Basic statistics container
#[derive(Debug, Default)]
struct EvolutionStats {
    /// Which generation these stats represent
    generation: i32,
    /// Maximum number of generations in the evolution
    max_generations: u32,
    /// Best fitness achieved this generation
    fitness: f64,
    /// The total elapsed time at this generation
    elapsed: f32,
    /// The total number of mutations this generation
    mutations: i32,
    /// The total number of mutations over the course of evolution
    total_mutations: i32,
    /// The total number of crossovers this generation
    crossovers: i32,
    /// The total number of crossovers over the course of evolution
    total_crossovers: i32,
}

/// String representation of the statistics container
impl Display for EvolutionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.max_generations == 0 {
            write!(
                f,
                "[{}] ({:.3}s) F: {:.3} C: {} M: {}",
                self.generation, self.elapsed, self.fitness, self.crossovers, self.mutations,
            )
        } else {
            write!(
                f,
                "[{}/{}] ({:.3}s) F: {:.3} C: {} M: {}",
                self.generation,
                self.max_generations,
                self.elapsed,
                self.fitness,
                self.crossovers,
                self.mutations,
            )
        }
    }
}

/// Select a parent using roulette wheel selection
fn roulette_wheel_select<'a, T>(
    population: &'a Vec<Individual<T>>,
    s: f64,
    minimize: bool,
    rng: &mut impl Rng,
) -> usize
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    let mut p = 0.0;
    let t = rng.gen_range(0.0..s);

    for (i, individual) in population.iter().enumerate() {
        if minimize {
            p += 1.0 / individual.fitness;
        } else {
            p += individual.fitness;
        }

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

/// Evaluate a collection of individuals
fn evaluate<'a, T>(population: &'a mut Vec<Individual<T>>)
where
    T: Genotype + Phenotype + PartialOrd,
{
    for i in population.into_iter() {
        i.evaluate();
    }
}

/// Sort a collection of individuals
fn sort<'a, T>(population: &'a mut Vec<Individual<T>>, reverse: bool)
where
    T: Genotype + Phenotype + PartialOrd,
{
    if reverse {
        population.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    } else {
        population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    }
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
    population: Vec<Individual<T>>,
    started: SystemTime,
    last_print: f32,
}

/// Standard population implementation
impl<T> StandardPopulation<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    /// Select parents for crossover and mutation
    fn select_parents(&mut self, total_fitness: f64) -> Vec<Individual<T>> {
        let mut new_population: Vec<Individual<T>> = Vec::with_capacity(self.options.problem_size);

        // TODO: Optimize, move chosen selector to struct member
        match self.options.parent_selection {
            ParentSelection::RouletteWheel => {
                while new_population.len() < self.options.population as usize {
                    let a = roulette_wheel_select(
                        &self.population,
                        total_fitness,
                        self.options.minimize,
                        &mut self.rng,
                    );
                    let individual_a = &self.population[a];
                    let new: Individual<T>;

                    if self.rng.gen_bool(self.options.crossover_rate) {
                        self.stats.crossovers += 1;
                        let b = roulette_wheel_select(
                            &self.population,
                            total_fitness,
                            self.options.minimize,
                            &mut self.rng,
                        );
                        let individual_b = &self.population[b];
                        new = individual_a.crossover(
                            individual_b,
                            self.stats.generation,
                            &mut self.rng,
                        );
                    } else {
                        // TODO: Clean this up. Need to move or copy
                        new = individual_a.crossover(
                            individual_a,
                            self.stats.generation,
                            &mut self.rng,
                        );
                    }

                    new_population.push(new);
                }
            }
            _ => {
                // TODO: Implement support for more methods
            }
        }

        // Mutate offspring
        self.stats.mutations = mutate(
            &mut new_population,
            self.options.mutation_rate,
            &mut self.rng,
        );

        // If we have elitism, replace one individual with the best from the existing population
        if !self.options.no_elitism {
            new_population.pop();
            new_population.push(self.population.remove(0));
        }

        new_population
    }

    /// Select survivors of this generation
    fn select_survivors(&mut self, new_generation: Vec<Individual<T>>) {
        // Population model determines if we are replacing entire generation or
        // performing some sort of generational mixing
        match self.options.population_model {
            PopulationModel::SteadyState => {
                match self.options.survivor_selection {
                    SurvivorSelection::FitnessBased => {
                        // TODO: Implement support for one of the fitness based selectors like
                        // roulette wheel or tournament etc
                    }
                    SurvivorSelection::AgeBased => {
                        // TODO: Implement support
                    }
                }
            }
            PopulationModel::Generational => {
                self.population = new_generation;
            }
        }
    }

    /// Advance to the next generation
    fn next(&mut self) {
        self.stats.generation += 1;
        self.stats.mutations = 0;
        self.stats.crossovers = 0;

        let mut total_fitness = 0.0;
        for i in &self.population {
            if self.options.minimize {
                total_fitness += 1.0 / i.fitness;
            } else {
                total_fitness += i.fitness
            }
        }

        let mut new_generation = self.select_parents(total_fitness);

        evaluate(&mut new_generation);
        sort(&mut new_generation, !self.options.minimize);

        self.select_survivors(new_generation);
        let best = &self.population[0];

        self.stats.fitness = best.fitness;
        self.stats.total_mutations += self.stats.mutations;
        self.stats.total_crossovers += self.stats.crossovers;
        self.stats.elapsed = self.started.elapsed().unwrap().as_secs_f32();

        // Output status every second
        if self.stats.elapsed - self.last_print > 1.0 {
            println!("{} Best: {}", self.stats, best);
            if self.options.debug {
                println!(
                    "{:?}",
                    self.population
                        .iter()
                        .map(|i| i.fitness)
                        .collect::<Vec<f64>>()
                );
                println!("{}", best.genotype);
            }
            self.last_print = self.stats.elapsed;
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<Individual<T>> {
        self.population.iter_mut()
    }
}

/// Implementation of the Population trait for the simple sandbox population
impl<T> Population for StandardPopulation<T>
where
    T: Genotype + Phenotype + Display + PartialOrd,
{
    /// Evolve this population based on the given command line arguments
    fn evolve(&mut self) {
        self.started = SystemTime::now();

        // Calculate fitness and sort the new population
        evaluate(&mut self.population);
        sort(&mut self.population, !self.options.minimize);

        if self.options.debug {
            println!("{:?}", self.options);
        }

        if self.options.max_generations == 0 {
            println!(
                "Attempting to evolve {} ({}) until target fitness {:.3} is met",
                self.options.problem, self.options.problem_size, self.options.target_fitness
            );
        } else {
            println!(
                "Attempting to evolve {} ({}) to target fitness {:.3} in maximum {} generations",
                self.options.problem,
                self.options.problem_size,
                self.options.target_fitness,
                self.options.max_generations
            );
        }

        // Max generations of 0 means run until target fitness is met
        if self.options.max_generations == 0 {
            loop {
                self.next();

                if (self.options.minimize && self.stats.fitness <= self.options.target_fitness)
                    || (!self.options.minimize && self.stats.fitness >= self.options.target_fitness)
                {
                    break;
                }
            }
        } else {
            for _ in 0..self.options.max_generations {
                self.next();

                if (self.options.minimize && self.stats.fitness <= self.options.target_fitness)
                    || (!self.options.minimize && self.stats.fitness >= self.options.target_fitness)
                {
                    break;
                }
            }
        }

        let best = &self.population[0];
        println!(
            "Reached {:.3} fitness in {} generations after {:.3}s with {} mutations and {} crossovers",
            self.stats.fitness,
            self.stats.generation,
            self.started.elapsed().unwrap().as_secs_f32(),
            self.stats.total_mutations,
            self.stats.total_crossovers
        );
        println!("{}", &best.genotype);
    }

    /// Create a new standard population
    fn new(options: Options) -> Self {
        let mut rng = thread_rng();
        let mut population: Vec<Individual<T>> = Vec::with_capacity(options.population);

        for _ in 0..options.population {
            population.push(Individual {
                generation: 0,
                fitness: 0.0,
                genotype: T::new(&mut rng, &options),
            });
        }

        StandardPopulation {
            population,
            stats: EvolutionStats {
                max_generations: options.max_generations,
                ..Default::default()
            },
            rng,
            options,
            started: SystemTime::now(),
            last_print: 0.0,
        }
    }
}
