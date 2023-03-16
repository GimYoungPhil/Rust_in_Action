fn main() {
    let a: f32 = 0.0;

    // println!("{}", a);
    // println!("{:b}", a); // Error
    // println!("{:x}", a); // Error

    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);
    println!("{:08X}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}
