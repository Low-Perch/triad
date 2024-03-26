use rand::{thread_rng, Rng};
use std::collections::HashSet;
use serde_json::{Result, Value};

pub fn generate(key: Option<String>) -> Result<()> {
    let data = read_json_data()?;
    let random_key = get_key(key.as_deref(), &data);

    match random_key.is_empty() {
        true => {
            println!("{:?} is not a valid puzzle key. Provide another key or use default random option.", key.unwrap_or_default());
        }
        false => {
            let selected_words = select_three_words(&data, &random_key);
            print_selected_data(&random_key, &selected_words);
        }
    }

    Ok(())
}

fn read_json_data() -> Result<Value> {
    let data_str = include_str!("../resources/dict.json");
    serde_json::from_str(data_str)
}

fn get_key(key: Option<&str>, data: &Value) -> String {
    match key {
        Some(key) => {
            if data[key].is_null() {
                String::from("")
            } else {
                key.to_string()
            }
        }
        None => get_random_key(data),
    }
}

fn get_random_key(data: &Value) -> String {
    let keys = data.as_object().unwrap().keys().collect::<Vec<_>>();
    let random_index = thread_rng().gen_range(0..keys.len());
    keys[random_index].to_string()
}

fn select_three_words(data: &Value, key: &str) -> Vec<String> {
    let prefix = data[key]["prefix"].as_array().map_or(Vec::new(), |v| v.to_vec());
    let suffix = data[key]["suffix"].as_array().map_or(Vec::new(), |v| v.to_vec());

    let mut selected_words = HashSet::new();

    // First, try to select one word from each array
    select_word(&prefix, &mut selected_words);
    select_word(&suffix, &mut selected_words);

    // Fill in the remaining slots
    while selected_words.len() < 3 {
        let prefix_or_suffix = if thread_rng().gen_bool(0.5) { &prefix } else { &suffix };
        if !prefix_or_suffix.is_empty() {
            select_word(prefix_or_suffix, &mut selected_words);
        }
    }

    selected_words.into_iter().collect()
}

fn select_word(words: &[Value], selected_words: &mut HashSet<String>) {
    if !words.is_empty() {
        let mut rng = thread_rng();
        while selected_words.len() < 3 {
            let random_index = rng.gen_range(0..words.len());
            let word = words[random_index].as_str().unwrap().to_string();
            if selected_words.insert(word.clone()) {
                break;
            }
        }
    }
}

fn print_selected_data(key: &str, words: &[String]) {
    println!("Random key: {}", key);
    println!("Random 3 words: {:?}", words);
}
