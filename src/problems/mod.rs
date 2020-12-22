use nqueens::NQueens;

use crate::{cli::Options, ga::StandardPopulation};

pub mod nqueens;

use crate::ga::Population;

pub fn create_eightqueens(options: Options) -> StandardPopulation<NQueens> {
    StandardPopulation::<NQueens>::new(options)
}
