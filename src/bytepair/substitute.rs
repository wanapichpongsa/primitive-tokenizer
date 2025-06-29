use std::vec::Vec;
use super::Token;

pub fn substitute_pairs<T: Token>(
  tokens: &[T],
  most_freq_pair: [T; 2],
  mint: u32,
) -> Vec<u32> {
  let mut new_tokens: Vec<u32> = Vec::new();

  // Number of substitutions performed to check no original tokens were lost
  // (each substitution replaces two original tokens with the newly minted "mint" token).
  let mut matches: usize = 0;

  // Iterate manually with an index so that we can look ahead safely and handle
  // odd-length input slices. The variable `i` stands for "index".
  let mut i: usize = 0;
  while i < tokens.len() {
    // Check that there is a following token to form a pair before attempting
    // the comparison.
    if i + 1 < tokens.len()
      && tokens[i] == most_freq_pair[0]
      && tokens[i + 1] == most_freq_pair[1]
    {
      // Found the most frequent pair – substitute it with the `mint` token.
      new_tokens.push(mint);
      matches += 1;
      // Skip the next element because it is part of the matched pair.
      i += 2;
      continue;
    }

    // No match (or no look-ahead possible) – copy the current token verbatim.
    new_tokens.push(tokens[i].into_u32());
    i += 1;
  }
  
  let original_length = tokens.len();
  let normalized_units = new_tokens.len() + matches;
  assert!(original_length == normalized_units);

  new_tokens
}