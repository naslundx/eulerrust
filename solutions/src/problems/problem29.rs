use std::collections::HashSet;

use big_number::BigNumber;

pub fn problem29() -> i64 {
    let mut numbers = HashSet::new();
    let limit = 100;

    for a in 2..=limit {
        let mut n = BigNumber::from_number(a);

        for _ in 2..=limit {
            n = n.mult(&BigNumber::from_number(a));
            numbers.insert(n.to_string());
        }
    }

    numbers.len() as i64
}
