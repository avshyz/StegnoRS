extern crate rand;

use std::fs::File;
use std::io::{Read, Write};
use std::fmt::Error;
use rand::Rng;

use crate::bits::{bitify_message, set_bit_at};


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
    let cipher_bits = bitify_message(plain);
    // res will be a vector containing a modified file data, which means it'll of the file's same size
    let mut res: Vec<u8> = Vec::with_capacity(img_data_bytes.len() * 8);

    // -1 because of the eof, which is another byte to the cipher
    if cipher_bits.len() > (img_data_bytes.len() - 1) * 8 {
        Err(String::from("File too small for given message"))
    } else {
        for (idx, &datum) in img_data_bytes.iter().enumerate() {
            if idx < cipher_bits.len() {
                let x = set_bit_at(datum, 0, cipher_bits[idx])?;
                res.push(x);
            } else {
                res.push(datum);
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{copy, remove_file};

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
        let res = encrypt(&vec![0; msg.len() * 8], msg).unwrap();

        let expected: Vec<u8> = bitify_message(msg).iter().map(|&bit| bit as u8).collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_enrypt_over_data() {
        let msg = "Sanguine, my brother. Sanguine.";
        let img_size = msg.len() * 8;

        let mut rng = rand::thread_rng();
        let img: Vec<u8> = (0..img_size).map(|_| rng.gen()).collect();

        let res = encrypt(&img, msg).unwrap();
        let bitified = bitify_message(msg);

        assert_eq!(bitified.len(), res.len());
        for (&byte, bit) in res.iter().zip(bitified) {
            assert_eq!(bit as u8, byte % 2)
        }
    }
}