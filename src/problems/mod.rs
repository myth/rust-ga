mod nqueens;

use crate::ea::population::Population;
use crate::ea::population::StandardPopulation;
use crate::Options;
use nqueens::NQueens;

pub fn create_eightqueens(options: Options) -> StandardPopulation<NQueens> {
    StandardPopulation::<NQueens>::new(options)
}
