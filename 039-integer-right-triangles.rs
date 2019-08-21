use std::collections::btree_map::*;

fn main() {
  let mut score = BTreeMap::new();
  for a in 1..1000 {
    for b in 1..1000 {
      let c2 = a * a + b * b;
      let c = (c2 as f32).sqrt() as u32;
      if c * c == c2 {
        let sum = a + b + c;
        if sum <= 1000 {
          let count = **score.get(&sum).get_or_insert(&0);
          score.insert(sum, count + 1);
          println!("{} + {} + {} = {}", a, b, c, sum);
        }
      }
    }
  }
  println!("{:?}", score);
  let best = score.iter().max_by_key(|(_, v)| *v).unwrap().0;
  println!("best: {}", best);
}
