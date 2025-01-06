use std::collections::HashSet;
use util::get_primes;

pub fn problem49() -> i64 {
    let mut primes = HashSet::new();
    for p in get_primes(10000) {
        primes.insert(p);
    }

    for p in primes.iter() {
        for d in (2..3500).rev() {
            let p_1 = *p + d;
            let p_2 = *p + 2 * d;

            if p_2 > 9999 {
                break;
            }
            if !primes.contains(&p_1) || !primes.contains(&p_2) {
                continue;
            }
            if is_perm(*p, p_1) && is_perm(*p, p_2) && *p != 1487 {
                return *p * 100000000 + p_1 * 10000 + p_2;
            }
        }
    }

    0
}

fn is_perm(m: i64, n: i64) -> bool {
    let mut m_chars = m.to_string().chars().collect::<Vec<char>>();
    m_chars.sort();
    let m_str = m_chars
        .iter()
        .fold(String::new(), |acc, c| acc + &c.to_string());

    let mut n_chars = n.to_string().chars().collect::<Vec<char>>();
    n_chars.sort();
    let n_str = n_chars
        .iter()
        .fold(String::new(), |acc, c| acc + &c.to_string());

    m_str == n_str
}
