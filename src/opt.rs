/// Options
use crate::ea::PopulationModel;
use crate::ea::{ParentSelection, SurvivorSelection};
use structopt::StructOpt;

/// Command line interface
#[derive(Debug, StructOpt)]
#[structopt(name = "rust-ga", about = "Simple genetic algorithm")]
pub struct Options {
    /// Population size
    #[structopt(short = "p", long = "population", default_value = "50")]
    pub population: usize,

    /// Maximum number of generations (set to 0 to run until reaching target fitness)
    #[structopt(short = "g", long = "generations", default_value = "0")]
    pub max_generations: u32,

    /// Target fitness
    #[structopt(short = "t", long = "target", default_value = "1.0")]
    pub target_fitness: f64,

    /// Mutation rate
    #[structopt(short = "m", long = "mutation", default_value = "0.1")]
    pub mutation_rate: f64,

    /// Crossover rate
    #[structopt(short = "c", long = "crossover", default_value = "0.5")]
    pub crossover_rate: f64,

    /// Whether or not to turn off elitism
    #[structopt(long = "no-elitism")]
    pub no_elitism: bool,

    /// Whether or not to minimize the fitness function
    #[structopt(long = "minimize")]
    pub minimize: bool,

    /// Parent selection strategy
    #[structopt(
        long = "parent-selection",
        possible_values = &ParentSelection::variants(),
        case_insensitive = true,
        default_value = "RouletteWheel"
    )]
    pub parent_selection: ParentSelection,

    /// Survivor selection stragegy
    #[structopt(
        long = "survivor-selection",
        possible_values = &SurvivorSelection::variants(),
        case_insensitive = true,
        default_value = "FitnessBased"
    )]
    pub survivor_selection: SurvivorSelection,

    /// Population model
    #[structopt(
        long = "population-model",
        possible_values = &PopulationModel::variants(),
        case_insensitive = true,
        default_value = "Generational"
    )]
    pub population_model: PopulationModel,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,
}
