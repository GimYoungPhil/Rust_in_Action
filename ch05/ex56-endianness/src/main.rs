use std::mem;

fn main() {
    let big_endian: [u16; 1] = [0x0001];
    let little_endian: [u16; 1] = [0x0100];

    let a: u16 = unsafe { mem::transmute(big_endian) };
    let b: u16 = unsafe { mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
}
