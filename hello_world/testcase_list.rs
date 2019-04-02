use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let vec = &self.0;

    // open bracket
    write!(f, "[")?;


    // iterate over `vec` in v while enumerating the iteration count in `count`
    for (count, v) in vec.iter().enumerate() {
      if count != 0 {
        write!(f, ", ")?;
      }
      
      write!(f, "{}: {}", count, v)?; // [0: 1, 1: 2, 2: 3]
    }

    // close bracket
    write!(f, "]") // NOTE TO SELF: Rust has no `return` keyword, so omission of semicolon `;` means return
  }
}

fn main() {
  let v = List(vec![1, 2, 3]);
  print!("{}", v);
}