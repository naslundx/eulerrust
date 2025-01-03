use std::fs::read_to_string;

fn main() {
    let filename = String::from("p22data.txt");
    let mut names: Vec<String> = read_to_string(&filename) 
        .unwrap()
        .split(",")
        .map(String::from)
        .map(|s| s.replace("\"", ""))
        .collect();

    //names.truncate(3);
    names.sort();

    let total = names
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, nxt)| acc + score(&nxt) * (idx as u64 + 1));
        
    println!("{total}");
}

fn score(name: &str) -> u64 {
    name.chars().fold(
        0, |cur, nxt| cur + (nxt as u64 - 'A' as u64 + 1)
    )
}