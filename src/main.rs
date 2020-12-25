use rust_ga::ea::Population;
use rust_ga::problems;
use rust_ga::Options;
use structopt::StructOpt;

fn main() {
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let args = Options::from_args();

    println!("{} v{}", name, version);

    match args.problem {
        problems::Problem::TravelingSalesman => {
            let (mut pop, distances) = problems::create_tsp(args);

            for p in pop.iter_mut() {
                p.genotype.distances = Some(&distances);
            }

            pop.evolve();
        }
        _ => {
            problems::create_nqueens(args).evolve();
        }
    }
}
