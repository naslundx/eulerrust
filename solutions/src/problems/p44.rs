use std::collections::HashSet;

fn main() {
    let mut numbers: HashSet<i64> = HashSet::new();

    for n in 1..5000 {
        let p = n * (3 * n - 1) / 2;
        numbers.insert(p);
    }

    println!("searching");

    for i in numbers.iter() {
        for j in numbers.iter() {
            if i == j {
                continue;
            }
            let sum = i + j;
            let diff = i - j;
            if numbers.contains(&sum) && numbers.contains(&diff) {
                println!("{}", diff);
            }
        }
    }
}
