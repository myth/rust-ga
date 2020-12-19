/// Cli module
use structopt::StructOpt;

/// Command line interface
#[derive(Debug, StructOpt)]
#[structopt(name = "rust-ga", about = "Simple genetic algorithm")]
pub struct Cli {
    /// Maximum number of epochs
    #[structopt(short = "e", long = "epochs", default_value = "20")]
    pub max_epochs: i32,

    /// Target fitness
    #[structopt(short = "t", long = "target", default_value = "0.9")]
    pub target_fitness: f32,

    /// Mutation rate
    #[structopt(short = "m", long = "mutation", default_value = "0.1")]
    pub mutation_rate: f64,

    /// Crossover rate
    #[structopt(short = "c", long = "crossover", default_value = "0.1")]
    pub crossover_rate: f64,

    /// Population size
    #[structopt(short = "p", long = "population", default_value = "30")]
    pub population: i32,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,
}
