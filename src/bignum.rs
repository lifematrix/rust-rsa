#[derive(Debug, Clone)]
pub struct BigNum {
    pub sign: i8,
    pub digits: Vec<u32>,
}

impl BigNum {
    pub fn new() -> Self {
        BigNum {
            sign: 1,
            digits: vec![0],
        }
    }

    pub fn from_u32(value: u32) -> Self{
        BigNum {
            sign: 1,
            digits: vec![value],
        }

    }

    pub fn from_i32(value: i32) -> Self{
        BigNum {
            sign: if value >= 0 { 1 } else { -1 },
            digits: vec![value.abs() as u32],
        }
    }

    pub fn from_u64(value: u64) -> Self{
        let low = (value & 0xFFFF_FFFF) as u32;
        let high = (value >> 32) as u32;

        let digits = if high != 0 {
            vec![low, high]
        } else { 
            vec![low]
        };

        BigNum {
            sign: 1,
            digits: digits, 
        }
    }

    // pub fn from_i64(value: i64) -> Self{
    //     let mut bgn = from_u64(value.abs() as u64)
    //     bgn.sign = if value >= 0 { 1 } else { -1 } as i8;

    //     bgn;
    // }

    pub fn from_i64(value: i64) -> Self{
        let sign = if value >= 0 { 1 } else { -1 } as i8;
        let digits = Self::from_u64(value.abs() as u64).digits;

        BigNum{
            sign,
            digits,
        }
    }
}

