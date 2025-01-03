pub fn problem1() -> i64 {
    let mut total = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    return total;
}
