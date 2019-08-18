fn main() {
    let prime_cache = PrimeCache::new();
    let mut qfs = vec![];
    qfs.reserve(1999 * 2001);
    for a in 0..=1998 {
        let a = (a as i64) - 999;
        for b in 0..=2000 {
            let b = (b as i64) - 1000;
            let qf = QuadFormula::new(a, b, &prime_cache);
            qfs.push(qf);
        }
    }
    let qf = qfs.iter_mut().flatten().max_by_key(|qf| qf.n).unwrap();
    println!("a={} b={} n={}", qf.a, qf.b, qf.n);
    println!("a*b={}", qf.a * qf.b);
}

#[derive(Copy, Clone)]
struct QuadFormula<'a> {
    a: i64,
    b: i64,
    n: i64,
    prime_cache: &'a PrimeCache,
}

impl<'a> QuadFormula<'a> {
    fn new(a: i64, b: i64, prime_cache: &'a PrimeCache) -> Self {
        Self {
            a,
            b,
            n: 0,
            prime_cache,
        }
    }

    fn is_prime(self: &Self) -> bool {
        let prime = self.n * self.n + self.a * self.n + self.b;
        self.prime_cache.is_prime(prime)
    }
}

impl<'a> Iterator for QuadFormula<'a> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_prime() {
            let prime_self = Some(*self);
            self.n += 1;
            prime_self
        } else {
            // eprintln!("dead {} {} after {} steps", self.a, self.b, self.n);
            None
        }
    }
}

struct PrimeCache {
    primes: std::cell::RefCell<Vec<i64>>,
    max_prime_checked: std::cell::RefCell<i64>,
}

impl PrimeCache {
    fn new() -> PrimeCache {
        PrimeCache {
            primes: std::cell::RefCell::new(vec![2]),
            max_prime_checked: std::cell::RefCell::new(2),
        }
    }

    fn is_prime(self: &Self, n: i64) -> bool {
        let mut primes = self.primes.borrow_mut();
        let mut max_prime_checked = self.max_prime_checked.borrow_mut();
        while n > *max_prime_checked {
            let n = *max_prime_checked + 1;
            let mut is_prime = true;
            for i in 2..n {
                if n % i == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(n);
            }
            *max_prime_checked += 1;
        }
        let is_prime = primes.binary_search(&n).is_ok();
        // eprintln!("is_prime({}) = {}", n, is_prime);
        is_prime
    }
}
