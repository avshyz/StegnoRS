extern crate rand;
extern crate itertools;

use std::fs::File;
use std::io::{Read, Write};
use rand::Rng;

use crate::bits::{bitify_str, set_bit_at, unbitify_str};
use self::itertools::{Itertools, EitherOrBoth, Chunk, Chunks};
use std::string::FromUtf8Error;

const EOF: &str = "\n";

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

// TODO - think about changing to non-str variant (to support data encoding), and add the conversion outside
pub fn inject(carrier: &Vec<u8>, plain: &str) -> Result<Vec<u8>, String> {
    let terminated_plain : String = String::from(plain) + EOF;
    let injected_bits = bitify_str(terminated_plain.as_str());
    let items = carrier.iter()
        .zip_longest(injected_bits)
        .map(|x| match x {
            EitherOrBoth::Both(&carrier_byte, injected_bit) => set_bit_at(carrier_byte, 0, injected_bit),
            EitherOrBoth::Left(&carrier_byte) => Ok(carrier_byte),
            EitherOrBoth::Right(_) => Err(String::from("File too small for given message"))
        })
        .collect();
    items
}

pub fn extract(carrier: &Vec<u8>) -> Result<String, FromUtf8Error> {
    carrier.iter()
        .map(|&byte| byte % 2 == 1)
        .chunks(8)
        .into_iter()
        .map(|bits| unbitify_str(bits.collect_vec()))
        .take_while(|byte| byte.as_ref().ok() != Some(&String::from(EOF)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{copy, remove_file};
    use crate::bits::get_bit_at;

    static TEST_INPUT_FILE_PATH: &str = "res/test.bmp";
    static TEST_OUTPUT_FILE_PATH: &str = "res/ciphered.bmp";

    fn setup() {
        copy("res/BMP_FILE.bmp", TEST_INPUT_FILE_PATH);
    }

    fn teardown() {
        remove_file(TEST_INPUT_FILE_PATH);
        remove_file(TEST_OUTPUT_FILE_PATH);
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

        let data: Vec<u8> = vec![1, 2, 3];

        write(e2e_test_file, &data).unwrap();
        let read_data = read(e2e_test_file)?;

        assert_eq!(read_data, data);

        remove_file(e2e_test_file).unwrap();
        Ok(())
    }

    #[test]
    fn test_inject_over_zeros() {
        let msg = "What is the color of the night?";
        // Add more byte, for the EOF
        let res = inject(&vec![0; (msg.len() + 1) * 8], msg).unwrap();

        let expected: Vec<u8> = bitify_str(msg)
            .iter()
            // Adding the eof
            .chain(bitify_str(EOF).iter())
            // Converting to the format appropriate by encryption
            .map(|&bit| bit as u8)
            .collect();
        assert_eq!(expected, res);
    }

    #[test]
    fn test_enrypt_over_data() {
        let msg = "Sanguine, my brother. Sanguine.";
        let carrier = generate_image(msg.len());

        let res = inject(&carrier, msg).unwrap();
        let expected = bitify_str((String::from(msg) + EOF).as_str());

        assert_eq!(expected.len(), res.len());
        // Assert the LSB is the same as the cipher's
        for (&byte, bit) in res.iter().zip(expected) {
            assert_eq!(bit as u8, byte % 2);
        }

        for (&res, original) in res.iter().zip(carrier) {
            for bit_idx in 1..8 {
                assert_eq!(
                    get_bit_at(original, bit_idx),
                    get_bit_at(res, bit_idx)
                )
            }
        }
    }

    #[test]
    pub fn test_encrypt_and_decrypt() {
        let msg = "He's dead, Jim...";
        let img = generate_image(msg.len() + 10);
        let injected = inject(&img, msg).unwrap();
        let extracted = extract(&injected).unwrap();
        assert_eq!(msg, extracted);
    }

    #[test]
    pub fn test_encrypt_to_tiny_image() {
        let msg = "What do we say to the god of death?";
        let img = generate_image(msg.len() - 1);
        let injected = inject(&img, msg);
        assert!(injected.is_err())
    }

    #[test]
    pub fn test_e2e() -> Result<(), String> {
        let msg = "What is a sound of one hand clapping?";

        setup();
        // Encrypting
        let file_data = read(TEST_INPUT_FILE_PATH).unwrap();
        let injected = inject(&file_data, msg).unwrap();
        write(TEST_OUTPUT_FILE_PATH, &injected).unwrap();

        // Decrypting
        let file_data = read(TEST_OUTPUT_FILE_PATH).unwrap();
        let extracted= extract(&file_data).unwrap();
        assert_eq!(msg, extracted);
        teardown();
        Ok(())
    }

    fn generate_image(length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let img: Vec<u8> = (0..(length + 1) * 8).map(|_| rng.gen()).collect();
        img
    }
}