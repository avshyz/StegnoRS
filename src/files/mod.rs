pub mod bmp;
use ::bits::PseudoByte;

trait StegnoReader {
    fn decrypt(self) -> Result<Vec<PseudoByte>, String>;
    fn encrypt(self, plain: Vec<PseudoByte>) -> Result<(), String>;
}