use std::fs::File;
use std::io::{self, BufRead};

pub fn problem42() -> i64 {
    let file = File::open("words.txt").unwrap(); // TODO
    let first_line = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .replace("\"", "");

    let triangle_numbers: Vec<i32> = (1..=26).map(|n| n * (n + 1) / 2).collect();

    let count = first_line
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| triangle_numbers.contains(&score(s)))
        .count();

    count as i64
}

fn score(word: &str) -> i32 {
    word.chars().map(|c| c as i32 - 'A' as i32 + 1).sum()
}
