use std::collections::btree_set::*;

fn main() {
  let range = 1..1000000;

  let digitize = |base| {
    move |n| {
      let mut n = n;
      let mut digits = vec![];
      while n > 0 {
        digits.insert(0, n % base);
        n /= base;
      }
      digits
    }
  };

  let reverse = |digits: &Vec<i32>| {
    let mut digits = digits.clone();
    digits.reverse();
    digits
  };

  let rebuild = |base: i32| {
    move |digits: &Vec<i32>| {
      let mut n = 0;
      for d in digits {
        n *= base;
        n += d;
      }
      n
    }
  };

  let base_ten = range.clone().map(digitize(10)).collect::<Vec<_>>();
  let base_two = range.clone().map(digitize(2)).collect::<Vec<_>>();

  let base_ten_palindromes = base_ten
    .iter()
    .filter(|digits| **digits == reverse(digits))
    .map(rebuild(10))
    .collect::<BTreeSet<_>>();
  let base_two_palindromes = base_two
    .iter()
    .filter(|digits| **digits == reverse(digits))
    .map(rebuild(2))
    .collect::<BTreeSet<_>>();

  let double_base_palindromes = base_ten_palindromes
    .intersection(&base_two_palindromes)
    .cloned()
    .collect::<BTreeSet<_>>();
  println!("{:?}", double_base_palindromes);
  println!("sum: {}", double_base_palindromes.iter().sum::<i32>());
}
