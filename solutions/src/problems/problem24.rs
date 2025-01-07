use itertools::Itertools;

pub fn problem24() -> i64 {
    let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for (idx, perm) in items.iter().permutations(items.len()).enumerate() {
        if idx == 999_999 {
            return perm.iter().fold(String::from(""), |acc, nxt| acc + &nxt.to_string()).parse::<i64>().unwrap();
        }
    }
    0i64
}
