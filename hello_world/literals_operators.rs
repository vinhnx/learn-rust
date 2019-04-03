fn main() {
  println!("1 + 2 = {}", 1u32 + 2);

  println!("1 - 2 = {}", 1i32 - 2);

  // AND: &&
  // OR: &&
  // negative: !

  // bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x90 >> 2 is 0x{:x}", 0x90u32 >> 2);

  // can use underscore to improve integer readability: 1_000
  println!("one thousand is written as {}", 1_000u32);
}