fn main() {

    // let frankentype_a: u32 = 0x_0000_0001;
    // let frankentype_b: u32 = 0x_0040_0000;
    // let frankentype_c: u32 = 0x_0080_0000;
    // let frankentype_d: u32 = 0x_00C0_0000;

    let frankentype_a: u32 = 0x_7F00_0000;

    println!("10진수: {}", frankentype_a);
    println!("2진수: {:032b}", frankentype_a);
    println!("16진수: {:08X}", frankentype_a);

    let a: f32 = unsafe {
        std::mem::transmute(frankentype_a)
    };

    println!("{}", a);

}
