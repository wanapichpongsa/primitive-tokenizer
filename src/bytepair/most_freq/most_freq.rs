use std::collections::{HashMap, HashSet};
use super::desc_sort::desc_merge_sort;

// Step 1: Initialize frequency hashmap
fn init_freq_hashmap(tokens: Vec<u8>) -> HashMap<u8, u8> {
  let mut freq_hashmap: HashMap<u8, u8> = HashMap::new();
  for token in tokens {
    if !freq_hashmap.contains_key(&token) {
      freq_hashmap.insert(token, 0);
    }
    freq_hashmap.insert(token, freq_hashmap[&token] + 1);
  }
  return freq_hashmap;
}

fn init_freq_hashset(freq_hashmap: &HashMap<u8, u8>) -> HashSet<u8> {
  let mut freq_hashset: HashSet<u8> = HashSet::new();
  for (_key, value) in freq_hashmap.iter() {
    freq_hashset.insert(*value);
  }
  return freq_hashset;
}

fn init_descending_freq(freq_hashset: &HashSet<u8>) -> Vec<u8> {
  let mut descending_freq: Vec<u8> = Vec::new();
  // O(n) populate vector with unique frequencies
  for freq in freq_hashset.iter() {
    descending_freq.push(*freq);
  }
  descending_freq = desc_merge_sort(descending_freq);
  return descending_freq;
}

// Step 2: Find most frequent codepoints
pub fn most_frequent_codepoint(tokens: Vec<u8>) -> Vec<u8> {
  let freq_hashmap: HashMap<u8, u8> = init_freq_hashmap(tokens);
  let freq_hashset: HashSet<u8> = init_freq_hashset(&freq_hashmap);
  let descending_freq: Vec<u8> = init_descending_freq(&freq_hashset);

  let mut keys_by_freq: Vec<u8> = Vec::with_capacity(freq_hashmap.len());
  for unique_freq in descending_freq {
    let mut batch: Vec<u8> = Vec::new();
    for (key, value) in freq_hashmap.iter() {
      if *value == unique_freq {
        batch.push(*key);
      }
    }
    keys_by_freq.extend(batch);
  }
 
  return keys_by_freq;
}