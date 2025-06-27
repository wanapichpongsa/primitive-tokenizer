use std::collections::{HashSet, HashMap};

/*
    Making a tokenizer...
*/
// COder / DECoder
struct Codec {
    stoi: HashMap<char, i32>,      // "string-to-integer" map
    itos: HashMap<i32, char>,      // "integer-to-string" map
}

impl Codec {
    // Helper
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
    /*** Constructor ***/
    fn new(text: &str) -> Self {
        let mut stoi = HashMap::new();
        let mut itos = HashMap::new();

        let alphabet = Self::unique_chars_sorted_ascii(text);

        for (i, c) in alphabet.chars().enumerate() {
            stoi.insert(c, i as i32);
            itos.insert(i as i32, c);
        }
        // handle space for now
        stoi.insert(' ', -1);
        itos.insert(-1, ' ');
        Self { stoi, itos }
    }

    // encoder
    fn encode(&self, text: &str) -> Vec<i32> {
        text.chars().map(|c| self.stoi[&c]).collect()
    }

    // decoder
    fn decode(&self, vec: &[i32]) -> String {
        vec.iter().map(|i| self.itos[i]).collect()
    }
}

fn main() {
    let text: &str = "Rust is fun to learn";

    let codec = Codec::new(text);

    let encoded = codec.encode(text);
    let decoded = codec.decode(&encoded);

    println!("encoded = {:?}, decoded = {}", encoded, decoded);
}
