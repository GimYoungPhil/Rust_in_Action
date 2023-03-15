fn main() {
    for number in -128_i8..=127_i8 {
        println!("{:08b} {}", number, number);
    }
}
