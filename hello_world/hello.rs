fn main() {
  // {} is Rust's string token argument, just like Python
  println!("hi {}", "vinh");

  // {} can be substituted
  println!("hi {0}, this is {1}!", "vinh", "alice");

  // named arguments
  println!("hello, my full name is {first} {last}", 
    first = "vinh",
    last = "nguyen"
    );

  // :b binary
  println!("{} of {:b}", 1, 2);
  
  // align spaces wth `width$`
  println!("{number:>width$}", number=1, width=3); // "   1"

  // alight with things
  println!("{num:>0width$}", num=7, width=3); // 007

  println!("My name is {0}, {0} {1}", "Vinh", "Nguyen");

  // create a structure which contains an `i32`, name is `Structure`
  #[allow(dead_code)]
  struct Structure(i32);

  // println!("this struct `{}` won't print", Structure(1))
}

