use std::vec::Vec;
use super::codec::{decode, encode};

pub fn substitute_pairs(text: &str, most_frequent: Vec<u8>) -> Vec<u8> {
  let mint: [char; 4] = ['W', 'X', 'Y', 'Z'];

  // Get the 2 most frequent chars
  let most_freq_tokens = vec![most_frequent[0], most_frequent[1]];
  let most_freq_chars_vec = decode(most_freq_tokens);
  let pair_chars: [char; 2] = [most_freq_chars_vec[0], most_freq_chars_vec[1]];
  let combinations: [String; 4] = [
    pair_chars[0].to_string().repeat(2),
    pair_chars[0].to_string() + &pair_chars[1].to_string(),
    pair_chars[1].to_string() + &pair_chars[0].to_string(),
    pair_chars[1].to_string().repeat(2),
  ];

  // value we will encode and return
  let mut new_text = String::new();

  // use Vec instead of &str for slicing because &str uses byte indexing && utf-8 is 2 bytes (would slice 1 char)
  let chars: Vec<char> = text.chars().collect();
  let length = chars.len();
  let mut matched: bool = false;
  
  for i in 0..length {
    // if previously minted pair, skip over the 2nd char
    if matched {
      matched = false;
      continue;
    }

    // If this is the last character, just add it
    if i == length - 1 {
      new_text.push(chars[i]);
      break;
    }

    let [first, second] = &chars[i..i+2]
      else { panic!("Not exactly 2 chars") };

    let pair = first.to_string() + &second.to_string();

    for i in 0..combinations.len() {
      if pair == combinations[i] {
        new_text.push(mint[i]);
        matched = true;
        break;
      }
    }
    if !matched {
      new_text.push(chars[i]);
    }
  }
  println!("{}", new_text);
  let new_tokens = encode(&new_text);
  return new_tokens;
}