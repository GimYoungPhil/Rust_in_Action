fn main() {
    let zero: u8 = 0b0000_0000;
    let one:  u8 = 0b0000_0001;
    let two:  u8 = 0b0000_0010;

    let onehundred_25: u8 = 0b1111_1101;
    let onehundred_26: u8 = 0b1111_1110;
    let onehundred_27: u8 = 0b1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", onehundred_25, onehundred_26, onehundred_27);
}
