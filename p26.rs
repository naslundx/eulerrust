fn main() {
    let mut best_i = 0;
    let mut best_length = 0;
    for i in 2..=1000 {
        let length = division_length(i);

        if length > best_length {
            best_i = i;
            best_length = length;
        }
    }

    println!("{}: {}", best_i, best_length);
}

fn division_length(a: u32) -> usize {
    let mut fraction = vec![];

    let mut n = 10;

    loop {
        let q = n / a;
        let r = n % a;
        let data = (q, r, n);
        if fraction.contains(&data) {
            break;
        }
        fraction.push(data);
        n = r * 10;
        if n == 0 {
            break;
        }

        //println!("q={q}, r={r}, n={n}");
        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    fraction.len()
}
