pub fn desc_merge_sort(arr: Vec<u32>) -> Vec<u32> {
  if arr.len() <= 1 {
    return arr;
  }
  let pivot = arr[arr.len() - 1];
  let mut left = Vec::new();
  let mut right = Vec::new();
  for (i, item) in arr.iter().enumerate() {
    if i == arr.len() - 1 {
      continue; // Skip the pivot element
    }
    // Descending order
    if *item > pivot {
      left.push(*item);
    } else {
      right.push(*item);
    }
  }
  
  let mut result = desc_merge_sort(left);
  result.push(pivot);
  result.extend(desc_merge_sort(right));
  return result;
}