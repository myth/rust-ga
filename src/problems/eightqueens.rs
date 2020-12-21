use super::super::ga::{Genotype, Phenotype};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EightQueens {
    genome: [i32; 8],
}

impl EightQueens {
    fn to_grid(&self) -> [[i32; 8]; 8] {
        let mut grid = [[0; 8]; 8];

        for x in 0usize..8usize {
            grid[self.genome[x] as usize][x] = 1;
        }

        grid
    }
}

impl fmt::Display for EightQueens {
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

impl Genotype for EightQueens {
    /// Create a new EightQueens specimen
    fn new(rng: &mut impl Rng) -> Self {
        let mut genome = [0; 8];
        let range = Uniform::from(0..8);

        for i in 0..8 {
            genome[i] = range.sample(rng);
        }

        EightQueens { genome }
    }

    /// Mutate this genomy by swapping two random positions
    fn mutate(&mut self, rng: &mut impl Rng) {
        // Random number of mutations between 1 and 5
        let num_mutations = rng.gen_range(1, 9);

        for _ in 0..num_mutations {
            let i = rng.gen_range(0, 8);
            // 50% change of swapping two slots or generating a new random entry
            if rng.gen_bool(0.5) {
                let j = rng.gen_range(0, 8);
                self.genome[i] = self.genome[j];
            } else {
                let v = rng.gen_range(0, 8);
                self.genome[i] = v;
            }
        }
    }

    /// Create a new specimen by performing crossover with other at random index
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        let mut genome = [0; 8];
        let index = rng.gen_range(0, 8);

        for i in 0..index {
            genome[i] = self.genome[i];
        }
        for i in index..8 {
            genome[i] = other.genome[i];
        }

        Self { genome }
    }
}

impl Phenotype for EightQueens {
    fn fitness(&self) -> f64 {
        let mut clashes = 0;

        for x in 0..self.genome.len() {
            let y: usize = self.genome[x] as usize;

            for i in 0..x {
                let other = self.genome[i] as usize;

                if other == y || (other as i32 - y as i32).abs() == (x as i32 - i as i32).abs() {
                    clashes += 1;
                }
            }
        }

        // Max number of non-attacking queen pairs is 28
        let fitness = 28.0 / (28 + clashes) as f64;

        fitness
    }
}
