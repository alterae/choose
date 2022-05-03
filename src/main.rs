use std::process;

use clap::Parser;
use rand::prelude::SliceRandom;

fn main() {
    let opts = Opts::parse();

    if opts.number == 0 {
        eprintln!("`number` must be at least 1");
        process::exit(2);
    }
    if opts.number > opts.options.len() {
        eprintln!("`number` cannot exceed the number of options");
        process::exit(2);
    }

    let mut rng = rand::thread_rng();

    for result in opts.options.choose_multiple(&mut rng, opts.number) {
        println!("{result}");
    }
}

/// Make a random selection from a list of options.
#[derive(Parser, Debug)]
struct Opts {
    /// The options to choose from.
    #[clap(required = true)]
    options: Vec<String>,
    /// How many options to choose.
    #[clap(short, long, default_value = "1")]
    number: usize,
}
