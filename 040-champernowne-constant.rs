fn main() {
  let decimals = (1..).flat_map(|n| {
    let mut n = n;
    let mut digits = vec![];
    while n > 0 {
      digits.insert(0, n % 10);
      n /= 10;
    }
    digits
  });
  let mut chosen = vec![];
  for (i, d) in decimals.enumerate() {
    match i + 1 {
      1 | 10 | 100 | 1000 | 10000 | 100000 | 1000000 => chosen.push(d),
      1000001 => break,
      _ => (),
    }
  }
  println!("{:?}", chosen);
  let p = chosen
    .iter()
    .scan(1, |p, n| {
      *p *= n;
      Some(*p)
    })
    .last()
    .unwrap();
  println!("{}", p);
}
