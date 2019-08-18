fn main() {
  let max = (1..=1000).max_by_key(|n| div(*n)).unwrap();
  print!("max: ");
  div(max);
}

fn div(n: u32) -> usize {
  let mut r = 1;
  // print!("{}/{} = 1.", r, n);
  // let mut decimals = vec![];
  let mut remainders = vec![];
  while r > 0 {
    r *= 10;
    let q = r / n;
    // decimals.push(q);
    r -= q * n;
    let sequence = remainders.iter().skip_while(|r2| **r2 != r).count();
    if sequence > 0 {
      println!("{} loops over {} values", n, sequence);
      return sequence;
    }
    remainders.push(r);
    // print!("{}", q);
    // print!("{} ", r);
  }
  return 0;
}
