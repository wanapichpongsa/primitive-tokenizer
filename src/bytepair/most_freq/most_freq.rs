use std::collections::{HashMap, HashSet};
use super::desc_sort::desc_merge_sort;
use crate::bytepair::Token;

fn init_freq_hashmap<T: Token>(tokens: &[T]) -> HashMap<[T; 2], u32> {
  let mut freq_hashmap: HashMap<[T; 2], u32> = HashMap::new();
  // runs [0, 1], [1, 2], [2, 3] ... if leftover lone token skip.
  for (first, second) in tokens.iter().zip(tokens.iter().skip(1)) {
    let pair: [T; 2] = [*first, *second];
    *freq_hashmap.entry(pair).or_insert(0) += 1;
  }
  freq_hashmap
}

fn init_freq_hashset<T: Token>(freq_hashmap: &HashMap<[T; 2], u32>) -> HashSet<u32> {
  freq_hashmap.values().copied().collect()
}

fn init_descending_freq(freq_hashset: &HashSet<u32>) -> Vec<u32> {
  let mut descending_freq: Vec<u32> = freq_hashset.iter().copied().collect();
  descending_freq = desc_merge_sort(descending_freq);
  descending_freq
}

pub fn most_frequent_codepoint<T: Token>(tokens: &[T]) -> Vec<[T; 2]> {
  if tokens.len() < 2 {
    println!("Message too short for bytepair encoding");
    return vec![];
  }

  let freq_hashmap: HashMap<[T; 2], u32> = init_freq_hashmap(tokens);
  let freq_hashset: HashSet<u32> = init_freq_hashset(&freq_hashmap);
  let descending_freq: Vec<u32> = init_descending_freq(&freq_hashset);

  let mut keys_by_freq: Vec<[T; 2]> = Vec::with_capacity(freq_hashmap.len());
  // O(m * n)
  // for every descending freq, push key with matching value
  for unique_freq in descending_freq {
    for (key, value) in freq_hashmap.iter() {
      if *value == unique_freq {
        keys_by_freq.push(*key);
      }
    }
  }
  keys_by_freq
}