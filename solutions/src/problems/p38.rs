use util::{digit_count, is_pandigital_str};

fn main() {
    for n in 1..15000 {
        let mut numbers = vec![];
        let mut digit_count = 0;
        let mut m_max = 0;
        for m in 1..=9 {
            m_max = m;
            let term = n * m;
            numbers.push(term);
            digit_count += digit_count(term);
            if digit_count >= 9 {
                break;
            }
        }
        if digit_count > 9 {
            continue;
        }
        let concatenated = numbers
            .iter()
            .fold(String::new(), |acc, n| acc + &n.to_string());

        if is_pandigital_str(&concatenated) {
            println!("{n}, 1..{m_max} -> {concatenated}");
            let q: i32 = concatenated.parse().expect("i32 size");
            println!("{q}");
        }
    }
}
