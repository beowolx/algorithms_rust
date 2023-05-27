pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
  if arr.is_empty() {
    return None;
  }

  let mut size = arr.len();
  let mut base = 0;

  while size > 0 {
    size /= 2;
    let mid = base + size;
    base = if arr[mid] > *target { base } else { mid };
  }

  if arr[base] == *target {
    Some(base)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_binary_search_empty() {
    let arr: [i32; 0] = [];
    assert_eq!(binary_search(&arr, &1), None);
  }

  #[test]
  fn test_binary_search_single_element_found() {
    let arr = [1];
    assert_eq!(binary_search(&arr, &1), Some(0));
  }

  #[test]
  fn test_binary_search_single_element_not_found() {
    let arr = [1];
    assert_eq!(binary_search(&arr, &2), None);
  }

  #[test]
  fn test_binary_search_multiple_elements_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(binary_search(&arr, &3), Some(2));
  }

  #[test]
  fn test_binary_search_multiple_elements_not_found() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(binary_search(&arr, &6), None);
  }
}
