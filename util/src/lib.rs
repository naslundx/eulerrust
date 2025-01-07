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
