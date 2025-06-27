use std::collections::HashMap;

/*
I was today years old when I learned that you can conduct next_token_prediction in intervals of n words. 
I'm not sure of the benefits of using intervals greater than 1, but here is a deterministic function to 
signify my learning experience.
*/

fn populate_tokens() -> HashMap<u32, String> {
    let mut tokens = HashMap::new();
    tokens.insert(0, "Rust".to_string());
    tokens.insert(1, "is".to_string());
    tokens.insert(2, "fun".to_string());
    tokens.insert(3, "to".to_string());
    tokens.insert(4, "learn".to_string());
    return tokens;
}

fn next_token_prediction(
    tokens: HashMap<u32, String>, 
    word: String,
    pred_interval: u32
) -> String {
    // Before semantic search algorithm that is provided by vector embeddings.
    fn brute_search(tokens: &HashMap<u32, String>, word: &String) -> i32 {
        for (key, value) in tokens.iter() {
            if value == word {
                return *key as i32;
            }
        }
        return -1;
    }

    let index = brute_search(&tokens, &word);
    if index == -1 {
        return "<EOS>".to_string();
    }
    let next_index = index + (pred_interval as i32);
    return tokens[&(next_index as u32)].clone();
}

fn main() {
    let tokens = populate_tokens();
    println!("{:?}", next_token_prediction(tokens, "Rust".to_string(), 2));
}
