use std::collections::HashSet;

fn main() {
    let mut primes = HashSet::new();
    let n = 11000; // You can change this value to any number you want
    for num in 2..=n {
        let mut is_prime = true;
        for i in 2..=((num as f64).sqrt() as u32) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.insert(num);
        }
    }

    for p in (1001..9999).step_by(2) {
        if !primes.contains(&p) {
            continue;
        }
        for d in (2..3500).rev() {
            let p_1 = p + d;
            let p_2 = p + 2 * d;

            if p_2 > 9999 {
                break;
            }

            let are_perms = is_perm(p, p_1) && is_perm(p, p_2);

            if are_perms && primes.contains(&p_1) && primes.contains(&p_2) {
                println!("{p}{p_1}{p_2}");
                break;
            }
        }
    }
}

fn is_perm(m: u32, n: u32) -> bool {
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
