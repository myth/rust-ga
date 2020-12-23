/// Implementation of the N-queens problem
use crate::ea::{Genotype, Phenotype};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::fmt;

// The size of the board in the N-queens problem (NxN)
const N: usize = 20;

// Simple N! implementation
fn factorial(n: u64) -> u64 {
    (1..=n).fold(1, |acc, v| acc * v)
}

// Max non-attacking queens in N-queens problem is N choose K=2 = !N / K!(N-K)!
fn max_clashes() -> u32 {
    (factorial(N as u64) / (2 * factorial(N as u64 - 2))) as u32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NQueens {
    genome: [usize; N],
    max_clashes: u32,
}

impl NQueens {
    fn to_grid(&self) -> [[usize; N]; N] {
        let mut grid = [[0; N]; N];

        for x in 0..self.genome.len() {
            grid[self.genome[x]][x] = 1;
        }

        grid
    }
}

impl fmt::Display for NQueens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = String::new();
        let grid = self.to_grid();

        for row in 0..grid.len() - 1 {
            rows.push_str(format!("{:?}\n", grid[row]).as_str());
        }
        rows.push_str(format!("{:?}", grid[grid.len() - 1]).as_str());

        write!(f, "{}", rows)
    }
}

impl Genotype for NQueens {
    /// Create a new NQueens specimen
    fn new(rng: &mut impl Rng) -> Self {
        let mut genome = [0; N];
        let range = Uniform::from(0..N);

        for i in 0..N {
            genome[i] = range.sample(rng);
        }

        NQueens {
            genome,
            max_clashes: max_clashes(),
        }
    }

    /// Mutate this genome in random locations
    fn mutate(&mut self, rng: &mut impl Rng) {
        let i = rng.gen_range(0, N);
        let v = rng.gen_range(0, N);
        self.genome[i] = v;
    }

    /// Create a new specimen by performing crossover with other at random index
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        let mut genome = [0; N];
        let index = rng.gen_range(0, N);

        for i in 0..index {
            genome[i] = self.genome[i];
        }
        for i in index..N {
            genome[i] = other.genome[i];
        }

        Self {
            genome,
            max_clashes: max_clashes(),
        }
    }
}

impl Phenotype for NQueens {
    fn fitness(&self) -> f64 {
        let mut clashes: u32 = 0;

        for x in 0..self.genome.len() {
            let y: usize = self.genome[x];

            for i in 0..x {
                let other = self.genome[i];

                if other == y || (other as i32 - y as i32).abs() == (x as i32 - i as i32).abs() {
                    clashes += 1;
                }
            }
        }

        // Max number of non-attacking queen pairs is N choose 2 for an NxN board
        // For N=8 this is 28
        let fitness = self.max_clashes as f64 / (self.max_clashes + clashes) as f64;

        fitness
    }
}
