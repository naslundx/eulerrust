use std::collections::HashSet;

use bignumber::BigNumber;

mod bignumber;

fn main() {
    let mut numbers = HashSet::new();
    let limit = 100;

    for a in 2..=limit {
        let mut n = BigNumber::from_number(a);

        for _ in 2..=limit {
            n = n.mult(&BigNumber::from_number(a));
            numbers.insert(n.to_string());
        }
    }

    //println!("{:?}", numbers);
    println!("{}", numbers.len());
}
