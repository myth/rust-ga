mod cli;
mod ga;

use cli::Cli;
use ga::Population;
use structopt::StructOpt;

fn main() {
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

    println!("{} v{}", name, version);

    let args = Cli::from_args();

    if args.debug {
        println!("{:?}", args);
    }

    let mut pop = ga::create_eight_queens(args.population);
    pop.evolve(&args);
}
