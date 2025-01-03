fn main() {
    let mut sum = 0;
    for i in 1..1000000 {
        let ib = to_binary(i);
        if is_palindrome(i) && is_palindrome(ib) {
            println!("{}, {}", i, ib);
            sum += i;
        }
    }
    println!("{sum}");
}

fn to_binary(n: i128) -> i128 {
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
        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    digits.iter().fold(0i128, |acc, nxt| acc * 10 + nxt)
}

fn is_palindrome(n: i128) -> bool {
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
