use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn problem18() -> i64 {
    let path = Path::new("assets/problem18.txt");
    let file = File::open(&path).expect("Could not open file");
    let mut lines: Vec<Vec<i64>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|s| s.split(" ").map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    for line_count in (1..lines.len()).rev() {
        let current = lines[line_count].clone();
        let next = &mut lines[line_count - 1];

        for idx in 1..current.len() {
            let left = current[idx - 1];
            let right = current[idx];
            next[idx - 1] += std::cmp::max(left, right);
        }
    }
    
    lines[0][0]
}
