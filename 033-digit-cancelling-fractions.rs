fn main() {
  let mut a_product = 1;
  let mut b_product = 1;
  for a in 10..100 {
    for b in (a + 1)..100 {
      let a_digits = (a / 10, a % 10);
      let b_digits = (b / 10, b % 10);
      if a_digits.0 == b_digits.0
        || a_digits.0 == b_digits.1
        || a_digits.1 == b_digits.0
        || a_digits.1 == b_digits.1
      {
        let a_diff = if a_digits.0 != b_digits.0 && a_digits.0 != b_digits.1 {
          a_digits.0
        } else {
          a_digits.1
        };
        let b_diff = if b_digits.0 != a_digits.0 && b_digits.0 != a_digits.1 {
          b_digits.0
        } else {
          b_digits.1
        };
        if a * b_diff == a_diff * b && a != a_diff * 10 {
          a_product *= a_diff;
          b_product *= b_diff;
          println!("{}/{} = {}/{}", a, b, a_diff, b_diff);
        }
      }
    }
  }
  println!("{}/{}", a_product, b_product);
}
