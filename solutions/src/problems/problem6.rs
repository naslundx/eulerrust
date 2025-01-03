pub fn problem6() -> i64 {
    let mut result: i64 = 0;
    let mut sum: i64 = 0;

    for i in 1..=100 {
        result += i * i;
        sum += i;
    }

    let sum_squared = sum * sum;
    let diff = sum_squared - result;

    diff
}
