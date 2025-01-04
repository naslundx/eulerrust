use big_number::BigNumber;
use std::fs::read_to_string;

pub fn problem13() -> i64 {
    read_to_string(&String::from("assets/problem13.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .map(|number| BigNumber::from_str(&number))
        .fold(BigNumber::zero(), |acc, num| acc.add(&num))
        .to_string()
        .chars()
        .take(10)
        .collect::<String>()
        .parse()
        .unwrap()
}
