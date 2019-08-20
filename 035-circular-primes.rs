use std::collections::btree_set::*;

fn main() {
  let max = 1000000;
  let not_primes = (2..max)
    .flat_map(|i| (2..=(max / i)).map(|j| i * j).collect::<Vec<_>>())
    .collect::<BTreeSet<_>>();
  let primes = (2..max)
    .collect::<BTreeSet<_>>()
    .difference(&not_primes)
    .cloned()
    .collect::<Vec<_>>();
  let mut circular = 0;
  for p in primes.iter() {
    let mut digits = vec![];
    {
      let mut p = *p;
      while p > 0 {
        digits.insert(0, p % 10);
        p /= 10;
      }
    }
    if (1..(digits.len()))
      .scan(digits.clone(), |rotated, _| {
        let last = rotated.pop().unwrap();
        rotated.insert(0, last);
        let mut p = 0;
        for d in rotated {
          p *= 10;
          p += *d;
        }
        Some(p)
      })
      .all(|rotation| primes.contains(&rotation))
    {
      circular += 1;
      println!("{:?}", p);
    }
  }
  println!("count: {:?}", circular);
}
