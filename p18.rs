use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(&path).expect("Could not open file");
    let mut lines: Vec<Vec<i32>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|s| s.split(" ").map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    let mut line_count = lines.len();

    while line_count >= 1 {
        line_count -= 1;

        let current = lines[line_count].clone();
        let next = &mut lines[line_count - 1];

        for idx in 1..current.len() {
            let left = current[idx - 1];
            let right = current[idx];
            let max = std::cmp::max(left, right);
            next[idx - 1] += max;
        }

        println!("{:?}", next);
    }
}
