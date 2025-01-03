use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let triangle_numbers: Vec<i32> = (1..=26).map(|n| n * (n + 1) / 2).collect();

    let file = File::open("words.txt").unwrap();
    let first_line = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .replace("\"", "");

    let count = first_line
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| triangle_numbers.contains(&score(s)))
        .count();

    println!("{}", count);
}

fn score(word: &str) -> i32 {
    word.chars().map(|c| c as i32 - 'A' as i32 + 1).sum()
}
