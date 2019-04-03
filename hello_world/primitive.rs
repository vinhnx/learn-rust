fn main() {
  // can be type annotated
  let logical: bool = true;
  // or can be inferred
  // let logical = true

  let a_float: f64 = 1.0; // regular annotation
  let an_integer = 5i32; // suffix annotation
  // ~= let an_integer: i32 = 5;
  println!("{}", an_integer);

  let default_float = 3.0; // `f64`
  let default_integer = 1; // `i32`

  // type can also be inteferred from context
  let mut inferred_type = 111; // type i64
  inferred_type = 4294967296i64;

  let mut mutable = 12;
  mutable = 21;

  // mutable = true;

  // variables can be overwritten with shadowing
  let mutable = true;
}