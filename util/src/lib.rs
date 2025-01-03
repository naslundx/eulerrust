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