pub fn problem15() -> i64 {
    let mut n_idx = 40i128;
    let mut k_idx = 20i128;
    let mut result = 1i128;

    loop {
        if n_idx >= 21 {
            result *= n_idx;
            n_idx -= 1;
        }

        if result % k_idx == 0 {
            result /= k_idx;
            k_idx -= 1;
        }

        if n_idx < 21 && k_idx < 2 {
            break;
        }
    }

    result as i64
}