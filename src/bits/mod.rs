#![feature(trait_alias)]
extern crate num;

use core::mem;
use num::NumCast;
use self::num::PrimInt;

/// ```
/// let result = stegno::bits::bitify_message("A");
/// assert_eq!(result, vec![false, true, false, false, false, false, false, true]);
///
/// let result = stegno::bits::bitify_message("Avshyz");
/// assert_eq!(result, vec![false, true, false, false, false, false, false, true,
///                         false, true, true, true, false, true, true, false,
///                         false, true, true, true, false, false, true, true,
///                         false, true, true, false, true, false, false, false,
///                         false, true, true, true, true, false, false, true,
///                         false, true, true, true, true, false, true, false]);
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
/// assert_eq!(stegno::bits::get_bit_at(7, 0).unwrap(), true);
/// assert_eq!(stegno::bits::get_bit_at(7, 1).unwrap(), true);
/// assert_eq!(stegno::bits::get_bit_at(7, 2).unwrap(), true);
/// assert_eq!(stegno::bits::get_bit_at(7, 3).unwrap(), false);
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
/// assert_eq!(stegno::bits::set_bit_at(7, 0, false).unwrap(), 6);
/// assert_eq!(stegno::bits::set_bit_at(7, 1, false).unwrap(), 5);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, false).unwrap(), 3);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, true).unwrap(), 7);
/// assert_eq!(stegno::bits::set_bit_at(7, 3, true).unwrap(), 15);
///
/// assert_eq!(stegno::bits::set_bit_at(255, 0, false).unwrap(), 254);
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