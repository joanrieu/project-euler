fn main() {
  // formula 1: # x #### = ####
  // formula 2: ## x ### = ####
  let mut products: Vec<u32> = vec![];
  fn with_digit(seq: Vec<u32>) -> Vec<Vec<u32>> {
    let mut seqs = vec![];
    for i in 1..10 {
      if !seq.contains(&i) {
        let mut seq = seq.clone();
        seq.push(i);
        seqs.push(seq);
      }
    }
    seqs
  }
  for digits in vec![vec![]]
    .iter()
    .cloned()
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
    .flat_map(with_digit)
  {
    for formula in 1..=2 {
      let mut digits = digits.clone();
      let (a, b) = {
        let mut pick = || digits.pop().unwrap();
        if formula == 1 {
          (pick(), pick() * 1000 + pick() * 100 + pick() * 10 + pick())
        } else {
          (pick() * 10 + pick(), pick() * 100 + pick() * 10 + pick())
        }
      };
      let c = a * b;
      if c <= 9999 {
        let c_digits = vec![c / 1000, (c / 100) % 10, (c / 10) % 10, c % 10];
        if c_digits == digits {
          println!("{} x {} = {}", a, b, c);
          products.push(c);
        }
      }
    }
  }
  products.sort();
  products.dedup();
  println!("{}", products.iter().sum::<u32>());
}
