use std::collections::HashMap;
use util::get_primes;

pub fn problem12() -> i64 {
    let primes = get_primes(10_000);

    let mut n: i64 = 0;
    let mut inc: i64 = 1;

    while divisors(n, &primes) <= 500 {
        n += inc;
        inc += 1;
    }

    n
}

fn divisors(mut n: i64, primes: &Vec<i64>) -> i64 {
    let mut factors: HashMap<i64, i64> = HashMap::new();

    while n > 1 {
        for &p in primes {
            if n % p == 0 {
                n /= p;
                *factors.entry(p).or_insert(0) += 1;
                break;
            }

            if p > n {
                break;
            }
        }
    }

    let mut product = 1;
    for (_key, value) in &factors {
        product *= value + 1;
    }
    product
}
