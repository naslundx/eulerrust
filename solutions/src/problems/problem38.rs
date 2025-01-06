use util::{digit_count, is_pandigital_str};

pub fn problem38() -> i64 {
    let mut best = 0;
    for n in 1..15000 {
        let mut numbers = vec![];
        let mut digits = 0;
        for m in 1..=9 {
            let term = n * m;
            numbers.push(term);
            digits += digit_count(term);
            if digits >= 9 {
                break;
            }
        }
        if digits > 9 {
            continue;
        }
        let concatenated = numbers
            .iter()
            .fold(String::new(), |acc, n| acc + &n.to_string());

        if is_pandigital_str(&concatenated) {
            let n = concatenated.parse::<i32>().unwrap();
            if n > best {
                best = n;
            }
        }
    }

    best as i64
}
