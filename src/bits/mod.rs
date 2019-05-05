use std::any::Any;
use core::mem;

type Byte = u8;

/// Converts a string to a list of bits
/// ```
/// let result = stegno::bits::bitify_message("A");
/// assert_eq!(result, vec![0, 1, 0, 0, 0, 0, 0, 1]);
/// ```
pub fn bitify_message(cipher: &str) -> Vec<Byte> {
    let mut res = vec![];
    for char in cipher.as_bytes() {
        let curr: Vec<Byte> = (0..mem::size_of_val(&char))
            .rev()
            .map(|i| get_bit_at(*char, i))
            .collect();
        res.extend(curr);
    }
    res
}

/// ```
/// assert_eq!(stegno::bits::get_bit_at(7, 0), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 1), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 2), 1);
/// assert_eq!(stegno::bits::get_bit_at(7, 3), 0);
/// ```
pub fn get_bit_at(input: Byte, n: usize) -> Byte {
    println!("{} {}", input, n);
    (input & (1 << n)) >> n
}


/// ```
/// assert_eq!(stegno::bits::set_bit_at(7, 0, 0), 6);
/// assert_eq!(stegno::bits::set_bit_at(7, 1, 0), 5);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, 0), 3);
/// assert_eq!(stegno::bits::set_bit_at(7, 2, 1), 7);
/// assert_eq!(stegno::bits::set_bit_at(7, 3, 1), 15);
/// ```
pub fn set_bit_at(input: Byte, n: usize, val: Byte) -> Byte {
    if val == 1 {
        input | (1 as Byte).rotate_left(n as u32)
    } else {
        input & (!1 as Byte).rotate_left(n as u32)
    }
}