pub fn problem48() -> i64 {
    let digit_limit = 100000000000;
    let mut sum = 0;

    for i in 1..=1000 {
        sum += get_term(i);
        if sum > digit_limit {
            sum %= digit_limit;
        }
    }

    sum as i64
}

fn get_term(n: i128) -> i128 {
    let digit_limit = 100000000000;
    let mut result = 1;
    for _ in 0..n {
        result *= n;
        if result > digit_limit {
            result %= digit_limit;
        }
    }
    result
}
