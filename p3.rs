fn main() {
    let mut n = 600851475143;

    //let highest_possible_factor = (n as f64).sqrt().ceil() as usize;
    let mut factor = 2;

    loop {
        if n == factor {
            println!("{factor}");
            break;
        }

        while n % factor == 0 {
            println!("Reducing by {factor}, new value {}", n / factor);
            n = n / factor;
        }

        factor += 1;
    }

    return;

    for p in (2..=((n as f64).sqrt().ceil() as usize)).rev() {
        // println!("{p}");
        if n % p == 0 {
            let mut candidate = true;

            for q in 2..p {
                if p % q == 0 {
                    // println!("{p} is not prime, divisible by {q}");
                    candidate = false;
                    break;
                }
            }

            if candidate {
                println!("{p} is prime");
                break;
            }
        }
    }
}
