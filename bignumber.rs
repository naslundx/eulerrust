#![allow(dead_code)]

pub struct BigNumber {
    pub negative: bool,
    pub value: Vec<u8>,
}

impl BigNumber {
    pub fn from_str(data: &str) -> BigNumber {
        let negative = data.chars().next() == Some('-');
        BigNumber {
            negative,
            value: data.chars().map(|digit| digit as u8 - '0' as u8).collect(),
        }
    }

    pub fn from_number(mut data: i32) -> BigNumber {
        let negative = data < 0;
        let mut value = vec![];
        data = data.abs();
        while data > 0 {
            value.push((data % 10) as u8);
            data = data / 10;
        }
        value.reverse();
        BigNumber { negative, value }
    }

    pub fn clone(&self) -> BigNumber {
        BigNumber {
            negative: self.negative,
            value: self.value.clone(),
        }
    }

    pub fn digit_sum(&self) -> i32 {
        self.value.iter().map(|digit| *digit as i32).sum()
    }

    pub fn flip(&self) -> BigNumber {
        BigNumber {
            negative: !self.negative,
            value: self.value.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        let sign = if self.negative { "-" } else { "" };
        self.value
            .iter()
            .map(|digit| digit.to_string())
            .fold(sign.to_string(), |acc, nxt| acc + &nxt)
    }

    pub fn as_i32(&self) -> Option<i32> {
        if self.value.len() < 7 {
            let result = self
                .value
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (idx, &nxt)| {
                    acc + (nxt as i32) * 10_i32.pow(idx as u32)
                });
            Some(result)
        } else {
            None
        }
    }

    pub fn zero() -> BigNumber {
        BigNumber {
            negative: false,
            value: vec![0u8],
        }
    }

    pub fn one() -> BigNumber {
        BigNumber {
            negative: false,
            value: vec![1u8],
        }
    }

    pub fn eq(&self, other: &BigNumber) -> bool {
        self.negative == other.negative && self.value == other.value
    }

    pub fn add(&self, other: &BigNumber) -> BigNumber {
        if self.eq(&BigNumber::zero()) {
            return other.clone();
        }
        if other.eq(&BigNumber::zero()) {
            return self.clone();
        }
        // TODO handle opposite negative values

        let mut value = vec![];
        let mut a_index = self.value.len();
        let mut b_index = other.value.len();
        let mut carry = 0;

        while a_index > 0 || b_index > 0 || carry > 0 {
            let a_digit = if a_index > 0 {
                a_index -= 1;
                self.value[a_index]
            } else {
                0u8
            };

            let b_digit = if b_index > 0 {
                b_index -= 1;
                other.value[b_index]
            } else {
                0u8
            };

            let sum = a_digit + b_digit + carry;
            value.push(sum % 10);
            carry = sum / 10;
        }

        value.reverse();

        BigNumber {
            negative: self.negative,
            value,
        }
    }

    pub fn mult(&self, other: &BigNumber) -> BigNumber {
        if self.eq(&BigNumber::zero()) || other.eq(&BigNumber::zero()) {
            return BigNumber::zero();
        }
        if self.eq(&BigNumber::one()) {
            return other.clone();
        }
        if other.eq(&BigNumber::one()) {
            return self.clone();
        }

        // TODO handle opposite negative

        let mut result = vec![0; self.value.len() + other.value.len()];

        for (i, a) in self.value.iter().rev().enumerate() {
            for (j, b) in other.value.iter().rev().enumerate() {
                let sum = a * b + result[i + j];

                result[i + j] = sum % 10;
                result[i + j + 1] += sum / 10;
            }
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        let value = result.into_iter().rev().collect();

        BigNumber {
            negative: self.negative != other.negative,
            value,
        }
    }
}

/*
impl Add for BigNumber {
    type Output = BigNumber;

    fn add(self, other: BigNumber) -> BigNumber {
        BigNumber::add(&self, &other)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let num = BigNumber::from_str("12345");
        assert_eq!(num.negative, false);
        assert_eq!(num.value, vec![1, 2, 3, 4, 5]);

        let num = BigNumber::from_str("-12345");
        assert_eq!(num.negative, true);
        assert_eq!(num.value, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_from_number() {
        let num = BigNumber::from_number(12345);
        assert_eq!(num.negative, false);
        assert_eq!(num.value, vec![1, 2, 3, 4, 5]);

        let num = BigNumber::from_number(-12345);
        assert_eq!(num.negative, true);
        assert_eq!(num.value, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_clone() {
        let num = BigNumber::from_str("12345");
        let cloned = num.clone();
        assert_eq!(num.negative, cloned.negative);
        assert_eq!(num.value, cloned.value);
    }

    #[test]
    fn test_digit_sum() {
        let num = BigNumber::from_str("12345");
        assert_eq!(num.digit_sum(), 15);
    }

    #[test]
    fn test_flip() {
        let num = BigNumber::from_str("12345");
        let flipped = num.flip();
        assert_eq!(flipped.negative, true);
        assert_eq!(flipped.value, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_to_string() {
        let num = BigNumber::from_str("12345");
        assert_eq!(num.to_string(), "12345");

        let num = BigNumber::from_str("-12345");
        assert_eq!(num.to_string(), "-12345");
    }

    #[test]
    fn test_as_i32() {
        let num = BigNumber::from_str("12345");
        assert_eq!(num.as_i32(), Some(12345));

        let num = BigNumber::from_str("12345678");
        assert_eq!(num.as_i32(), None);
    }

    #[test]
    fn test_zero() {
        let num = BigNumber::zero();
        assert_eq!(num.negative, false);
        assert_eq!(num.value, vec![0]);
    }

    #[test]
    fn test_one() {
        let num = BigNumber::one();
        assert_eq!(num.negative, false);
        assert_eq!(num.value, vec![1]);
    }

    #[test]
    fn test_eq() {
        let num1 = BigNumber::from_str("12345");
        let num2 = BigNumber::from_str("12345");
        assert!(num1.eq(&num2));

        let num3 = BigNumber::from_str("-12345");
        assert!(!num1.eq(&num3));
    }

    #[test]
    fn test_add() {
        let num1 = BigNumber::from_str("123");
        let num2 = BigNumber::from_str("456");
        let result = num1.add(&num2);
        assert_eq!(result.to_string(), "579");

        let num1 = BigNumber::from_str("-123");
        let num2 = BigNumber::from_str("456");
        let result = num1.add(&num2);
        assert_eq!(result.to_string(), "333"); // This will fail until negative handling is implemented
    }

    #[test]
    fn test_mult() {
        let num1 = BigNumber::from_str("123");
        let num2 = BigNumber::from_str("456");
        let result = num1.mult(&num2);
        assert_eq!(result.to_string(), "56088");

        let num1 = BigNumber::from_str("-123");
        let num2 = BigNumber::from_str("456");
        let result = num1.mult(&num2);
        assert_eq!(result.to_string(), "-56088");
    }
}
