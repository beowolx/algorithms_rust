// Problem statement
// Imagine you have a group of n people standing in a circle. They are numbered sequentially from 1 to n.
// Starting with the first person, you count off around the circle. Every
// k-th person is eliminated, and the process continues around the circle, skipping the eliminated people, until only one person remains.
// Here's a more concrete example:
// You have `n` people standing in a circle.
// You pick a number k, and starting from the first person, you count off k people.
// The person who is at the k-th position is eliminated.
// You continue counting from the next person, again counting
// k people, and eliminate the
// k-th person from the remaining group.
// This process continues until only one person remains, and that person is the "winner."

fn brute_force_josephus_algorithm(mut n: Vec<u32>, k: u32) -> u32 {
  let mut current_position = 0;
  while n.len() > 1 {
    // Calculate the index of the person to be removed
    current_position = (current_position + k as usize - 1) % n.len();

    // Remove the person at the current_position
    n.remove(current_position);
  }
  n[0]
}

fn josephus_mathematical(n: u32, k: u32) -> u32 {
  if n == 1 {
    return 1;
  }
  (josephus_mathematical(n - 1, k) + k - 1) % n + 1
}

fn josephus_mathematical_vec(
  circle: &mut Vec<u32>,
  start: usize,
  k: usize,
) -> u32 {
  if circle.len() == 1 {
    return circle[0];
  }

  let remove_index = (start + k - 1) % circle.len();

  circle.remove(remove_index);

  josephus_mathematical_vec(circle, remove_index, k)
}

mod tests {
  use super::*;
  #[test]
  fn test_josephus_brute_force() {
    assert_eq!(
      brute_force_josephus_algorithm(vec![1, 2, 3, 4, 5, 6, 7], 1),
      7
    );
    assert_eq!(brute_force_josephus_algorithm(vec![1, 2, 3, 4, 5, 6], 2), 5);
    assert_eq!(
      brute_force_josephus_algorithm(vec![1, 2, 3, 4, 5, 6, 7], 3),
      4
    );
  }

  #[test]
  fn test_josephus_recursive() {
    assert_eq!(josephus_mathematical(7, 1), 7);
    assert_eq!(josephus_mathematical(6, 2), 5);
    assert_eq!(josephus_mathematical(7, 3), 4);
    assert_eq!(josephus_mathematical(5, 3), 4);
  }

  #[test]
  fn test_josephus_recursive_vec() {
    let mut circle: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];
    let mut circle2: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(josephus_mathematical_vec(&mut circle, 0, 1), 7);
    assert_eq!(josephus_mathematical_vec(&mut circle2, 0, 2), 5);
    //todo: fix this test
    // assert_eq!(josephus_mathematical_vec(&mut circle, 0, 3), 4);
  }
}
