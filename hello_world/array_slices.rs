use std::mem;

fn analyze_slice(slice: &[i32]) {
  println!("first: {}", slice[0]);
  println!("{} element", slice.len());
}

fn main() {
  // fixed-size array
  let xs: [i32; 5] = [1,2,3,4,5];
  let ys: [i32; 500] = [0; 500];

  println!("first: {}", xs[0]);
  
  // arrays are stack allocated
  println!("array occupies {} bytes", mem::size_of_val(&xs));

  // borrow/reference (&)
  analyze_slice(&xs);

  analyze_slice(&ys[1..4]);

  // println!("{}", xs[5]);
}
