#![feature(trait_alias)]
extern crate num;
extern crate itertools;

use core::mem;
use num::NumCast;
use self::num::PrimInt;
use std::string::FromUtf8Error;
use itertools::Itertools;


pub fn bitify_message(cipher: &str) -> Vec<bool> {
    let mut res = vec![];
    for char in cipher.as_bytes() {
        let curr: Vec<bool> = (0..mem::size_of_val(&char))
            .rev()
            .map(|i| get_bit_at(*char, i as u32).unwrap())
            .collect();
        res.extend(curr);
    }
    res
}

pub fn unbitify_message(bits: Vec<bool>) -> Result<String, FromUtf8Error> {
    String::from_utf8(
        bits.iter()
            .chunks(8)
            .into_iter()
            .map(|split_byte| {
                split_byte.fold(0, |res, &bit| (res << 1) | (bit as u8))
            })
            .collect()
    )
}

pub fn get_bit_at<T: PrimInt>(input: T, n: u32) -> Result<bool, String> {
    let mask = T::one() << NumCast::from(n).unwrap();
    if (n as usize) < mem::size_of_val(&input) * 8 {
        Ok(input & mask != T::zero())
    } else {
        Err(String::from("Accessing bit that doesn't exist"))
    }
}

pub fn set_bit_at<T: PrimInt>(input: T, n: u32, val: bool) -> Result<T, String> {
    if (n as usize) < mem::size_of_val(&input) * 8 {
        if val {
            Ok(input | T::one().rotate_left(n))
        } else {
            Ok(input & !T::one().rotate_left(n))
        }
    } else {
        Err(String::from("Accessing bit that doesn't exist"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_set_bit_at() {
        assert_eq!(6, set_bit_at(7, 0, false).unwrap());
        assert_eq!(5, set_bit_at(7, 1, false).unwrap());
        assert_eq!(3, set_bit_at(7, 2, false).unwrap());
        assert_eq!(7, set_bit_at(7, 2, true).unwrap());
        assert_eq!(15, set_bit_at(7, 3, true).unwrap());
        assert_eq!(254, set_bit_at(255, 0, false).unwrap());
    }

    #[test]
    pub fn test_get_bit_at() {
        assert_eq!(true, get_bit_at(7, 0).unwrap());
        assert_eq!(true, get_bit_at(7, 1).unwrap());
        assert_eq!(true, get_bit_at(7, 2).unwrap());
        assert_eq!(false, get_bit_at(7, 3).unwrap());
    }

    #[test]
    pub fn test_unbitify_message() {
        assert_eq!("A", unbitify_message(vec![false, true, false, false, false, false, false, true]).unwrap());
        let bitified = vec![false, true, false, false, false, false, false, true,
                            false, true, true, true, false, true, true, false,
                            false, true, true, true, false, false, true, true,
                            false, true, true, false, true, false, false, false,
                            false, true, true, true, true, false, false, true,
                            false, true, true, true, true, false, true, false];
        assert_eq!("Avshyz", unbitify_message(bitified).unwrap());
    }

    #[test]
    pub fn test_bitify_message() {
        let result = bitify_message("A");
        assert_eq!(vec![false, true, false, false, false, false, false, true], result);

        let result = bitify_message("Avshyz");
        assert_eq!(vec![false, true, false, false, false, false, false, true,
                        false, true, true, true, false, true, true, false,
                        false, true, true, true, false, false, true, true,
                        false, true, true, false, true, false, false, false,
                        false, true, true, true, true, false, false, true,
                        false, true, true, true, true, false, true, false], result);
    }
}