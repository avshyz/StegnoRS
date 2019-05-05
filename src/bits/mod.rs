use core::mem;
use std::fmt::Error;

pub type PseudoByte = u32;

/// ```
/// let result = stegno::bits::bitify_message("A");
/// assert_eq!(result, vec![0, 1, 0, 0, 0, 0, 0, 1]);
///
/// let result = stegno::bits::bitify_message("Avshyz");
/// assert_eq!(result, vec![0, 1, 0, 0, 0, 0, 0, 1,
///                         0, 1, 1, 1, 0, 1, 1, 0,
///                         0, 1, 1, 1, 0, 0, 1, 1,
///                         0, 1, 1, 0, 1, 0, 0, 0,
///                         0, 1, 1, 1, 1, 0, 0, 1,
///                         0, 1, 1, 1, 1, 0, 1, 0]);
/// ```
pub fn bitify_message(cipher: &str) -> Vec<PseudoByte> {
    let mut res = vec![];
    for char in cipher.as_bytes() {
        let curr: Vec<PseudoByte> = (0..mem::size_of_val(&char))
            .rev()
            .map(|i| get_bit_at(*char as u32, i).unwrap())
            .collect();
        res.extend(curr);
    }
    res
}

/// ```
/// assert_eq!(stegno::bits::get_bit_at(7, 0).unwrap(), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 1).unwrap(), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 2).unwrap(), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 3).unwrap(), 0);
/// ```
pub fn get_bit_at(input: PseudoByte, n: usize) -> Result<PseudoByte, String> {
    if n < mem::size_of_val(&input) * 8 {
        Ok((input & (1 << n)) >> n)
    } else {
        Err(String::from("Accessing bit that doesn't exist"))
    }
}


/// ```
/// assert_eq!(stegno::bits::set_bit_at(7, 0, 0).unwrap(), 6);
/// assert_eq!(stegno::bits::set_bit_at(7, 1, 0).unwrap(), 5);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, 0).unwrap(), 3);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, 1).unwrap(), 7);
/// assert_eq!(stegno::bits::set_bit_at(7, 3, 1).unwrap(), 15);
///
/// assert_eq!(stegno::bits::set_bit_at(255, 0, 0).unwrap(), 254);
/// ```
pub fn set_bit_at(input: PseudoByte, n: usize, val: PseudoByte) -> Result<PseudoByte, String> {
    if n < mem::size_of_val(&input) * 8 {
        if val == 1 {
            Ok(input | (1 as PseudoByte).rotate_left(n as u32))
        } else {
            Ok(input & (!1 as PseudoByte).rotate_left(n as u32))
        }
    } else {
        Err(String::from("Accessing bit that doesn't exist"))
    }
}