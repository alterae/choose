use clap::{arg, command, error::ErrorKind, CommandFactory, Parser};
use colored::Colorize;
use rand::prelude::SliceRandom;

fn main() {
    let opts = Opts::parse();

    // FIXME: move this logic into the actual command parsing
    if opts.number > opts.choices.len() {
        Opts::command()
            .error(
                ErrorKind::ValueValidation,
                format!(
                    "Invalid value '{}' for '{}': cannot exceed number of choices ({})",
                    format!("{}", opts.number).yellow(),
                    "--number <NUMBER>".bold(),
                    opts.choices.len(),
                ),
            )
            .exit();
    }

    let mut rng = rand::thread_rng();

    for result in opts.choices.choose_multiple(&mut rng, opts.number) {
        println!("{result}");
    }
}

/// Make a random selection from a list of choices.
#[derive(Parser, Debug)]
#[command(version)]
struct Opts {
    /// The possible choices to pick from.
    #[arg(required = true)]
    choices: Vec<String>,
    /// How many choices to pick.
    #[arg(short, long, default_value = "1", value_parser = validate_number)]
    number: usize,
}

/// Value parser for ensuring that the `number` option is greater than 0.
///
/// FIXME: Should also ensure it doesn't exceed the number of choices provided, but
/// clap doesn't want to allow that :(
fn validate_number(s: &str) -> Result<usize, String> {
    let number: usize = s.parse().map_err(|_| "invalid digit found in string")?;

    if number == 0 {
        Err("must be at least 1".into())
    } else {
        Ok(number)
    }
}
