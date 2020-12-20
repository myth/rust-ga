mod cli;
mod ga;
mod problems;

use cli::Options;
use ga::Population;
use structopt::StructOpt;

fn main() {
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let args = Options::from_args();

    println!("{} v{}", name, version);

    if args.debug {
        println!("{:?}", args);
    }

    let mut pop = problems::create_eightqueens(args);
    pop.evolve();
}
