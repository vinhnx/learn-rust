#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

fn main() {
  let name = "Vinh";
  let age = 29;
  let person = Person { name, age };
  println!("{:#?}", person);
}
