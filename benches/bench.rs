use algorithms_rust::binary_search_eytzinger::EytzingerArray;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn binary_search_benchmark(c: &mut Criterion) {
  // Define your array sizes.
  let array_sizes = [10_000, 100_000, 1_000_000];

  // Generate a limited set of random keys.
  let mut rng = rand::thread_rng();
  let keys: Vec<_> = (0..5).map(|_| rng.gen_range(0..1_000_000)).collect();

  for &size in &array_sizes {
    let sorted_array = (0..size).collect::<Vec<_>>();
    let eytzinger_array = EytzingerArray::new(&sorted_array);

    for &key in &keys {
      let function_name = format!("binary_search_{}_{}", size, key);
      c.bench_function(&function_name, |b| {
        b.iter(|| eytzinger_array.binary_search(black_box(&key)))
      });

      let function_name = format!("traditional_binary_search_{}_{}", size, key);
      c.bench_function(&function_name, |b| {
        b.iter(|| sorted_array.binary_search(black_box(&key)))
      });
    }
  }
}

criterion_group!(benches, binary_search_benchmark);
criterion_main!(benches);
