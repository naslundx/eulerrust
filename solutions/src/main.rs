extern crate big_number;
extern crate util;

use std::env;

mod problems;
use problems::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).map(String::as_str).unwrap_or("-1");

    let problem_id = arg.parse::<i32>();

    let result = match problem_id {
        Ok(1) => problem1::problem1(),
        Ok(2) => problem2::problem2(),
        Ok(4) => problem4::problem4(),
        Ok(6) => problem6::problem6(),
        Ok(16) => problem16::problem16(),
        Ok(45) => problem45::problem45(),

        _ => panic!("Invalid problem ID")
    };

    println!("Problem {}: {}", problem_id.unwrap(), result);
}