use std::{collections::VecDeque, vec};

pub type Bit = u8;

pub fn explode_bitstring(str: &str) -> Vec<Bit> {
    str.as_bytes()
        .iter()
        .filter(|ch| **ch == b'0' || **ch == b'1')
        .map(|ch| if *ch == b'0' { 0 } else { 1 })
        .collect::<Vec<Bit>>()
}

pub trait StringBit {
    fn bits(&self) -> String;
}

impl StringBit for Vec<Bit> {
    fn bits(&self) -> String {
        self.clone()
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|bit| bit.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl StringBit for VecDeque<Bit> {
    fn bits(&self) -> String {
        Vec::from(self.clone()).bits()
    }
}

pub trait ToInteger {
    fn to_u8(&self) -> u8;
    fn to_u64(&self) -> u64;
}

impl ToInteger for Vec<Bit> {
    fn to_u8(&self) -> u8 {
        let mut reversed = Vec::from(self.clone());
        reversed.reverse();

        let mut n = 0u8;
        for (i, bit) in reversed.iter().enumerate() {
            let p = 2_u8.pow(i as u32) * (*bit as u8);
            n += p;
        }

        n
    }

    fn to_u64(&self) -> u64 {
        let mut reversed = Vec::from(self.clone());
        reversed.reverse();

        let mut n = 0u64;
        for (i, bit) in reversed.iter().enumerate() {
            let p = 2_u64.pow(i as u32) * (*bit as u64);
            n += p;
        }

        n
    }
}

pub trait ToBitVec {
    fn to_bit_vec(&self) -> Vec<Bit>;
}

impl ToBitVec for u8 {
    fn to_bit_vec(&self) -> Vec<Bit> {
        let mut bits: Vec<Bit> = vec![];

        for i in 0..8 {
            if (self >> i) & 0b1 > 0 {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        bits.reverse();

        bits
    }
}

impl ToBitVec for u64 {
    fn to_bit_vec(&self) -> Vec<Bit> {
        let mut bits: Vec<Bit> = vec![];

        for i in 0..64 {
            if (self >> i) & 0b1 > 0 {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        bits.reverse();

        bits
    }
}

pub trait ShiftLeft {
    fn shift_left(&mut self, rhs: u32, wrap_around: bool);
}

impl ShiftLeft for VecDeque<Bit> {
    fn shift_left(&mut self, rhs: u32, wrap_around: bool) {
        for _ in 0..rhs {
            let bit = self.pop_front().unwrap();

            if wrap_around {
                self.push_back(bit);
            } else {
                self.push_back(0);
            }
        }
    }
}

pub trait Bitwise {
    fn xor(&self, rhs: &Vec<Bit>) -> Vec<Bit>;
    fn and(&self, rhs: &Vec<Bit>) -> Vec<Bit>;
}

impl Bitwise for Vec<Bit> {
    fn xor(&self, rhs: &Vec<Bit>) -> Vec<Bit> {
        assert_eq!(
            self.len(),
            rhs.len(),
            "xor entre Vec<Bit> de tamanhos distintos"
        );

        let mut xored: Vec<Bit> = vec![];
        for i in 0..self.len() {
            let a = self[i];
            let b = rhs[i];

            xored.push((a + b) & 0b1); // tome o xor seu fdp
        }

        xored
    }

    fn and(&self, rhs: &Vec<Bit>) -> Vec<Bit> {
        assert_eq!(
            self.len(),
            rhs.len(),
            "and entre Vec<Bit> de tamanhos distintos"
        );

        let mut anded: Vec<Bit> = vec![];
        for i in 0..self.len() {
            let a = self[i];
            let b = rhs[i];

            anded.push(a & b);
        }

        anded
    }
}
