extern crate rand;
extern crate itertools;

use std::fs::File;
use std::io::{Read, Write};
use rand::Rng;

use crate::bits::{bitify_message, set_bit_at, unbitify_message};
use self::itertools::{Itertools, EitherOrBoth, Chunk, Chunks};
use std::string::FromUtf8Error;


pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write(path: &str, data: &Vec<u8>) -> Result<(), std::io::Error> {
    let mut f = File::create(path)?;
    f.write_all(data.as_slice())?;
    Ok(())
}

pub fn encrypt(img_data_bytes: &Vec<u8>, plain: &str) -> Result<Vec<u8>, String> {
    let terminated_plain : String = String::from(plain) + "\n";
    let cipher_bits = bitify_message(terminated_plain.as_str());
    let items = img_data_bytes.iter()
        .zip_longest(cipher_bits)
        .map(|x| match x {
            EitherOrBoth::Both(&img_byte, cipher_bit) => set_bit_at(img_byte, 0, cipher_bit),
            EitherOrBoth::Left(&img_byte) => Ok(img_byte),
            EitherOrBoth::Right(_) => Err(String::from("File too small for given message"))
        })
        .collect();
    items
}

pub fn decrypt(img_data_bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    img_data_bytes.iter()
        .map(|&byte| byte % 2 == 1)
        .chunks(8)
        .into_iter()
        .map(|bits| unbitify_message(bits.collect_vec()))
        .take_while(|byte| byte.as_ref().ok() != Some(&String::from("\n")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{copy, remove_file};
    use crate::bits::get_bit_at;

    static TEST_FILE_PATH: &str = "res/test.bmp";

    fn setup() {
        copy("res/BMP_FILE.bmp", TEST_FILE_PATH);
    }

    fn teardown() {
        remove_file(TEST_FILE_PATH);
    }

    #[test]
    fn test_read() {
        setup();

        let res = read("res/test.bmp").unwrap();
        assert!(res.len() > 0);

        teardown();
    }

    #[test]
    fn test_read_and_write() -> Result<(), std::io::Error> {
        let e2e_test_file = "E2E.bin";
        remove_file(e2e_test_file);

        let data: Vec<u8> = vec![1, 2, 3];
        write(e2e_test_file, &data);
        let read_data = read(e2e_test_file)?;

        assert_eq!(read_data, data);
        Ok(())
    }

    #[test]
    fn test_encrypt_over_zeros() {
        let msg = "What is the color of the night?";
        // Add more byte, for the EOF
        let res = encrypt(&vec![0; (msg.len() + 1) * 8], msg).unwrap();

        let expected: Vec<u8> = bitify_message(msg).iter().map(|&bit| bit as u8).collect();

        // Message should be the same, except for the last byte
        assert_eq!(expected.as_slice(), &res[..res.len() - 8]);
        // Last byte should contain the EOF (0x0A)
        assert_eq!([0, 0, 0, 0, 1, 0, 1, 0], &res[res.len() - 8..]);
    }

    #[test]
    fn test_enrypt_over_data() {
        let msg = "Sanguine, my brother. Sanguine.";
        let img = generate_image(msg.len());

        let res = encrypt(&img, msg).unwrap();

        let msg_terminated = (String::from(msg) + "\n").as_str();
        let expected = bitify_message(msg_terminated);

        assert_eq!(expected.len(), res.len());
        // Assert the LSB is the same as the cipher's
        for (&byte, bit) in res.iter().zip(expected) {
            assert_eq!(bit as u8, byte % 2);
        }

        // Assert the rest of the bits are left as the original's
        for (&res, original) in res.iter().zip(img) {
            for bit_idx in 1..8 {
                assert_eq!(get_bit_at(original, bit_idx), get_bit_at(res, bit_idx))
            }
        }
    }

    #[test]
    pub fn test_encrypt_and_decrypt() {
        let msg = "He's dead, Jim...";
        let img = generate_image(msg.len() + 10);
        let encrypted = encrypt(&img, msg).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(msg, decrypted);
    }

    #[test]
    pub fn test_encrypt_to_tiny_image() {
        let msg = "What do we say to the god of death?";
        let img = generate_image(msg.len() - 1);
        let encrypted = encrypt(&img, msg);
        assert!(encrypted.is_err())
    }

    fn generate_image(length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let img: Vec<u8> = (0..(length + 1) * 8).map(|_| rng.gen()).collect();
        img
    }
}