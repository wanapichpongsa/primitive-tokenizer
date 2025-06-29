use std::fs;
use std::error::Error;

mod bytepair;
use crate::bytepair::{
    encode_u8, 
    most_frequent_codepoint, 
    substitute_pairs,
    decompress_bytepair,
    decode_u8
};
use std::vec::Vec;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // make sure path is relative to Cargo.toml
    let text: String = fs::read_to_string("unicode.txt")?;

    let tokens: Vec<u8> = encode_u8(&text);
    // Note: Not byte length (that stays the same)

    let vocab_size: usize = 356;
    let num_merges: usize = vocab_size - 256;

    // to track tokenizing history (so we can reverse)
    let mut first_bytepair: HashMap<u32, [u8; 2]> = HashMap::with_capacity(1);
    let mut most_freq_pairs: HashMap<u32, [u32; 2]> = HashMap::with_capacity(num_merges as usize);
    // Binary Forest
    fn iterative_bytepair_encoding(
        num_merges: usize, 
        tokens: &[u8],
        first_bytepair: &mut HashMap<u32, [u8; 2]>,
        most_freq_pairs: &mut HashMap<u32, [u32; 2]>
    ) -> Vec<u32> {
        let mut next_mint: u32 = 256;
        // first iteration
        let original_most_freq: Vec<[u8; 2]> = most_frequent_codepoint(tokens);
        first_bytepair.insert(next_mint, original_most_freq[0]);
        let mut bytepaired_tokens: Vec<u32> = substitute_pairs(tokens, original_most_freq[0], next_mint);
        next_mint += 1;

        for _ in 0..num_merges {
            let most_freq: Vec<[u32; 2]> = most_frequent_codepoint(&bytepaired_tokens);
            most_freq_pairs.insert(next_mint, most_freq[0]);
            bytepaired_tokens = substitute_pairs(&bytepaired_tokens, most_freq[0], next_mint);
            next_mint += 1;
        }
        bytepaired_tokens
    }

    // DEBUG: Find the most frequent bytepair @...

    let result_tokens = iterative_bytepair_encoding(num_merges, &tokens, &mut first_bytepair, &mut most_freq_pairs);
    println!("Compression ratio {}", (tokens.len() as f64/result_tokens.len() as f64));

    let decompressed_bytepair: Vec<u32> = decompress_bytepair(result_tokens, &first_bytepair, &most_freq_pairs);
    let as_u8: Vec<u8> = decompressed_bytepair.into_iter().map(|v| v as u8).collect();
    let to_text = decode_u8(as_u8);
    println!("{}", to_text);

    Ok(())
}
