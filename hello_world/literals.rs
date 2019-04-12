fn main() {
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  let i = 1;
  let f = 1.0;

  // `size_of_val` returns the size of a variable in bytes
  println!("{}", std::mem::size_of_val(&x));
  println!("{}", std::mem::size_of_val(&y));
  println!("{}", std::mem::size_of_val(&z));
  println!("{}", std::mem::size_of_val(&i));
  println!("{}", std::mem::size_of_val(&f));
}