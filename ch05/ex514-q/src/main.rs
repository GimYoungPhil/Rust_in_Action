fn main() {
  let half: f32 = 0.5;

  println!("{:x}", half.to_bits());
  // 0011_1111_0000_0000_0000_0000_0000_0000

  let a: u8 = 126;
  println!("{:b}", a);
}
