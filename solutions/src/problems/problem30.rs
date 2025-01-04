use std::collections::HashMap;

pub fn problem30() -> i64 {
    let mut fifths: HashMap<u64, u64> = HashMap::new();

    for i in 0..=9 {
        fifths.insert(i, i.pow(5));
    }

    let sum = (2u64..400000u64)
        .filter(|n| is_match(*n, &fifths))
        .sum::<u64>() as i64;

    sum
}

fn is_match(n: u64, fifths: &HashMap<u64, u64>) -> bool {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += fifths[&(num % 10)];
        num /= 10;
    }
    sum == n
}
