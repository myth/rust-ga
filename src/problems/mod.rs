use eightqueens::EightQueens;

use crate::{cli::Options, ga::StandardPopulation};

pub mod eightqueens;

use crate::ga::Population;

pub fn create_eightqueens(options: Options) -> StandardPopulation<EightQueens> {
    StandardPopulation::<EightQueens>::new(options)
}
