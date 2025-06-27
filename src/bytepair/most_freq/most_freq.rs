use std::collections::{HashMap, HashSet};
use super::desc_sort::desc_merge_sort;

// Step 1: Initialize frequency hashmap
fn init_freq_hashmap(tokens: Vec<u32>) -> HashMap<u32, u32> {
  let mut freq_hashmap: HashMap<u32, u32> = HashMap::new();
  for token in tokens {
    if !freq_hashmap.contains_key(&token) {
      freq_hashmap.insert(token, 0);
    }
    freq_hashmap.insert(token, freq_hashmap[&token] + 1);
  }
  return freq_hashmap;
}

fn init_freq_hashset(freq_hashmap: &HashMap<u32, u32>) -> HashSet<u32> {
  let mut freq_hashset: HashSet<u32> = HashSet::new();
  for (_key, value) in freq_hashmap.iter() {
    freq_hashset.insert(*value);
  }
  return freq_hashset;
}

fn init_descending_freq(freq_hashset: &HashSet<u32>) -> Vec<u32> {
  let mut descending_freq: Vec<u32> = Vec::new();
  // O(n) populate vector with unique frequencies
  for freq in freq_hashset.iter() {
    descending_freq.push(*freq);
  }
  descending_freq = desc_merge_sort(descending_freq);
  return descending_freq;
}

pub fn most_frequent_codepoint(tokens: Vec<u32>) -> Vec<u32> {
  let freq_hashmap: HashMap<u32, u32> = init_freq_hashmap(tokens);
  let freq_hashset: HashSet<u32> = init_freq_hashset(&freq_hashmap);
  let descending_freq: Vec<u32> = init_descending_freq(&freq_hashset);

  let mut keys_by_freq: Vec<u32> = Vec::with_capacity(freq_hashmap.len());
  for unique_freq in descending_freq {
    let mut batch: Vec<u32> = Vec::new();
    for (key, value) in freq_hashmap.iter() {
      if *value == unique_freq {
        batch.push(*key);
      }
    }
    keys_by_freq.extend(batch);
  }
 
  return keys_by_freq;
}