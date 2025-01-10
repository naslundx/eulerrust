use util::{is_palindrome_i128, to_binary};

pub fn problem36() -> i64 {
    (1i128..1_000_000i128)
        .filter(|&x| is_palindrome_i128(x))
        .filter(|&x| is_palindrome_i128(to_binary(x)))
        .sum::<i128>() as i64
}
