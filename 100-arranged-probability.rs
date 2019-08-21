// let x be the blue discs
// let y be the total number of discs

// solve
// 2 * x * (x-1) == y * (y-1)
// 0 < x < y
// y > 10^12
// over the integers

// wolfram alpha finds the solution instantly:
// integer solutions of 2 * x * (x-1) == y * (y-1) ; 0 < x < y ; y > 10^12

// the curve is almost a straight line for large values,
// so we start at a point roughly on the line
// by approximating x ≈ y / sqrt(2)
// then by alternating increments of x and y,
// we find the first integer solution

// runtime on my laptop: 6m40s

fn main() {
  let mut y: i128 = 1000000000000;
  let mut x: i128 = 707106781000;
  let mut xx = 2 * x * (x - 1);
  let mut yy = y * (y - 1);
  loop {
    if xx < yy {
      xx += 2 * 2 * x;
      x += 1;
    } else if xx > yy {
      yy += 2 * y;
      y += 1;
    } else {
      break;
    }
  }
  println!("{}", x);
}
