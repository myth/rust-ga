use rust_ga::ea::population::Population;
use rust_ga::problems;
use rust_ga::Options;
use structopt::StructOpt;

fn main() {
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let args = Options::from_args();

    println!("{} v{}", name, version);

    let mut pop = problems::create_eightqueens(args);
    pop.evolve();
}
