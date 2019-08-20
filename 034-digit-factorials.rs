fn main() {
  let mut sum = 0;
  let mut factorials = vec![1, 1];
  for i in 2..10 {
    factorials.push(i * factorials[i - 1]);
  }
  // println!("factorials: {:?}", factorials);
  for number in 3..1000000 {
    let digits = {
      let mut number = number;
      let mut digits = vec![];
      while number > 0 {
        digits.insert(0, number % 10);
        number /= 10;
      }
      digits
    };
    // println!("{} = {:?}", number, digits);
    if digits.iter().map(|d| factorials[*d]).sum::<usize>() == number {
      sum += number;
      println!("{} (sum = {})", number, sum);
    }
  }
}
