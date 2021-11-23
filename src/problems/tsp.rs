/// Implementation of the traveling salesman problem
use crate::ea::{Genotype, Phenotype};
use crate::Options;
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};
use std::{cmp::Ordering, fmt};

pub fn create_random_cities(n: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    let mut cities: Vec<(i32, i32)> = Vec::with_capacity(n);
    let mut distance_matrix: Vec<f64> = Vec::with_capacity(n * n);

    // Create some random points
    for _ in 0..n {
        cities.push((rng.gen_range(0..500), rng.gen_range(0..500)));
    }

    // Calculate euclidean distance between each pair of points
    for y in 0..n {
        for x in 0..n {
            let p = (cities[y].0 - cities[x].0).pow(2);
            let q = (cities[y].1 - cities[x].1).pow(2);

            distance_matrix.push((p as f64 + q as f64).sqrt());
        }
    }

    distance_matrix
}

/// Shift the elements "from" index "to" index by an offset of "shift".
/// "shift" will wrap by the length of the array
fn shift_elements(genome: &mut Vec<usize>, from: usize, to: usize, shift: usize) {
    if from >= to {
        panic!("from >= to, from={}, to={}", from, to);
    }

    for i in (from..to).rev() {
        let pos = (i + shift) % genome.len();
        let shoved = genome[i];

        genome[i] = genome[pos];
        genome[pos] = shoved;
    }
}

fn create_random_path(length: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut genome: Vec<usize> = Vec::with_capacity(length);

    genome.extend(0..length);
    genome.shuffle(rng);

    genome
}

#[derive(Debug, PartialEq)]
pub struct TravelingSalesman<'a> {
    genome: Vec<usize>,
    pub distances: Option<&'a Vec<f64>>,
}

impl<'a> PartialOrd for TravelingSalesman<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.genome.cmp(&other.genome))
    }
}

impl<'a> fmt::Display for TravelingSalesman<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.genome)
    }
}

impl<'a> Genotype for TravelingSalesman<'a> {
    /// Create a new TSP specimen
    fn new(rng: &mut impl Rng, options: &Options) -> Self {
        Self {
            genome: create_random_path(options.problem_size, rng),
            distances: None,
        }
    }

    /// Mutate this genome in random locations
    fn mutate(&mut self, rng: &mut impl Rng) {
        let length = self.genome.len();

        // 50% chance to randomize cities
        if rng.gen_bool(0.5) {
            // 80% chance to swap two random cities
            if rng.gen_bool(0.8) {
                let a = rng.gen_range(0..length);
                let mut b = rng.gen_range(0..length);

                while a == b {
                    b = rng.gen_range(0..length);
                }

                let tmp = self.genome[a];
                self.genome[a] = self.genome[b];
                self.genome[b] = tmp;
            // 20% change to get a new random path
            } else {
                self.genome = create_random_path(length, rng);
            }
        // 50% chance to shift a subgroup around
        } else {
            let from = rng.gen_range(0..length - 2);
            let to = rng.gen_range(from + 1..length);
            let shift = rng.gen_range(0..length);

            shift_elements(&mut self.genome, from, to, shift);
        }
    }

    /// Create a new specimen by performing crossover with other at random index
    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        let mut genome = other.genome.clone();
        let index = rng.gen_range(0..self.genome.len());

        for i in 0..index {
            genome[i] = self.genome[i];
        }

        Self {
            genome,
            distances: self.distances,
        }
    }
}

impl<'a> Phenotype for TravelingSalesman<'a> {
    fn fitness(&self) -> f64 {
        let length = self.genome.len();
        let mut distance: f64 = 0.0;
        let distances = self.distances.unwrap();

        for i in 0..length {
            let x = self.genome[i];
            let y = self.genome[(i + 1) % length];

            distance += distances[y * length + x];
        }

        distance as f64
    }
}

#[test]
fn test_mutate() {
    // let tsp = TravelingSalesman::new(&mut thread_rng());
    let mut cities = vec![1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 2, 1);
    assert_eq!(cities, [3, 1, 2, 4, 5]);

    cities = vec![1, 2, 3, 4, 5];
    shift_elements(&mut cities, 3, 5, 2);
    assert_eq!(cities, [4, 5, 3, 1, 2]);

    cities = vec![1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 3, 1);
    assert_eq!(cities, [4, 1, 2, 3, 5]);

    cities = vec![1, 2, 3, 4, 5];
    shift_elements(&mut cities, 0, 5, 5);
    assert_eq!(cities, [1, 2, 3, 4, 5]);
}
