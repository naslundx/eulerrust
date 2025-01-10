use std::fs::read_to_string;

pub fn problem22() -> i64 {
    let filename = String::from("assets/problem22.txt");
    let mut names: Vec<String> = read_to_string(&filename)
        .unwrap()
        .split(",")
        .map(String::from)
        .map(|s| s.replace("\"", ""))
        .collect();

    names.sort();

    let total = names
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, nxt)| acc + score(&nxt) * (idx as u64 + 1));

    total as i64
}

fn score(name: &str) -> u64 {
    name.chars()
        .fold(0, |cur, nxt| cur + (nxt as u64 - 'A' as u64 + 1))
}
