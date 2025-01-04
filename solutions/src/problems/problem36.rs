use util::{is_palindrome_i128, to_binary};

pub fn problem36() -> i64 {
    let mut sum = 0i64;
    for i in 1..1000000 {
        let ib = to_binary(i);
        if is_palindrome_i128(i) && is_palindrome_i128(ib) {
            sum += i as i64;
        }
    }
    sum
}
