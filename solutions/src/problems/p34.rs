use std::collections::HashMap;

fn main() {
    let mut factorials = HashMap::new();

    for i in 0..=9 {
        factorials.insert(i, (1..=i).product::<u64>());
    }

    for i in 1..100000000 {
        if is_match(i, &factorials) {
            println!("{}", i);
        }
    }
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
