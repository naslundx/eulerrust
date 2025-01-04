pub fn problem39() -> i64 {
    let mut best_idx = 0;
    let mut best_result = 0;
    for p in 4..=1000 {
        let result = number_of_solutions(p);
        if result > best_result {
            best_result = result;
            best_idx = p;
        }
    }
    best_idx
}

fn number_of_solutions(n: i64) -> i64 {
    let mut count = 0;
    for a in 1..n {
        for b in a..n - a {
            let c = n - a - b;
            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }
    count
}
