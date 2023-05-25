/// Binary search implementation that uses the Eytzinger layout.
pub fn binary_search(data: &[u32], value: u32) -> usize {
  let mut idx = 1;
  while idx < data.len() {
    let el = data[idx];
    if el == value {
      return idx;
    }
    idx = 2 * idx + usize::from(el < value);
  }
  0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_empty() {
        let data: [u32; 0] = [];
        assert_eq!(binary_search(&data, 1), 0);
    }

    #[test]
    fn test_binary_search_single_item_found() {
        let data: [u32; 1] = [1];
        assert_eq!(binary_search(&data, 1), 0);
    }

    #[test]
    fn test_binary_search_single_item_not_found() {
        let data: [u32; 1] = [1];
        assert_eq!(binary_search(&data, 2), 0);
    }

    #[test]
    fn test_binary_search_multiple_items_found() {
        let data: [u32; 7] = [4, 2, 6, 1, 3, 5, 7];
        assert_eq!(binary_search(&data, 6), 2);
    }

    #[test]
    fn test_binary_search_multiple_items_not_found() {
        let data: [u32; 7] = [4, 2, 6, 1, 3, 5, 7];
        assert_eq!(binary_search(&data, 8), 0);
    }

    #[test]
    fn test_binary_search_random_items_found() {
        let data: [u32; 15] = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(binary_search(&data, 10), 5);
    }

    #[test]
    fn test_binary_search_random_items_not_found() {
        let data: [u32; 15] = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(binary_search(&data, 16), 0);
    }
}


