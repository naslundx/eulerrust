pub fn problem26() -> i64 {
    let mut best_d = 0;
    let mut best_length = 0;

    for d in 2..=1000 {
        let length = division_length(d);

        if length > best_length {
            (best_d, best_length) = (d, length);
        }
    }

    best_d as i64
}

fn division_length(d: u32) -> usize {
    let mut fraction = vec![];
    let mut n = 10;

    while n != 0 {
        let q = n / d;
        let r = n % d;
        let data = (q, r);
        if fraction.contains(&data) {
            break;
        }
        fraction.push(data);
        n = r * 10;
    }

    fraction.len()
}
