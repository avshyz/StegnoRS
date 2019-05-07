pub mod bmp;

trait StegnoReader {
    fn decrypt(self) -> Result<Vec<bool>, String>;
    fn encrypt(self, plain: Vec<bool>) -> Result<(), String>;
}