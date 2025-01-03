mod bignumber;

fn main() {
    let mut best_idx = 0;
    let mut best_result = 0;
    for n in 4..=1000 {
        let result = number_of_solutions(n);
        if result > best_result {
            best_result = result;
            best_idx = n;
        }
    }
    println!("{}, {}", best_idx, best_result);
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
