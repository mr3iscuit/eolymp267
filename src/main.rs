use std::io;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
struct BigNumber {
    digits: Vec<u32>,
    sign: Sign,
}

#[inline]
fn swap_sign_with_other<'a>(mut a: &'a mut Sign, mut b: &'a mut Sign) {
    let temp_sign = a;
    a = b;
    b = temp_sign;
}

impl BigNumber {
    fn make_abs(&mut self) {
        self.sign = Sign::Positive;
    }

    fn swap_digits(&mut self, other: &mut BigNumber) {
        std::mem::swap(&mut self.digits, &mut other.digits);
    }

    fn from_string(input: &str) -> Self {
        let (sign, number_str) = match input.chars().next() {
            Some('-') => (Sign::Negative, &input[1..]),
            _ => (Sign::Positive, input),
        };

        let digits: Vec<u32> = number_str
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        BigNumber { digits, sign }
    }

    fn is_greater_than(&self, other: &BigNumber) -> bool {
        if self.digits.len() > other.digits.len() {
            return true;
        } else if self.digits.len() < other.digits.len() {
            return false;
        }

        for (&self_digit, &other_digit) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if self_digit > other_digit {
                return true;
            } else if self_digit < other_digit {
                return false;
            }
        }

        false
    }

    fn add(&mut self, other: &mut BigNumber) {
        let self_is_greater = self.is_greater_than(other);
        if !self_is_greater {
            self.swap_digits(other);
            swap_sign_with_other(&mut self.sign, &mut other.sign);
        }

        if self_is_greater {
            println!("self is greater");
        } else {
            println!("other is greater");
        }

        if self.sign == Sign::Negative && 
            other.sign == Sign::Positive
        {
            self._subtract(other);
            return;
        }
        self._add(other);
    }

    fn subtract(&mut self, other: &mut BigNumber) {
        let self_is_greater = self.is_greater_than(other);
        if !self_is_greater {
            self.swap_digits(other);
            swap_sign_with_other(&mut self.sign, &mut other.sign);
        }

        if self.sign == Sign::Positive && 
            other.sign == Sign::Positive 
        {
            self._subtract(other);
            return;
        }

        self._add(other);
    }

    fn _subtract(&mut self, other: &BigNumber) {
        let mut carry = 0;
        for i in 0..self.digits.len() {
            let other_digit = if i < other.digits.len() { other.digits[i] } else { 0 };
            let mut diff :i32 = self.digits[i] as i32 - other_digit as i32 - carry;
            if diff < 0 {
                diff += 10;
                carry = 1;
            } else {
                carry = 0;
            }
            self.digits[i] = diff as u32;
        }
        self.normalize();
    }


    fn _add(&mut self, other: &BigNumber) {
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        for i in 0..max_len {
            let self_digit = if i < self.digits.len() { self.digits[i] } else { 0 };
            let other_digit = if i < other.digits.len() { other.digits[i] } else { 0 };
            let sum = self_digit + other_digit + carry;
            self.digits.resize(i + 1, 0);
            self.digits[i] = sum % 10;
            carry = sum / 10;
        }

        if carry > 0 {
            self.digits.push(carry);
        }
    }

    /*
    BigInteger operator+(const BigInteger& b)
    {
        string a = str;
        string c = b.str;
        int alen = a.length(), clen = c.length();
        int n = max(alen, clen);
        if (alen > clen)
            c.insert(0, alen - clen, '0');
        else if (alen < clen)
            a.insert(0, clen - alen, '0');
        string res(n + 1, '0');
        int carry = 0;
        for (int i = n - 1; i >= 0; i--) {
            int digit = (a[i] - '0') + (c[i] - '0') + carry;
            carry = digit / 10;
            res[i + 1] = digit % 10 + '0';
        }
        if (carry == 1) {
            res[0] = '1';
            return BigInteger(res);
        }
        else {
            return BigInteger(res.substr(1));
        }
    }
    */

    fn normalize(&mut self) {
        while let Some(&digit) = self.digits.last() {
            if digit == 0 {
                self.digits.pop();
            } else {
                break;
            }
        }
        if self.digits.len() == 0 {
            self.digits.push(0);
        }
    }

    fn print(&self) {
        if self.digits.is_empty() {
            println!("0");
        } else {
            if self.sign == Sign::Negative {
                print!("-");
            }
            for &digit in self.digits.iter().rev() {
                print!("{}", digit);
            }
            println!();
        }
    }
}

fn main() {
    // Read input strings
    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");
    let first_string = first_line.trim();

    // Read the second string
    let mut second_line = String::new();
    io::stdin()
        .read_line(&mut second_line)
        .expect("Failed to read line");
    let second_string = second_line.trim();

    // Create BigNumber instances from input strings
    let num1 = BigNumber::from_string(&first_string);
    let mut num2 = BigNumber::from_string(&second_string);

    let mut result = num1.clone();
    result.subtract(&mut num2);
    result.make_abs();

    result.print();
}
