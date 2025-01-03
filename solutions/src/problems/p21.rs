use std::collections::HashMap;

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

fn divisor_sum(n: i32) -> i32 {
    1 + (2..n).filter(|d| n % d == 0).sum()
}
