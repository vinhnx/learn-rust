// import (via `use`) the `fmt` module to make it available
use std::fmt; // import `fmt`

// struct MyStruct(i32);

// // we must implement trait `fmt::Display` in order to use `{}` marker
// // NOTE: can hover on `Display` and copy the code
// impl fmt::Display for MyStruct {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // write strictly the first element into the supplied output
//     // stream: `f`. Returns `fmt::Result` which indicates whether the operation succeeded or failed
//     write!(f, "{}", self.0)
//   }
// }

// this is for auto-implement `Debug` trait
#[derive(Debug)]
struct MinMax(i64, i64); // inferred as typle use (0, 1)

// implement `Display` trait
impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

// impl fmt::Binary for Point2D {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     let val = self.0;
//     write!(f, "{:b}", val) // delegate to i32's implementation
//   }
// }

struct Complex {
  real: f64,
  imagine: f64
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} +{}i", self.real, self.imagine)
  }
}

impl fmt::Debug for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imagine)
  }
}

fn main() {
  let min_max = MinMax(1, 10);
  println!("Display: {}", min_max); // prints: (1, 10)
  println!("Debug: {:?}", min_max); // prints: MinMax(1, 10)

  let point = Point2D { x: 1.0, y: 9.9 };
  println!("Display: {}", point); // (1, 9.9)
  println!("Debug: {:?}", point); // Point2D { x: 1.0, y: 9.9 }

  let big_range = MinMax(-10, 10);
  let small_range = MinMax(-1, 1);
  println!("big range is {big}, small range is {small}", 
    big=big_range,
    small=small_range
  ); // big range is (-10, 10), small range is (-1, 1)

  // how to solve this ?
  // println!("What does Point2D look like in binary: {:b}?", point); 

  let complex = Complex { real: 1.1, imagine: 3.3 };
  println!("Debug: {}", complex);
  println!("Display: {:?}", complex);
}
