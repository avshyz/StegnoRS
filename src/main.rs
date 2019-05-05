use std::any::Any;
use stegno::bits as bits;

fn main() {
    for i in 0..8 {

        println!("{:?}", bits::get_bit_at(!1, i));
    }
}
