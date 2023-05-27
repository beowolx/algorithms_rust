use std::intrinsics::prefetch_read_data;

/// A struct that represents an Eytzinger array.
///
/// It provides methods to create an Eytzinger array from a sorted array
/// and perform a binary search on the Eytzinger array.
pub struct EytzingerArray<T: Copy + Ord> {
  array: Vec<T>,
}

impl<T: Copy + Ord> EytzingerArray<T> {
  /// Creates a new `EytzingerArray` from a sorted array.
  ///
  /// It rearranges the elements of the sorted array to follow the Eytzinger layout.
  ///
  /// # Examples
  ///
  /// ```
  /// use algorithms_rust::binary_search_eytzinger::EytzingerArray;
  /// let sorted_array = vec![1, 2, 3, 4, 5, 6, 7];
  /// let eytzinger_array = EytzingerArray::new(&sorted_array);
  /// ```
  pub fn new(sorted_array: &[T]) -> Self {
    let len = sorted_array.len();
    let mut array = vec![sorted_array[0]; len];
    for (i, element) in array.iter_mut().enumerate().take(len) {
      let pos = get_permutation_element(len - 1, i);
      *element = sorted_array[pos];
    }
    EytzingerArray { array }
  }

  /// Performs a binary search on the Eytzinger array in a branchless manner.
  ///
  /// This function uses a branchless binary search algorithm, which aims to minimize
  /// the number of conditional branches and thereby improve the prediction efficiency
  /// of modern superscalar processors. The implementation uses the characteristic of
  /// Eytzinger layout where child nodes of an element at index `i` are at indexes
  /// `2*i + 1` (for left child) and `2*i + 2` (for right child).
  ///
  /// Memory prefetching is used to improve performance by fetching potential future
  /// data before it is actually needed. The function prefetches the right child of
  /// the current node at each step of the search, which is likely to be accessed in
  /// the next iteration if the key is greater than the current element.
  ///
  /// # Arguments
  ///
  /// * `key` - A key value to search for in the array.
  ///
  /// # Returns
  ///
  /// It returns the index of the searched key if it is present in the array,
  /// or `None` if the key is not found.
  ///
  /// # Safety
  ///
  /// This function uses raw pointers and the `prefetch_read_data` intrinsic, which
  /// are considered unsafe operations in Rust.   
  ///
  /// # Examples
  ///
  /// ```
  /// use algorithms_rust::binary_search_eytzinger::EytzingerArray;
  /// let sorted_array = vec![1, 2, 3, 4, 5, 6, 7];
  /// let eytzinger_array = EytzingerArray::new(&sorted_array);
  ///
  /// assert_eq!(eytzinger_array.binary_search(&5), Some(5));
  /// assert_eq!(eytzinger_array.binary_search(&8), None);
  /// ```
  pub fn binary_search(&self, key: &T) -> Option<usize> {
    let mut idx = 0;

    while idx < self.array.len() {
      unsafe {
        let prefetch = self.array.as_ptr().wrapping_add(2 * idx + 2);
        prefetch_read_data::<T>(prefetch, 0);
      }
      match self.array[idx].cmp(key) {
        std::cmp::Ordering::Less => idx = 2 * idx + 2, // right child
        std::cmp::Ordering::Greater => idx = 2 * idx + 1, // left child
        std::cmp::Ordering::Equal => return Some(idx),
      }
    }

    None
  }
}

/// Returns the permutation element by node for the Eytzinger layout.
///
/// It is a helper function used in the creation of the Eytzinger array.
fn get_permutation_element_by_node(n: usize, ipk: usize, li: usize) -> usize {
  let zk = li * 2 + 1;
  let last_power_of_two = (n + 2).next_power_of_two() / 2;
  let y = (last_power_of_two >> (ipk - 1)) * zk;
  let kp = y >> 1;
  let x = kp + last_power_of_two;
  let x = x.saturating_sub(n + 1);
  y - x - 1
}

/// Transforms an index to a node for the Eytzinger layout.
///
/// It is a helper function used in the creation of the Eytzinger array.
fn index_to_node(i: usize) -> (usize, usize) {
  let ipk = (i + 2).next_power_of_two().trailing_zeros() as usize;
  let li = i + 1 - (1 << (ipk - 1));
  (ipk, li)
}

/// Returns the permutation element for the Eytzinger layout.
///
/// It is a helper function used in the creation of the Eytzinger array.
fn get_permutation_element(n: usize, i: usize) -> usize {
  let (ipk, li) = index_to_node(i);
  get_permutation_element_by_node(n, ipk, li)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eytzinger_layout() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let eytzinger_array = EytzingerArray::new(&arr);
    assert_eq!(eytzinger_array.array, vec![4, 2, 6, 1, 3, 5, 7]);
  }

  #[test]
  fn test_binary_search_eytzinger() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let eytzinger_array = EytzingerArray::new(&arr);
    assert_eq!(eytzinger_array.binary_search(&6), Some(2));
    assert_eq!(eytzinger_array.binary_search(&3), Some(4));
    assert_eq!(eytzinger_array.binary_search(&1), Some(3));
    assert_eq!(eytzinger_array.binary_search(&7), Some(6));
    assert_eq!(eytzinger_array.binary_search(&8), None);
  }
}
