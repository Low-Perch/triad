#![warn(clippy::all, clippy::pedantic)]

extern crate triword;

use triword::solver::{get_word_list, solve};

fn main() {
    let word_search = get_word_list();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: <puzzle> <size>");
        return;
    }

    let puzzle = &args[1];
    let size: usize = if let Ok(s) = args[2].parse() {
        s
    } else {
        println!("Invalid Size. Size must be >= 3");
        return;
    };

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
