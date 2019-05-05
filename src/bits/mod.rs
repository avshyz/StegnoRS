/// Converts a string to a list of bits
/// ```
/// let result = stegno::bits::bitify_message("A");
/// assert_eq!(result, vec![0, 1, 0, 0, 0, 0, 0, 1]);
/// ```
pub fn bitify_message(cipher: &str) -> Vec<u8> {
    let mut res = vec![];
    for char in cipher.as_bytes() {
        let curr: Vec<u8> = (0..8).rev().map(|i| get_bit_at(*char, i)).collect();
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
pub fn get_bit_at(input: u8, n: u8) -> u8 {
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
pub fn set_bit_at(input: u8, n: u32, val: u8) -> u8 {
    if val == 1 {
        input | (1 as u8).rotate_left(n)
    } else {
        input & (!1 as u8).rotate_left(n)
    }
}