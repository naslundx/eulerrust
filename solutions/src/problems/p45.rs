use std::collections::HashSet;

fn main() {
    let mut triangles = HashSet::new();
    let mut pentagonals = HashSet::new();
    let mut hexagonals = HashSet::new();

    for n in 1..100000 {
        triangles.insert(n * (n + 1) / 2 as i64);
        pentagonals.insert(n * (3 * n - 1) / 2 as i64);
        hexagonals.insert(n * (2 * n - 1) as i64);
    }

    let mut common: HashSet<i64> = triangles.intersection(&pentagonals).cloned().collect();
    common = common.intersection(&hexagonals).cloned().collect();

    println!("{:?}", common);
}
