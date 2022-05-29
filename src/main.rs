use std::process;

use clap::Parser;
use rand::prelude::SliceRandom;

fn main() {
    let opts = Opts::parse();

    if opts.number == 0 {
        eprintln!("`number` must be at least 1");
        process::exit(2);
    }
    if opts.number > opts.choices.len() {
        eprintln!("`number` cannot exceed the number of choices");
        process::exit(2);
    }

    let mut rng = rand::thread_rng();

    for result in opts.choices.choose_multiple(&mut rng, opts.number) {
        println!("{result}");
    }
}

/// Make a random selection from a list of choices.
#[derive(Parser, Debug)]
#[clap(version)]
struct Opts {
    /// The possible choices to pick from.
    #[clap(required = true)]
    choices: Vec<String>,
    /// How many choices to pick.
    #[clap(short, long, default_value = "1")]
    number: usize,
}
