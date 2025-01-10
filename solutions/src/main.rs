#![allow(dead_code)]

use std::env;

mod problems;
use problems::*;

macro_rules! problem_calls {
    ($problem_id:expr, $($i:literal),*) => {
        match $problem_id {
            $(
                $i => paste::paste! { [<problem $i>]::[<problem $i>]() },
            )*
            _ => panic!("Invalid id"),
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem_id = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("-1")
        .parse::<i32>()
        .expect("Valid problem ID");

    let result = problem_calls!(
        problem_id, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
        46, 48, 49, 50
    );

    println!("Problem {}: {}", problem_id, result);
}
