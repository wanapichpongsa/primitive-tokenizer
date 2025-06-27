use std::collections::HashSet;

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

fn main() {
    let text = "Rust is fun to learn".to_string();
    println!("{}", unique_chars_sorted_ascii(&text));
}
