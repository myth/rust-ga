/// Implementation of the traveling salesman problem
use crate::ea::{Genotype, Phenotype};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

// The coordinates of each city
const CITIES: [[i32; 2]; 32] = [
    [35, 51],
    [113, 213],
    [82, 280],
    [322, 340],
    [256, 352],
    [160, 24],
    [322, 145],
    [12, 349],
    [282, 20],
    [241, 8],
    [398, 153],
    [182, 305],
    [153, 257],
    [275, 190],
    [242, 75],
    [19, 229],
    [303, 352],
    [39, 309],
    [383, 79],
    [226, 343],
    [22, 356],
    [11, 57],
    [336, 673],
    [99, 482],
    [241, 81],
    [351, 12],
    [321, 123],
    [111, 358],
    [150, 77],
    [250, 250],
    [200, 350],
    [299, 399],
];
const N: usize = CITIES.len();

/// Shift the elements "from" index "to" index by an offset of "shift".
/// "shift" will wrap by the length of the array
fn shift_elements(genome: &mut [usize], from: usize, to: usize, shift: usize) {
    for i in (from..to).rev() {
        let pos = (i + shift) % genome.len();
        let shoved = genome[i];

        genome[i] = genome[pos];
        genome[pos] = shoved;
    }
}

fn create_random_path(rng: &mut impl Rng) -> [usize; N] {
    let mut genome = [0; N];
    let mut order: Vec<usize> = (0..N).into_iter().collect();

    order.shuffle(rng);

    for (i, v) in order.into_iter().enumerate() {
        genome[i] = v;
    }

    genome
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TravelingSalesman {
    genome: [usize; N],
}

impl fmt::Display for TravelingSalesman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.genome)
    }
}

impl Genotype for TravelingSalesman {
    /// Create a new NQueens specimen
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            genome: create_random_path(rng),
        }
    }

    /// Mutate this genome in random locations
    fn mutate(&mut self, rng: &mut impl Rng) {
        // 50% chance to randomize cities
        if rng.gen_bool(0.5) {
            // 80% chance to swap two random cities
            if rng.gen_bool(0.8) {
                let a = rng.gen_range(0, N);
                let mut b = rng.gen_range(0, N);

                while a == b {
                    b = rng.gen_range(0, N);
                }

                let tmp = self.genome[a];
                self.genome[a] = self.genome[b];
                self.genome[b] = tmp;
            // 20% change to get a new random path
            } else {
                self.genome = create_random_path(rng);
            }
        // 50% chance to shift a subgroup around
        } else {
            let from = rng.gen_range(0, N - 2);
            let to = rng.gen_range(from + 1, N);
            let shift = rng.gen_range(0, N);

            shift_elements(&mut self.genome, from, to, shift);
        }
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

        Self { genome }
    }
}

impl Phenotype for TravelingSalesman {
    // TODO: Figure out a nice way to precompute distances in a lookup table
    fn fitness(&self) -> f64 {
        let mut distance: f64 = 0.0;

        for x in 0..N {
            let a = self.genome[x];
            let b = self.genome[(x + 1) % N];

            let p = (CITIES[b][0] - CITIES[a][0]).pow(2);
            let q = (CITIES[b][1] - CITIES[a][1]).pow(2);

            distance += (p as f64 + q as f64).sqrt();
        }

        distance as f64
    }
}

#[test]
fn test_mutate() {
    // let tsp = TravelingSalesman::new(&mut thread_rng());
    let mut cities: [usize; 5] = [1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 2, 1);
    assert_eq!(cities, [3, 1, 2, 4, 5]);

    cities = [1, 2, 3, 4, 5];
    shift_elements(&mut cities, 3, 5, 2);
    assert_eq!(cities, [4, 5, 3, 1, 2]);

    cities = [1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 3, 1);
    assert_eq!(cities, [4, 1, 2, 3, 5]);

    cities = [1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 5, 5);
    assert_eq!(cities, [1, 2, 3, 4, 5]);
}
