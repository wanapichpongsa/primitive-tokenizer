use std::fs;
use std::error::Error;

mod bytepair;
use crate::bytepair::{
    split_english_grammar,
    cleaned_encode_u8,
    most_frequent_codepoint, 
    substitute_bytevec_pairs,
    decompress_bytepair,
    decode_u8,
    decode_u32
};
use std::vec::Vec;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // make sure path is relative to Cargo.toml
    let text: String = fs::read_to_string("unicode.txt")?;

    let cleaned_strings: Vec<String> = split_english_grammar(&text);

    /*
        Step 1: Encode into utf-8 (base vocab of 255 combinations, ASCII compatible).
        Will turn non-ASCII into >= 2 bytes.
     */
    let cleaned_tokens: Vec<Vec<u8>> = cleaned_encode_u8(&cleaned_strings);

    /*
        Step 2: Compress 130,000 unicode base vocab -> ~ 32,000 - 64,000 Bytepair Encoding (BPE) base vocab
    */
    // determine num of times we compress
    let vocab_size: usize = 356;
    let num_compress: usize = vocab_size - 256;

    // to track tokenizing history (so we can reverse via decompression)
    let mut first_bytepair: HashMap<u32, [u8; 2]> = HashMap::with_capacity(1);
    let mut most_freq_pairs: HashMap<u32, [u32; 2]> = HashMap::with_capacity(num_compress as usize);
    // Binary Forest
    fn bytepair_compress(
        num_compress: usize,
        cleaned_tokens: &Vec<Vec<u8>>,
        first_bytepair: &mut HashMap<u32, [u8; 2]>,
        most_freq_pairs: &mut HashMap<u32, [u32; 2]>
    ) -> Vec<Vec<u32>> {
        let mut next_mint: u32 = 256;
        // first iteration
        let original_most_freq: Vec<[u8; 2]> = most_frequent_codepoint(cleaned_tokens);
        println!("{}", decode_u8(original_most_freq[0].to_vec()));
        first_bytepair.insert(next_mint, original_most_freq[0]);
        let mut bytepaired_tokens: Vec<Vec<u32>> = substitute_bytevec_pairs(cleaned_tokens, original_most_freq[0], next_mint);
        next_mint += 1;

        for _ in 0..num_compress {
            let most_freq: Vec<[u32; 2]> = most_frequent_codepoint(&bytepaired_tokens);
            println!("{}", decode_u32(most_freq[0].to_vec()));
            most_freq_pairs.insert(next_mint, most_freq[0]);
            bytepaired_tokens = substitute_bytevec_pairs(&bytepaired_tokens, most_freq[0], next_mint);
            next_mint += 1;
        }
        bytepaired_tokens
    }

    // DEBUG: Find the most frequent bytepair @...

    let result_tokens: Vec<u32> = bytepair_compress(num_compress, &cleaned_tokens, &mut first_bytepair, &mut most_freq_pairs)
        .iter().flat_map(|byte_vec| byte_vec.clone()).collect();
    // After cleaning: 1.7 -> 0.3
    println!("Compression ratio {}", (cleaned_tokens.len() as f64/result_tokens.len() as f64));

    let decompressed_bytepair: Vec<u32> = decompress_bytepair(result_tokens, &first_bytepair, &most_freq_pairs);
    let as_u8: Vec<u8> = decompressed_bytepair.into_iter().map(|v| v as u8).collect();
    let _to_text = decode_u8(as_u8);
    // println!("{}", to_text);

    Ok(())
}
