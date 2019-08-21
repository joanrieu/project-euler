use std::collections::btree_map::*;

fn main() {
  let mut c_cache: BTreeMap<(i128, i128), i128> = BTreeMap::new();
  for n in 1..=100 {
    c_cache.insert((n, 0), 1);
    c_cache.insert((n, n), 1);
    for k in 1..n {
      c_cache.insert(
        (n, k),
        c_cache.get(&(n - 1, k - 1)).unwrap() + c_cache.get(&(n - 1, k)).unwrap(),
      );
    }
  }
  println!("{:?}", c_cache);
  println!("{:?}", c_cache.get(&(5, 3)));
  println!("{:?}", c_cache.get(&(23, 10)));
  println!(
    "{}",
    c_cache.iter().filter(|((_, _), v)| **v > 1000000).count()
  );
}
