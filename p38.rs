fn main() {
    for n in 1..15000 {
        let mut numbers = vec![];
        let mut digit_count = 0;
        let mut m_max = 0;
        for m in 1..=9 {
            m_max = m;
            let term = n * m;
            numbers.push(term);
            digit_count += digits(term);
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

        if is_pandigital(&concatenated) {
            println!("{n}, 1..{m_max} -> {concatenated}");
            let q: i32 = concatenated.parse().expect("i32 size");
            println!("{q}");
        }
    }
}

fn digits(n: i32) -> i32 {
    n.to_string().len() as i32
}

fn is_pandigital(s: &str) -> bool {
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
