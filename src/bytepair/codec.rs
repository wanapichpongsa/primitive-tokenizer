use std::vec::Vec;

// Step 1: Convert text to tokens
pub fn encode(text: &str) -> Vec<u8> {
  let mut tokens = Vec::new();
  for char in text.chars() {
    tokens.push(char as u8);
  }
  return tokens;
}

pub fn decode(tokens: Vec<u8>) -> Vec<char> {
  let mut chars = Vec::new();
  for token in tokens.iter() {
    chars.push(*token as char);
  }
  return chars;
}