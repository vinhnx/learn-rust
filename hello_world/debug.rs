#[derive(Debug)]
struct MyStruct(i64);

// put a `Structure` inside of the structure `Deep`. Make it printable also
#[derive(Debug)]
struct Deep(MyStruct);

fn main() {
  // printing with `{:?}` is similar to `{}`
  println!("{:?} months in a year", 12);
  println!("{0:?} {1:?} is {actor:?} name",
    "Vinh", "Nguyen", actor="his"
  );

  println!("now {:?} is printed", MyStruct(9));
  println!("{:?} is also printed", Deep(MyStruct(1)));
}
