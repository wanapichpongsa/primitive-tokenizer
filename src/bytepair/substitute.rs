use std::vec::Vec;

pub fn substitute_pairs(tokens: &Vec<u8>, most_frequent: Vec<[u8; 2]>) -> Vec<u16> {
  let mint: u16 = 256;
  let most_freq_pair: [u8; 2] = most_frequent[0];
  println!("{:?}", most_freq_pair);

  let mut new_tokens: Vec<u16> = Vec::new();
  let mut matched: bool = false;
  // again, sliding window so last lone byte ignored.
  for (first, second) in tokens.iter().zip(tokens.iter().skip(1)) {
    if matched {
      matched = false;
      continue;
    }
    let pair: [u8; 2] = [*first, *second];
    if pair == most_freq_pair {
      new_tokens.push(mint);
      matched = true;
      continue;
    }
    new_tokens.push(*first as u16);
  }

  return new_tokens;
}