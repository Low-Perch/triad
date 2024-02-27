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

pub fn solve(puzzle: &str, size: usize, ws: &HashSet<String>) -> String {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let low_puzzle = puzzle.to_lowercase();
    let parts: Vec<&str> = low_puzzle.split('/').collect();

    let min_length: usize = parts.iter().map(|p| p.len()).max().unwrap_or(0) + size;
    let max_length: usize = parts.iter().map(|p| p.len()).min().unwrap_or(0) + size;

    let valid_words: HashSet<&str> = ws
        .iter()
        .filter(|w| {
            let word_len = w.len();
            word_len == min_length || word_len == max_length
        })
        .map(|w| w.as_str())
        .collect();

    for word in valid_words {
        for part in &parts {
            let word_len = word.len();
            let part_len = part.len();
            if part_len + size == word_len {
                if word.ends_with(part) {
                    *counts.entry(&word[..size]).or_insert(0) += 1;
                } else if word.starts_with(part) {
                    *counts.entry(&word[word_len - size..]).or_insert(0) += 1;
                }
            }
        }
    }

    counts
        .into_iter()
        .filter_map(|(k, v)| if v >= 3 { Some(k) } else { None })
        .collect::<Vec<&str>>()
        .join(" ")
}
