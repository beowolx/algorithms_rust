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

fn josephus_algorithm(n: u32, k: u32) -> u32 {
  if n == 1 {
    return 0;
  }
  (josephus_algorithm(n - 1, k) + k) % n
}

mod tests {
  use super::*;
  #[test]
  fn test_josephus_algorithm() {
    assert_eq!(josephus_algorithm(7, 1), 7);
  }
}
