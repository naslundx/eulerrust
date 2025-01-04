use std::collections::HashSet;

pub fn problem44() -> i64 {
    let mut numbers: HashSet<i64> = HashSet::new();

    for n in 1..5000 {
        let p = n * (3 * n - 1) / 2;
        numbers.insert(p);
    }

    let mut lowest_diff = 999999999;

    for i in numbers.iter() {
        for j in numbers.iter() {
            if i == j {
                continue;
            }
            let sum = i + j;
            let diff = i - j;
            if diff < lowest_diff && numbers.contains(&sum) && numbers.contains(&diff) {
                lowest_diff = diff;
            }
        }
    }

    lowest_diff
}
