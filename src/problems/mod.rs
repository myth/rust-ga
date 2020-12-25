mod nqueens;
mod tsp;

use crate::ea::population::Population;
use crate::ea::population::StandardPopulation;
use crate::Options;
use nqueens::NQueens;
use structopt::clap::arg_enum;
use tsp::TravelingSalesman;

// These are wrapped in arg_enum since we are constructing these directly from StructOpt
arg_enum! {
    /// Available parent selection strategies
    #[derive(Copy, Clone, Debug)]
    pub enum Problem {
        NQueens,
        TravelingSalesman,
    }
}

pub fn create_nqueens(options: Options) -> StandardPopulation<NQueens> {
    StandardPopulation::<NQueens>::new(options)
}

pub fn create_tsp(
    mut options: Options,
) -> (StandardPopulation<TravelingSalesman<'static>>, Vec<f64>) {
    // Don't have crossover yet
    options.crossover_rate = 0.0;
    // Have to minimize fitness
    options.minimize = true;

    let distances = tsp::create_random_cities(options.problem_size);

    (
        StandardPopulation::<TravelingSalesman>::new(options),
        distances,
    )
}
