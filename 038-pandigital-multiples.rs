fn main() {
  let to_digits = |n: u32| {
    let mut n = n;
    let mut digits = vec![];
    while n > 0 {
      digits.insert(0, n % 10);
      n /= 10;
    }
    digits
  };
  let product = |n: u32, i: u32| (1..=i).flat_map(|i| to_digits(n * i)).collect::<Vec<_>>();
  let is_valid = |nd: &Vec<u32>| {
    let mut nd = nd.clone();
    nd.sort();
    nd == (1..=9).collect::<Vec<_>>()
  };
  let is_out_of_bounds = |nd: &Vec<u32>| {
    let mut n = 0;
    for d in nd {
      n *= 10;
      n += d;
    }
    n > 999999999
  };
  for n in 1..=9999 {
    for i in 1.. {
      let p = product(n, i);
      if is_out_of_bounds(&p) {
        break;
      } else if is_valid(&p) {
        println!("{:?}", p);
      }
    }
  }
}
