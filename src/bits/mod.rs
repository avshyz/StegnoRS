#![feature(trait_alias)]
extern crate num;

use core::mem;
use num::NumCast;
use self::num::PrimInt;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// ```
/// let result = stegno::bits::bitify_message("A");
/// assert_eq!(vec![false, true, false, false, false, false, false, true], result);
///
/// let result = stegno::bits::bitify_message("Avshyz");
/// assert_eq!(vec![false, true, false, false, false, false, false, true,
///                 false, true, true, true, false, true, true, false,
///                 false, true, true, true, false, false, true, true,
///                 false, true, true, false, true, false, false, false,
///                 false, true, true, true, true, false, false, true,
///                 false, true, true, true, true, false, true, false], result);
/// ```
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

/// ```
/// assert_eq!("A", stegno::bits::unbitify_message(vec![false, true, false, false, false, false, false, true]).unwrap());
///
/// let bitified = vec![false, true, false, false, false, false, false, true,
///                     false, true, true, true, false, true, true, false,
///                     false, true, true, true, false, false, true, true,
///                     false, true, true, false, true, false, false, false,
///                     false, true, true, true, true, false, false, true,
///                     false, true, true, true, true, false, true, false];
/// assert_eq!("Avshyz", stegno::bits::unbitify_message(bitified).unwrap());
/// ```
pub fn unbitify_message(bits: Vec<bool>) -> Result<String, FromUtf8Error> {
    let mut res: Vec<u8> = vec![];
    let mut curr = 0;

    for (idx, &bit) in bits.iter().enumerate() {
        let x = (idx % 8) as u8;
        if idx % 8 != 0 || idx == 0 {
            curr = set_bit_at(curr, (7 - (idx % 8)) as u32, bit).unwrap();
        } else {
            res.push(curr);
            curr = 0
        }
    }
    res.push(curr);
    String::from_utf8(res)
}

/// ```
/// assert_eq!(true, stegno::bits::get_bit_at(7, 0).unwrap());
/// assert_eq!(true, stegno::bits::get_bit_at(7, 1).unwrap());
/// assert_eq!(true, stegno::bits::get_bit_at(7, 2).unwrap());
/// assert_eq!(false, stegno::bits::get_bit_at(7, 3).unwrap());
/// ```
pub fn get_bit_at<T: PrimInt>(input: T, n: u32) -> Result<bool, String> {
    let mask = T::one() << NumCast::from(n).unwrap();
    if (n as usize) < mem::size_of_val(&input) * 8 {
        Ok(input & mask != T::zero())
    } else {
        Err(String::from("Accessing bit that doesn't exist"))
    }
}


/// ```
/// assert_eq!(6, stegno::bits::set_bit_at(7, 0, false).unwrap());
/// assert_eq!(5, stegno::bits::set_bit_at(7, 1, false).unwrap());
/// assert_eq!(3, stegno::bits::set_bit_at(7, 2, false).unwrap());
/// assert_eq!(7, stegno::bits::set_bit_at(7, 2, true).unwrap());
/// assert_eq!(15, stegno::bits::set_bit_at(7, 3, true).unwrap());
/// assert_eq!(254, stegno::bits::set_bit_at(255, 0, false).unwrap());
/// ```
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