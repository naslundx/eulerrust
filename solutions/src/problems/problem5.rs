use util::get_primes;
use std::cmp::max;
use std::collections::HashMap;

pub fn problem5() -> i64 {
    let primes = get_primes(20);
    let mut factors: HashMap<i64, i64> = HashMap::new();

    for i in 2..=20 {
        let mut new_factors: HashMap<i64, i64> = HashMap::new();
        let mut n = i;

        while !primes.contains(&n) {
            for p in &primes {
                if n % p == 0 {
                    new_factors.entry(*p).and_modify(|v| *v += 1).or_insert(1);
                    n /= *p;
                    break;
                }
            }
        }

        new_factors.entry(n).and_modify(|v| *v += 1).or_insert(1);

        for (k, v) in new_factors {
            let v2 = factors.get(&k).unwrap_or(&0);
            factors.insert(k, max(v, *v2));
        }
    }

    factors.iter().map(|(&k, &v)| k.pow(v as u32) as i64).product()
}