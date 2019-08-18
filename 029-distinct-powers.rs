fn main() {
  let primes = (2..=100)
    .filter(|n| (2..*n).all(|i| n % i != 0))
    .collect::<Vec<_>>();
  println!("{:?}", primes);
  let decomposition = (2..=100)
    .map(|mut n| {
      (
        n,
        primes
          .iter()
          .map(move |p| {
            let mut count = 0;
            while n % p == 0 {
              count += 1;
              n /= p;
            }
            (p, count)
          })
          .filter(|(_p, count)| *count > 0)
          .collect::<Vec<_>>(),
      )
    })
    .collect::<Vec<_>>();
  println!("{:?}", decomposition);
  let pow = |a: i32, b: i32| {
    decomposition[decomposition.binary_search_by_key(&a, |(n, _factors)| *n).unwrap()].1.iter().map(|&(p, count)| (p, count * b)).collect::<Vec<_>>()
  };
  let mut powers = (2..=100)
    .flat_map(|a| (2..=100).map(move |b| pow(a, b)))
    .collect::<Vec<_>>();
  println!("{} before dedup", powers.len());
  powers.sort();
  powers.dedup();
  println!("{}", powers.len());
}
