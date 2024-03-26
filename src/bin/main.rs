#![warn(clippy::all, clippy::pedantic)]

extern crate triword;

use clap::{arg, Command};
use triword::{
    generator::generate,
    solver::{get_word_list, solve},
};

fn main() {
    let cmd = Command::new("triword")
        .bin_name("triword")
        .about("Triword Puzzle Generator and Solver")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("solve")
                .about("Solve puzzle")
                .arg(arg!(<PUZZLE> "Puzzle to solve"))
                .arg(arg!(<SIZE> "Puzzle key size").value_parser(clap::value_parser!(usize)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate random puzzle unless key is provided")
                .arg(arg!(<KEY> "Generate puzzle from key").required(false)),
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
            let _ = generate(key);
        }
        _ => unreachable!(),
    }
}
