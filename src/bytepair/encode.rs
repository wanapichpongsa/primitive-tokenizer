use std::vec::Vec;

pub fn encode(text: &str) -> Vec<u32> {
  let mut tokens = Vec::new();
  for char in text.chars() {
    tokens.push(char as u32);
  }
  return tokens;
}
