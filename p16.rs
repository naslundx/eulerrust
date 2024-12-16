fn main() {
    let mut number = BigNumber { number: "1".to_string() };

    for power in 1..=1000 {
        println!("power={power}");
        number = number.mult(2);
        println!("{}", number.number);
    }

    println!("{}", number.digit_sum());
}

pub struct BigNumber {
    pub number: String,
}

impl BigNumber {
    pub fn zero() -> BigNumber {
        BigNumber { number: "0".to_string() }
    }

    pub fn digit_sum(&self) -> i32 {
        self.number.chars().map(|s| s as i32 - '0' as i32).sum()
    }

    pub fn mult(&self, other: i32) -> BigNumber {
        let a = &self.number;
        let mut a_index: usize = a.len();
        let mut carry = 0;
        let mut result = Vec::new();

        while a_index != 0 {
            a_index -= 1;

            let a_digit = a.chars().nth(a_index).map_or(0, |ch| ch as i32 - '0' as i32);
            let digit_product = a_digit * other + carry;
            result.push(digit_product % 10);
            carry = digit_product / 10;
        }

        if carry != 0 {
            result.push(carry);
        }

        let number = result.iter().rev()
            .map(|s| s.to_string())
            .reduce(|cur: String, nxt: String| cur + &nxt)
            .unwrap();
    
        BigNumber { number }
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