use std::vec::Vec;
use std::collections::HashMap;
use crate::bytepair::desc_merge_sort;

// not called yet so keep local
fn encode_u8(text: &str) -> Vec<u8> {
  text.bytes().collect()
}

/* 
  * To work with our Regex algorithm...
  * We want to apply the rules of special tokens! (Token for each Regex pattern matched) e.g., 've, 'll, 're
  * AND we can only count bytepair frequencies within each String item
  * e.g., ["\'ve", " you", " been", "?"], "e " won't ever be a bytepair again.

  This function will synergise with most_frequent_codepoint to decide pairs, and end its lifetime there.
*/
pub fn cleaned_encode_u8(cleaned_strings: &Vec<String>) -> Vec<Vec<u8>> {
  cleaned_strings.iter().map(|str| 
    encode_u8(str)
  ).collect()
}

// lossy decodes as: U+FFFD REPLACEMENT CHARACTER � if byte not String
pub fn decode_u8(tokens: Vec<u8>) -> String {
  String::from_utf8_lossy(&tokens).to_string()
}

pub fn decode_u32(tokens: Vec<u32>) -> String {
  tokens.iter().map(|t| char::from_u32(*t).unwrap_or('\u{FFFD}')).collect()
}

/*
CORRECTION with prev comment committed: Information loss bug was because no even/odd check.
*/
pub fn decompress_bytepair(
    compression: Vec<u32>,
    first_byte_pair: &HashMap<u32, [u8; 2]>,
    most_freq_pairs: &HashMap<u32, [u32; 2]>
) -> Vec<u32> {
  let mut all_byte_pairs = most_freq_pairs.clone();
  all_byte_pairs.insert(256, first_byte_pair[&256].map(|v| v as u32));

  // .keys() -> Keys<_, K: &T, _>, .cloned() -> Keys<_, K: T, _>, .collect() -> Vec<T>
  // ^ .keys() actually returns Keys<_, K: &T, V> (idk why) but we don't need it so I marked as _
  let mut desc_keys: Vec<u32> = all_byte_pairs.keys().cloned().collect();
  desc_keys = desc_merge_sort(desc_keys);

  let mut decompressed_tokens: Vec<u32> = Vec::new();
  // stage 1: load compression values
  let last_key = desc_keys[0];
  desc_keys.remove(0); // remove first val
  for token in compression.iter() {
    if *token == last_key {
      decompressed_tokens.push(all_byte_pairs[&last_key][0]);
      decompressed_tokens.push(all_byte_pairs[&last_key][1]);
      continue;
    }
    decompressed_tokens.push(*token);
  }

  // stage 2: iteratively decompress tokens
  for key in desc_keys.iter() {
    let mut iteration_vec: Vec<u32> = Vec::new();
    for token in decompressed_tokens.iter() {
      if *token == *key {
        iteration_vec.push(all_byte_pairs[key][0]);
        iteration_vec.push(all_byte_pairs[key][1]);
        continue;
      }
      iteration_vec.push(*token);
    }
    decompressed_tokens = iteration_vec;
  }

  return decompressed_tokens;
}