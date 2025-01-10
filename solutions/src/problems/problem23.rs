use std::collections::HashSet;
use util::divisor_sum;

pub fn problem23() -> i64 {
    let limit = 28123;
    let abundants = (2..limit).filter(|&i| divisor_sum(i) > i);
    let map: HashSet<i32> = HashSet::from_iter(abundants);

    let mut sum = 0;
    for i in 1..=limit {
        if let None = map
            .iter()
            .filter(|&&x| x < i)
            .find(|&&x| map.contains(&(i - x)))
        {
            sum += i;
        }
    }

    sum as i64
}
