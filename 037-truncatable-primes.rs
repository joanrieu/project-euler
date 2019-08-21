use std::collections::btree_set::*;

fn main() {
  let is_prime = |p| p > 1 && (2..=((p as f32).sqrt() as usize)).all(|d| p % d != 0);
  let mut sum = 0;
  for p in 10.. {
    let mut splits = vec![];
    let mut p1 = p;
    while p1 > 0 {
      splits.push(p1);
      p1 /= 10;
    }
    let mut mask = (10 as f32).powf((splits.len() - 1) as f32) as usize;
    while mask > 1 {
      splits.push(p % mask);
      mask /= 10;
    }
    if splits.iter().cloned().all(is_prime) {
      sum += p;
      println!("{:?} (sum: {})", splits, sum);
    }
  }
}
