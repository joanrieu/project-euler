fn main() {
    add_digits(&vec![], &mut 0);
}

fn add_digits(digits: &Vec<i8>, count: &mut u64) {
    let target = 1000000;
    if *count < target {
        let digits_left = 10 - digits.len() as u64;
        if digits_left > 0 {
            let branch_count = factorial_or_zero(digits_left);
            println!("{} + {} with {} digits left", count, branch_count, digits_left);
            if *count + branch_count < target {
                *count += branch_count;
            } else {
                for digit in (0..=9).filter(|digit| !digits.contains(digit)) {
                    let mut digits = digits.clone();
                    digits.push(digit);
                    add_digits(&digits, count);
                }
            }
        } else {
            *count += 1;
            if *count == target {
                println!("{} → {:?}", target, digits);
            }
        }
    }
}

fn factorial_or_zero(mut n: u64) -> u64 {
    if n > 0 {
        let mut f = 1;
        while n > 1 {
            f *= n;
            n -= 1;
        }
        f
    } else {
        0
    }
}
