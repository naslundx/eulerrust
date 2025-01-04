use std::collections::HashMap;

pub fn problem34() -> i64 {
    let mut factorials = HashMap::new();

    for i in 0..=9 {
        factorials.insert(i, (1..=i).product::<u64>());
    }

    let mut sum = 0;
    for i in 1..100000000 {
        if is_match(i, &factorials) {
            sum += i;
        }
    }
    sum as i64
}

fn is_match(n: u64, factorials: &HashMap<u64, u64>) -> bool {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += factorials[&(num % 10)];
        num /= 10;
    }
    sum == n
}
