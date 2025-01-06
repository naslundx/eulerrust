use std::collections::HashSet;
use util::divisor_sum;

pub fn problem23() -> i64 {
    let limit = 28123;
    let mut map: HashSet<i32> = HashSet::new();

    for i in 2..limit {
        if divisor_sum(i) > i {
            map.insert(i);
        }
    }

    let mut sum = 0;
    for i in 1..=limit {
        let mut candidate = true;
        for j in &map {
            let diff = i - j;
            if diff > 0 && map.contains(&diff) {
                candidate = false;
                break;
            }
        }
        if candidate {
            sum += i;
        }
    }

    sum as i64
}
