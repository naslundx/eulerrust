fn main() {
    let mut result: i64 = 0;
    let mut sum: i64 = 0;

    for i in 1..=100 {
        result += i * i;
        sum += i;
    }

    let sum_squared = sum * sum;

    println!("{sum_squared}, {result}");

    let diff = sum_squared - result;

    println!("{diff}, {}", diff * diff)
}
