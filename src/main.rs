use std::fs;
use std::error::Error;

mod bytepair;
use crate::bytepair::{
    split_english_grammar,
    cleaned_encode_u8,
    most_frequent_codepoint, 
    substitute_bytevec_pairs,
    decompress_bytepair,
    decode_u8
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
    // vocab_size determines num times we compress
    let vocab_size: usize = 5000;
    let num_compress: usize = vocab_size - 256;

    // to track tokenizing history (so we can reverse via decompression)
    let mut first_bytepair: HashMap<u32, [u8; 2]> = HashMap::with_capacity(1);
    let mut most_freq_pairs: HashMap<u32, [u32; 2]> = HashMap::with_capacity(num_compress as usize);

    fn bytepair_compress(
        num_compress: usize,
        cleaned_tokens: &Vec<Vec<u8>>,
        first_bytepair: &mut HashMap<u32, [u8; 2]>,
        most_freq_pairs: &mut HashMap<u32, [u32; 2]>
    ) -> Vec<Vec<u32>> {
        let mut next_mint: u32 = 256;
        // first iteration
        let original_codepoints: Vec<[u8; 2]> = most_frequent_codepoint(cleaned_tokens);
        if original_codepoints.len() == 0 {
            return vec![];
        }
        let original_most_freq = original_codepoints[0];
        first_bytepair.insert(next_mint, original_most_freq);
        let mut bytepaired_tokens: Vec<Vec<u32>> = substitute_bytevec_pairs(cleaned_tokens, original_most_freq, next_mint);
        next_mint += 1;

        for _ in 0..num_compress {
            let codepoints: Vec<[u32; 2]> = most_frequent_codepoint(&bytepaired_tokens);
            if codepoints.len() == 0 {
                break;
            }
            let most_freq = codepoints[0];
            most_freq_pairs.insert(next_mint, most_freq);
            bytepaired_tokens = substitute_bytevec_pairs(&bytepaired_tokens, most_freq, next_mint);
            next_mint += 1;
        }
        bytepaired_tokens
    }

    let result_tokens: Vec<u32> = bytepair_compress(num_compress, &cleaned_tokens, &mut first_bytepair, &mut most_freq_pairs)
        .iter().flat_map(|byte_vec| byte_vec.clone()).collect();
    println!("Compression ratio {}", (text.len() as f64/result_tokens.len() as f64));

    let decompressed_bytepair: Vec<u32> = decompress_bytepair(result_tokens, &first_bytepair, &most_freq_pairs);
    let as_u8: Vec<u8> = decompressed_bytepair.into_iter().map(|v| v as u8).collect();
    let _to_text = decode_u8(as_u8);
    println!("{}", _to_text);

    Ok(())
}
