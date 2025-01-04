use big_number::BigNumber;
use std::fs::read_to_string;

fn main() {
    let filename = String::from("p13data.txt");
    let numbers: Vec<BigNumber> = read_to_string(&filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|number| BigNumber { number })
        .collect();

    let result = numbers
        .iter()
        .fold(BigNumber::zero(), |acc, num| acc.add(num));

    println!("{}", result.number);
}
