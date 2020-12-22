pub mod individual;
pub mod population;

pub use individual::{Genotype, Individual, Phenotype};
pub use population::{
    ParentSelection, Population, PopulationModel, StandardPopulation, SurvivorSelection,
};
