use std::collections::HashMap;

pub fn problem30() -> i64 {
    let numbers = (0u64..=9u64).map(|n| n.pow(5));
    let fifths: HashMap<u64, u64> = HashMap::from_iter((0u64..=9u64).zip(numbers));

    (2..400000).filter(|n| is_match(*n, &fifths)).sum::<u64>() as i64
}

fn is_match(n: u64, fifths: &HashMap<u64, u64>) -> bool {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += fifths[&(num % 10)];
        num /= 10;
    }
    sum == n as u64
}
