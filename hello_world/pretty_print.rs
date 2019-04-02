#[derive(Debug)]
struct Person<'a> {
  name: &'a str, // https://stackoverflow.com/a/47641166/1477298
  age: u8
}

fn main() {
  let name = "Vinh";
  let age = 29;
  let person = Person { name, age };
  println!("{:#?}", person);
}

// #[derive(Debug)]
// struct AnotherPerson {
//   name: String,
//   age: u8
// }

// fn main() {
//   // https://stackoverflow.com/a/47641166/1477298
//   // let name = "Vinh"; // error: expected type `std::string::String`
//                         // found type `&str`
//   let name = String::from("Vinh");
//   let age = 29;
//   let person = AnotherPerson { name, age };

//   println!("{:#?}", person);
// }
