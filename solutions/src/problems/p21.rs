use std::collections::HashMap;
use utils::divisor_sum;

fn main() {
    let mut divisors = HashMap::new();
    for n in 1..10000 {
        divisors.insert(n, divisor_sum(n));
    }

    let mut sum = 0;
    for n in 1..10000 {
        let b = divisors.get(&n).unwrap();
        if let Some(a) = divisors.get(&b)
            && *a == n
            && a != b
        {
            sum += a;
        }
    }

    println!("{sum}");
}
