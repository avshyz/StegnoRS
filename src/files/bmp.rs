use std::fs::File;
use std::io::{Read, Write};
use crate::bits::{bitify_message, set_bit_at};
use std::fmt::Error;

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(buffer)
}

pub fn write(path: &str, data: &Vec<u8>) -> Result<(), std::io::Error> {
    let mut f = File::create(path)?;
    f.write_all(data.as_slice())?;
    Ok(())
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
}