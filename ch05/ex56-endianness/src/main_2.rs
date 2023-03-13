use std::mem;

fn main() {
    let big_endian: [u8; 2] = [0x00, 0x01];
    let little_endian: [u8; 2] = [0x01, 0x00];

    let a: u16 = unsafe { mem::transmute(big_endian) };
    let b: u16 = unsafe { mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
}
