pub struct BigNumber {
    pub number: String,
}

impl BigNumber {
    pub fn zero() -> BigNumber {
        BigNumber { number: "0".to_string() }
    }
    
    pub fn one() -> BigNumber {
        BigNumber { number: "1".to_string() }
    }

    pub fn add(&self, other: &BigNumber) -> BigNumber {
        let a = &self.number;
        let b = &other.number;

        if a == "0" {
            return BigNumber { number: b.to_string() };
        }
        if b == "0" {
            return BigNumber { number: a.to_string() };
        }
    
        let mut carry = 0;
        let mut a_index: usize = a.len() - 1;
        let mut b_index: usize = b.len() - 1;
        let mut c = Vec::new();
        let mut overflown = 0;
    
        while overflown < 2 {
            let a_digit = a.chars().nth(a_index).map_or(0, |ch| ch as i32 - '0' as i32);
            let b_digit = b.chars().nth(b_index).map_or(0, |ch| ch as i32 - '0' as i32);
            let c_digit = a_digit + b_digit + carry;
    
            c.push(c_digit % 10);
            carry = c_digit / 10;
    
            if a_index > 0 {
                a_index -= 1;
            } else {
                a_index = 9999;
                overflown += 1;
            }
            if b_index > 0 {
                b_index -= 1;
            }
            else {
                b_index = 9999;
                overflown += 1;
            }
        }
    
        if carry != 0 {
            c.push(carry);
        }
    
        let number = c.iter().rev()
            .map(|s| s.to_string())
            .reduce(|cur: String, nxt: String| cur + &nxt)
            .unwrap();
    
        BigNumber { number }
    }
}

fn main() {
    let mut idx = 1;
    let mut a = BigNumber::zero();
    let mut b = BigNumber::one();

    loop {
        idx += 1;

        let c = a.add(&b);
        println!("idx={idx}, len={}", c.number.len());

        a = b;
        b = BigNumber { number: c.number };

        if b.number.len() > 1000 {
            break;
        }
    }
}