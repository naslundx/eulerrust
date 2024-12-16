use std::collections::HashMap;

fn main() {
    let primes = get_primes(10_000);

    let mut n: i64 = 0;
    let mut inc: i64 = 1;

    loop {
        n += inc;
        inc += 1; 

        let count = divisors(n, &primes);

        if count > 500 {
            println!("{n} has {count} divisors");
            break;
        }
    }
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

fn get_primes(limit: usize) -> Vec<i64> {
    let mut primes = vec![2];
    primes.push(2);

    let mut candidate = 3;

    while primes.len() < limit {
        if primes.iter().take_while(|&&p| p * p <= candidate).all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes
}