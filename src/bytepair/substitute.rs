use std::vec::Vec;
use super::Token;

pub fn substitute_pairs<T: Token>(
  tokens: &[T],
  most_frequent: Vec<[T; 2]>,
  mint: u32,
) -> Vec<u32> {
  let most_freq_pair: [T; 2] = most_frequent[0];

  let mut new_tokens: Vec<u32> = Vec::new();
  let mut matched: bool = false;
  // sliding window; last lone token ignored.
  for (first, second) in tokens.iter().zip(tokens.iter().skip(1)) {
    if matched {
      matched = false;
      continue;
    }
    let pair: [T; 2] = [*first, *second];
    if pair == most_freq_pair {
      new_tokens.push(mint);
      matched = true;
      continue;
    }
    new_tokens.push((*first).into_u32());
  }

  new_tokens
}