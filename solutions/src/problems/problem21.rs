use std::collections::HashMap;
use util::divisor_sum;

pub fn problem21() -> i64 {
    let n_max = 10000;
    let mut divisors = HashMap::new();
    for n in 1..n_max {
        divisors.insert(n, divisor_sum(n));
    }

    let mut sum = 0;
    for n in 1..n_max {
        let b = divisors.get(&n).unwrap();
        if let Some(a) = divisors.get(&b) {
            if *a == n && a != b {
                sum += a;
            }
        }
    }

    sum as i64
}
