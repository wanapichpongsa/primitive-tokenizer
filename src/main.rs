use std::collections::{HashSet, HashMap};

/*
    Making a tokenizer...
*/
fn unique_chars_sorted_ascii(text: &str) -> String {
    let mut chars: Vec<char> = text
        .chars()  
        .filter(|&c| c != ' ')
        .collect::<HashSet<_>>()// remove duplicates
        .into_iter()
        .collect();
    chars.sort_unstable(); // ASC-order
    chars.into_iter().collect()
}

fn tokenize(text: &str) -> HashMap<i32, char> {
    let mut tokens: HashMap<i32, char> = HashMap::new();
    let mut index: i32 = 0;
    for c in text.chars() {
        tokens.insert(index, c);
        index += 1;
    }
    return tokens;
}

fn main() {
    let text: String = "Rust is fun to learn".to_string();
    let chars: String = unique_chars_sorted_ascii(&text);
    // Follows Hash bucket order not insertion order.
    let tokens: HashMap<i32, char> = tokenize(&chars);
    println!("{:?}", tokens);
}
