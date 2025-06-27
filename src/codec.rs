use std::collections::{HashSet, HashMap};

/*
    Making a tokenizer...
*/
// COder / DECoder
pub struct Codec {
    stoi: HashMap<char, i32>,      // "string-to-integer" map
    itos: HashMap<i32, char>,      // "integer-to-string" map
}

impl Codec {
    // Helper
    fn unique_chars_sorted_ascii(text: &str) -> Vec<char> {
        // String (utf-8) -> Vec<char> (utf-32 scalar value)
        // utf-8 is better because it is backwards compatible with ASCII
        let mut chars: Vec<char> = text.chars()
            .filter(|&c| c != ' ').
            collect::<HashSet<_>>()// remove duplicates
            .into_iter().collect();
        // by unicode order
        // for locale dictionary order, use unicode-collation
        chars.sort_unstable();
        return chars;
    }
    /*** Constructor ***/
    pub fn new(text: &str) -> Self {
        let mut stoi = HashMap::new();
        let mut itos = HashMap::new();

        let alphabet = Self::unique_chars_sorted_ascii(text);

        for (i, c) in alphabet.iter().enumerate() {
            stoi.insert(*c, i as i32);
            itos.insert(i as i32, *c);
        }
        // handle space for now
        stoi.insert(' ', -1);
        itos.insert(-1, ' ');
        Self { stoi, itos }
    }

    // encoder
    pub fn encode(&self, text: &str) -> Vec<i32> {
        text.chars().map(|c| self.stoi[&c]).collect()
    }

    // decoder
    pub fn decode(&self, vec: &[i32]) -> String {
        vec.iter().map(|i| self.itos[i]).collect()
    }
}