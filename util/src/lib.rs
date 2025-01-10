pub fn is_palindrome(n: i32) -> bool {
    let mut divisor_left = 10f64.powf((n as f64).log10().floor()) as i32;
    let mut divisor_right = 1;

    while divisor_left > divisor_right {
        let (digit_left, digit_right) = ((n / divisor_left) % 10, (n / divisor_right) % 10);

        if digit_left != digit_right {
            return false;
        }

        divisor_left /= 10;
        divisor_right *= 10;
    }

    true
}

pub fn get_primes(limit: usize) -> Vec<i64> {
    let mut primes = vec![2];

    let mut candidate = 3;

    while primes.len() <= limit {
        if primes
            .iter()
            .take_while(|&&p| p * p <= candidate)
            .all(|&p| candidate % p != 0)
        {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes
}

pub fn to_binary(n: i128) -> i128 {
    let mut i = n as u64;
    let mut d = 2u64.pow((n as f64).log2().ceil() as u32);
    let mut digits = vec![];

    while i != 0 || d != 0 {
        //println!("{i}, {d}");
        if d <= i {
            digits.push(1);
            i -= d;
        } else {
            digits.push(0);
        }
        d /= 2;
    }

    digits.iter().fold(0i128, |acc, nxt| acc * 10 + nxt)
}

pub fn is_palindrome_i128(n: i128) -> bool {
    let mut digits = vec![];
    let mut i = n;
    while i > 0 {
        let digit = i % 10;
        digits.push(digit);
        i /= 10;
    }
    let rev_n = digits.iter().fold(0i128, |acc, x| acc * 10 + x);
    n == rev_n
}

pub fn digit_count(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    (n as f64).log10().floor() as i32 + 1
}

pub fn is_pandigital_str(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    for i in 1..=9 {
        if !s.contains(&i.to_string()) {
            return false;
        }
    }
    true
}

pub fn divisor_sum(n: i32) -> i32 {
    1 + (2..n).filter(|d| n % d == 0).sum::<i32>()
}

pub fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(123));
        assert!(is_palindrome(1221));
        assert!(!is_palindrome(1231));
    }

    #[test]
    fn test_get_primes() {
        assert_eq!(get_primes(0), vec![2]);
        assert_eq!(get_primes(1), vec![2, 3]);
        assert_eq!(get_primes(2), vec![2, 3, 5]);
        assert_eq!(get_primes(5), vec![2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary(0), 0);
        assert_eq!(to_binary(1), 1);
        assert_eq!(to_binary(2), 10);
        assert_eq!(to_binary(5), 101);
        assert_eq!(to_binary(10), 1010);
    }

    #[test]
    fn test_is_palindrome_i128() {
        assert!(is_palindrome_i128(121));
        assert!(!is_palindrome_i128(123));
        assert!(is_palindrome_i128(1221));
        assert!(!is_palindrome_i128(1231));
    }

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(123), 3);
    }

    #[test]
    fn test_is_pandigital_str() {
        assert!(is_pandigital_str("123456789"));
        assert!(!is_pandigital_str("12345678"));
        assert!(!is_pandigital_str("112345678"));
        assert!(!is_pandigital_str("123456780"));
    }

    #[test]
    fn test_divisor_sum() {
        assert_eq!(divisor_sum(1), 1);
        assert_eq!(divisor_sum(6), 6);
        assert_eq!(divisor_sum(28), 28);
        assert_eq!(divisor_sum(12), 16);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}
