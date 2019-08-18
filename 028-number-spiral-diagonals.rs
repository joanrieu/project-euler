fn main() {
  let sum: u32 = (1..=1001)
    .filter(|i| i % 2 == 1)
    .map(|i| i - 1)
    .flat_map(|i| (1..=4).map(move |_| i))
    .scan(1, |acc, i| {
      *acc += i;
      Some(*acc)
    })
    .skip(3)
    .sum();
  println!("{}", sum);
}
