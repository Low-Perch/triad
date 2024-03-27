#![warn(clippy::all, clippy::pedantic)]

extern crate triad;

use clap::{arg, Command};
use triad::{
    generator::{generate, GenerateResult},
    solver::{get_word_list, solve},
};

fn main() {
    let cmd = Command::new("triad")
        .bin_name("triad")
        .version("0.1.0")
        .about("Triad Puzzle Generator and Solver")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("solve")
                .short_flag('s')
                .long_flag("solve")
                .about("Solve Triad PUZZLE")
                .arg(arg!(<PUZZLE> "PUZZLE to solve. Case insensitive. Must use / separator. Ex. TAR/RICE/IL"))
                .arg(arg!(<SIZE> "SIZE of missing letters in PUZZLE. Must be 3 or 4.").value_parser(clap::value_parser!(usize)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("generate")
                .short_flag('g')
                .long_flag("generate")
                .about("Generate Triad PUZZLE. Provide KEY to base PUZZLE upon, otherwise random.")
                .arg(arg!(<KEY> "Generate PUZZLE based on KEY. KEY must 3 or 4 letters.").required(false)),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("solve", submatches)) => {
            let puzzle = submatches.get_one::<String>("PUZZLE").expect("required");
            let size = *submatches.get_one::<usize>("SIZE").expect("required");

            let word_search = get_word_list();
            let result = solve(puzzle, size, &word_search);

            if result.solution.is_empty() {
                println!("No solution found for puzzle {puzzle} with size {size}. Please verify the puzzle and size.");
            } else {
                println!(
                    "Solution for puzzle {puzzle} with size {size} is {}.",
                    result.solution
                );
                println!("Words in puzzle: {:?}.", &result.words_used[..=2]);
            }
        }
        Some(("generate", submatches)) => {
            let key = submatches
                .get_one::<String>("KEY")
                .map(std::clone::Clone::clone);
            if let Ok(GenerateResult { words_used, key }) = generate(key) {
                println!("Key: {key}");
                println!("Puzzle: {words_used:?}");
            } else {
                println!("An error occurred generating puzzle.");
            }
        }
        _ => unreachable!(),
    }
}
