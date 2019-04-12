fn main() {
  let long_lived_binding = 1;

  // scope
  {
    let short_lived_binding = 2;
    println!("inner short: {}", short_lived_binding);
    let long_lived_binding = 5f32;
    println!("inner long: {}", long_lived_binding);
  }

  println!("outer long: {}", long_lived_binding);

  // this binding will "shadow" the previous binding
  let long_lived_binding = "a";
  println!("outer long: {}", long_lived_binding);
}