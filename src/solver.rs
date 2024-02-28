use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn get_library_path() -> PathBuf {
    let path = std::env::current_dir().expect("Failed to get current directory.");
    let file_path = path.join("src/resources/dict.txt");
    file_path
}

pub fn get_word_list() -> HashSet<String> {
    let file_path = get_library_path();
    let file = std::fs::File::open(&file_path).expect("Failed to open file.");
    let reader = BufReader::new(file);
    let word_search: HashSet<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line").to_lowercase())
        .collect();

    word_search
}

#[derive(Debug)]
pub struct SolveResult {
    pub solution: String,
    pub words_used: Vec<String>,
}

fn get_valid_words<'a>(
    parts: &'a Vec<&'a str>,
    size: usize,
    ws: &'a HashSet<String>,
) -> HashSet<&'a str> {
    let min_length: usize = parts.iter().map(|p| p.len()).max().unwrap_or(0) + size;
    let max_length: usize = parts.iter().map(|p| p.len()).min().unwrap_or(0) + size;

    ws.iter()
        .filter(|w| {
            let word_len = w.len();
            word_len == min_length || word_len == max_length
        })
        .map(|w| w.as_str())
        .collect()
}

pub fn solve(puzzle: &str, size: usize, ws: &HashSet<String>) -> SolveResult {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut prefix: HashSet<&str> = HashSet::new();
    let mut suffix: HashSet<&str> = HashSet::new();
    let parts: Vec<&str> = puzzle.split('/').collect();

    let valid_words = get_valid_words(&parts, size, &ws);
    let low_parts: Vec<String> = parts.iter().map(|part| part.to_lowercase()).collect();

    for word in valid_words {
        for part in &low_parts {
            let word_len = word.len();
            let part_len = part.len();
            if part_len + size == word_len {
                if word.starts_with(&*part) {
                    *counts.entry(&word[word_len - size..]).or_insert(0) += 1;
                    prefix.insert(word);
                } else if word.ends_with(&*part) {
                    *counts.entry(&word[..size]).or_insert(0) += 1;
                    suffix.insert(word);
                }
            }
        }
    }

    let solution = counts
        .into_iter()
        .filter_map(|(k, v)| if v >= 3 { Some(k) } else { None })
        .collect::<Vec<&str>>()
        .join(" ")
        .to_uppercase();

    let words_solution: Vec<String> = low_parts
        .iter()
        .map(|part| {
            let at_start = format!("{}{}", part, solution).to_lowercase();
            let at_end = format!("{}{}", solution, part).to_lowercase();

            if prefix.contains(&at_start as &str) {
                return at_start.to_uppercase();
            }

            at_end.to_uppercase()
        })
        .collect();

    let words_used = if words_solution.len() < 3 {
        Vec::new()
    } else {
        words_solution
    };

    SolveResult {
        solution,
        words_used,
    }
}
