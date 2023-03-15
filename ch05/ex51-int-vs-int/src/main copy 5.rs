fn main() {
    for number in 0_u8..=255_u8 {
        println!("{:08b} {}", number, number);
    }
}
