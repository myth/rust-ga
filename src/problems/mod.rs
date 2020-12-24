mod nqueens;
mod tsp;

use crate::ea::population::Population;
use crate::ea::population::StandardPopulation;
use crate::Options;
use nqueens::NQueens;
use tsp::TravelingSalesman;

pub fn create_eightqueens(options: Options) -> StandardPopulation<NQueens> {
    StandardPopulation::<NQueens>::new(options)
}

pub fn create_tsp(options: Options) -> StandardPopulation<TravelingSalesman> {
    StandardPopulation::<TravelingSalesman>::new(options)
}
