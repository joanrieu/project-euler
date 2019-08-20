fn main() {
  let values = vec![1, 2, 5, 10, 20, 50, 100, 200];
  let mut count = 0;
  let mut ways = vec![vec![]];
  while ways.len() > 0 {
    ways = ways
      .iter()
      .flat_map(|way| {
        values
          .iter()
          .map(|coin| {
            let mut way = way.clone();
            way.push(*coin);
            way.sort();
            way
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    ways.sort();
    ways.dedup();
    ways.retain(|way| {
      let sum = way.iter().sum::<u32>();
      let target = 200;
      if sum < target {
        true
      } else {
        if sum == target {
          count += 1;
          // eprintln!("{} → {:?}", count, way);
        }
        false
      }
    })
  }
  println!("total: {}", count);
}
