use clap::Parser;
use rand::prelude::SliceRandom;
use std::io::{self, BufRead};

fn main() {
    let opts = Opts::parse();

    // if opts.number > opts.choices.len() {
    //     Opts::command()
    //         .error(
    //             ErrorKind::ValueValidation,
    //             format!(
    //                 "Invalid value '{}' for '{}': cannot exceed number of choices ({})",
    //                 format!("{}", opts.number).yellow(),
    //                 "--number <NUMBER>".bold(),
    //                 opts.choices.len(),
    //             ),
    //         )
    //         .exit();
    // }

    let mut rng = rand::thread_rng();

    #[cfg(debug_assertions)]
    dbg!(&opts);

    if opts.lines {
        let lines = get_lines_from_files(opts.choices);
        for result in lines.choose_multiple(&mut rng, opts.number) {
            println!("{result}");
        }
    } else {
        for result in opts.choices.choose_multiple(&mut rng, opts.number) {
            println!("{result}");
        }
    }
}

/// Make a random selection from a list of choices.
#[derive(Parser, Debug)]
#[command(version)]
struct Opts {
    /// The possible choices to pick from.
    choices: Vec<String>,
    /// How many choices to pick.
    #[arg(short, long, default_value = "1", value_parser = validate_number)]
    number: usize,
    /// Whether to treat `choices` as a list of files, and select random lines from those files.
    /// If no files are provided, read from stdin.
    #[arg(short, long)]
    lines: bool,
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

fn get_lines_from_files(files: Vec<String>) -> Vec<String> {
    let mut lines = Vec::new();

    for file in files {
        let file_name = file.clone();
        let file = std::fs::File::open(&file).expect(&format!("Could not open file `{}`", &file));
        let reader = std::io::BufReader::new(&file);

        for line in reader.lines() {
            lines.push(line.expect(&format!("Could not read line from file `{}`", &file_name)));
        }
    }

    // If no files were provided, read from stdin
    if lines.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();

        for line in reader.lines() {
            lines.push(line.unwrap());
        }
    }

    lines
}
