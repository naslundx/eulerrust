use util::digit_count;

pub fn problem40() -> i64 {
    let mut digits_seen = 0;
    let mut idx = 0;
    let mut looking_for = 1;
    let mut product = 1;

    while looking_for <= 1000000 {
        idx += 1;
        digits_seen += digit_count(idx);

        if digits_seen >= looking_for {
            let mut n = idx;
            for _ in 0..(digits_seen - looking_for) {
                n /= 10;
            }

            let digit = n % 10;
            product *= digit;
            looking_for *= 10;
        }
    }

    product as i64
}
