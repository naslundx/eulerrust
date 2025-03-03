use std::collections::HashSet;

pub fn problem27() -> i64 {
    let mut primes: HashSet<i64> = HashSet::new();
    let n = 100000;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            primes.insert(i as i64);
            let mut multiple = i * 2;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    let mut best_a = 0;
    let mut best_b = 0;
    let mut best_run = 0;

    for b in -1000..=1000 {
        if !primes.contains(&(b as i64).abs()) {
            continue;
        }
        for a in -999..1000 {
            let mut n = 0;

            loop {
                n += 1;
                let p = n * n + a * n + b;
                if !primes.contains(&p) {
                    break;
                }
            }

            if n > best_run {
                (best_a, best_b, best_run) = (a, b, n);
            }
        }
    }

    best_a * best_b
}
