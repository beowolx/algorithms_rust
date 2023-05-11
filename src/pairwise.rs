pub fn pairwise_product(numbers: &Vec<i64>) -> i64 {
  let mut max_num1 = 0;
  let mut max_num2 = 0;

  if numbers.len() < 2 {
    max_num1 = numbers[0];
  }

  for &n in numbers {
    if n > max_num1 {
      max_num2 = max_num1;
      max_num1 = n;
    } else if n > max_num2 {
      max_num2 = n;
    }
  }

  max_num1 * max_num2
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  #[test]
  fn test_pairwise_product() {
    assert_eq!(pairwise_product(&vec![1, 2, 3, 4, 5]), 20);
    assert_eq!(pairwise_product(&vec![100_000, 90_000]), 9000000000);
  }

  #[test]
  fn test_pairwise_product_stress() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
      let n = rng.gen_range(2..1000);
      let mut numbers = Vec::with_capacity(n);
      for _ in 0..n {
        numbers.push(rng.gen_range(0..100_000));
      }
      let result = pairwise_product(&numbers);
      let mut target = std::i64::MIN;
      for i in 0..n {
        for j in i + 1..n {
          if numbers[i] * numbers[j] > target {
            target = numbers[i] * numbers[j];
          }
        }
      }
      assert_eq!(result, target);
    }
  }
}

/* Main function for testing */
// fn main() {
//     // Buffer for the first line (length)
//     let mut buffer = String::new();
//     std::io::stdin()
//         .read_line(&mut buffer)
//         .expect("Failed to read line");
//
//     // Parse the length and allocate capacity for numbers
//     let length: usize = buffer.trim().parse().expect("Please enter a valid length");
//
//     // Buffer for the second line (numbers)
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//
//     // Initialize numbers with the known capacity
//     let mut numbers: Vec<i64> = Vec::with_capacity(length);
//
//     // Parse the numbers and push them into numbers
//     for s in input.split_whitespace() {
//         let num: i64 = s.parse().expect("Please enter a valid i64 number");
//         numbers.push(num);
//     }
//
//     // Print the resulting Vec<i64>
//     let result = pairwise_product(&numbers);
//     println!("{}", result);
// }
