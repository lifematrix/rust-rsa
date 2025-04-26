use std::ops::{Add, Neg, Sub};
use std::cmp::{min, max, Ordering};


#[derive(Debug, Clone)]
pub struct BigNum {
    pub sign: i8,
    pub digits: Vec<u32>,
}

impl BigNum {
    pub fn new() -> Self {
        Self {
            sign: 1,
            digits: vec![0],
        }
    }

    pub fn from_u32(value: u32) -> Self{
        Self {
            sign: 1,
            digits: vec![value],
        }

    }

    pub fn from_i32(value: i32) -> Self{
        Self {
            sign: if value >= 0 { 1 } else { -1 },
            digits: vec![value.abs() as u32],
        }
    }

    #[inline]
    pub(crate) fn split_u64(value: u64) -> (u32, u32) {
        (
            (value & 0xFFFF_FFFF) as u32,
            (value >> 32) as u32,
        )
    }

    pub fn from_u64(value: u64) -> Self{
        let low = (value & 0xFFFF_FFFF) as u32;
        let high = (value >> 32) as u32;

        let digits = if high != 0 {
            vec![low, high]
        } else { 
            vec![low]
        };

        Self {
            sign: 1,
            digits: digits, 
        }
    }

    pub fn from_i64(value: i64) -> Self{
        let sign = if value >= 0 { 1 } else { -1 } as i8;
        let digits = Self::from_u64(value.abs() as u64).digits;

        Self {
            sign,
            digits,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    pub fn abs(&self) -> Self {
        if self.sign > 0 {
            self.clone()
        }
        else {
            Self {
                sign: 1,
                digits: self.digits.clone(),
            }
        }
    }

    pub fn abs_cmp(&self, other: &Self) -> Ordering {
        Self::cmp_digits(&self.digits, &other.digits)
    }

    pub(crate) fn cmp_digits(a: &Vec<u32>, b: &Vec<u32>) -> Ordering {
        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        }

        for i in (0..a.len()).rev() {
            let ord = a[i].cmp(&b[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }

    pub fn add_digits(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
        let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);

        let (min_len, longer) = if a.len() > b.len() {
            (b.len(), a)
        } else {
            (a.len(), b)
        };

        let mut carry: u32= 0;
        for i in 0..min_len {
            let x: u64 = a[i] as u64 + b[i] as u64 + carry as u64;
            let (low, high) = Self::split_u64(x);          
            result.push(low);
            carry = high;
        }

        for i in min_len..longer.len() {
            if carry != 0 {
                let x: u64 = longer[i] as u64 + carry as u64;
                let (low, high) = Self::split_u64(x);
                result.push(low);
                carry = high;    
            }
            else {
                result.push(longer[i]);
            }
        }

        if carry != 0 {
            result.push(carry);
        }
        
        result
    }


    pub fn sub_digits(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
        debug_assert!(
            Self::cmp_digits(a, b) != Ordering::Less,
            "sub_digits: first operand must be >= the second one! a = {:?}, b = {:?}", a, b
        );

        let mut result = Vec::with_capacity(a.len());

        let mut borrow: u32 = 0;
        let mut minuend: u64 = 0;
        let mut subtrahend: u64 = 0;
        for i in 0..b.len() {
            minuend = a[i] as u64;
            subtrahend = b[i] as u64 + borrow as u64;
            if minuend >= subtrahend {
                borrow = 0;
            }
            else {
                minuend += 1<<32;
                borrow = 1;
            }
            let difference = minuend - subtrahend;
            result.push(difference as u32);
        }

        for i in b.len()..a.len() {
            if borrow != 0 {
                minuend = a[i] as u64;
                subtrahend = borrow as u64;

                if minuend >= subtrahend {
                    borrow = 0;
                }
                else {
                    minuend += 1<<32;
                    borrow = 1;
                }
                let difference = minuend - subtrahend;
                result.push(difference as u32);
            }
            else {
                result.push(a[i]);
            }
        }

        let mut num_trailing_zero: usize = 0;
        for i in (0..result.len()).rev() {
            if result[i] == 0 {
                num_trailing_zero += 1;
            }
            else {
                break;
            }
        }
        // for x in &result.into_iter().rev() {
        //     if x == 0 {
        //         num_trailing_zero += 1;
        //     }
        //     else {
        //         break;
        //     }
        // }

        println!("after search trailing zeor, result: {:?}, num_trailing_zeor: {:?}", result, num_trailing_zero);

        if num_trailing_zero > 0 {
            let trimmed_len = max(result.len() - num_trailing_zero, 1);
            println!("trimmed_len: {:?}", trimmed_len);
            result.truncate(trimmed_len);
        }

        result
    }
}


// // Implement add operations

impl Add for BigNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (sign, digits) = if self.sign == rhs.sign {
            let digits = Self::add_digits(&self.digits, &rhs.digits);
            (self.sign, digits)
        }
        else {
            match Self::cmp_digits(&self.digits, &rhs.digits) {
                Ordering::Greater => {
                    let digits = Self::sub_digits(&self.digits, &rhs.digits);
                    (self.sign, digits)
                }
                Ordering::Less => {
                    let digits = Self::sub_digits(&rhs.digits, &self.digits);
                    (rhs.sign, digits)
                }
                Ordering::Equal => {
                    (1, vec![0 as u32])
                }
            }
        };
            
        Self {
            sign,
            digits,
        }
    }
}

impl Neg for BigNum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            self
        }
        else {
            Self {
                sign: -self.sign,
                digits: self.digits,
            }
        }
    }
}

impl Sub for BigNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}