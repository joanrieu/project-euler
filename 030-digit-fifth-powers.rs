fn main() {
  let digit_powers = vec![
    0 * 0 * 0 * 0 * 0,
    1 * 1 * 1 * 1 * 1,
    2 * 2 * 2 * 2 * 2,
    3 * 3 * 3 * 3 * 3,
    4 * 4 * 4 * 4 * 4,
    5 * 5 * 5 * 5 * 5,
    6 * 6 * 6 * 6 * 6,
    7 * 7 * 7 * 7 * 7,
    8 * 8 * 8 * 8 * 8,
    9 * 9 * 9 * 9 * 9,
  ];
  println!("{:?}", digit_powers);
  let sum_digit_powers = |n| {
    (
      n,
      format!("{}", n)
        .into_bytes()
        .iter()
        .map(|c| c - 48)
        .map(|i| digit_powers[i as usize])
        .sum::<i64>(),
    )
  };
  let numbers = (10..1000000)
    .map(sum_digit_powers)
    .filter(|(n, sum)| n == sum)
    .map(|(n, _)| n)
    .collect::<Vec<_>>();
  println!("{:?}", numbers);
  println!("{}", numbers.iter().sum::<i64>());
}
