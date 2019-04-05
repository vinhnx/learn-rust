#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
  x: f32,
  y: f32,
}

struct Rect {
  p1: Point,
  p2: Point,
}

fn main() {
  let name = "Vinh";
  let age = 29;
  let vinh = Person { name, age };

  println!("{:?}", vinh);

  let point = Point { x: 0.1, y: 0.2 };
  println!("coordinates: ({}, {})", point.x, point.y);

  // make a new point by using struct update syntax to use the fields of our other one
  let new_point = Point { x: 0.1, ..point };
  println!("({}, {})", new_point.x, new_point.y);

  let Point { x: my_x, y: my_y } = point;
  let _rect = Rect {
    p1:  Point { x: my_x, y: my_y },
    p2: point,
  };

  let _nil = Nil;
  let pair = Pair(1, 0.1);
  
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // composing
  let Pair(integer, decimal) = pair;
  println!("pair contains {:?} and {:?}", integer, decimal);
}