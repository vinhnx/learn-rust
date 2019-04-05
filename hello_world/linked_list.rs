use List::*;

enum List {
  // Cons: tuple struct that wraps an element and a pointer to the next node
  Cons(u32, Box<List>),
  // Nil: a node that signifies the end of the linked list
  Nil,
}

// method can be attached to an enum
impl List {
  fn new() -> List { Nil }

  fn prepend(self, element: u32) -> List {
    Cons(element, Box::new(self))
  }

  fn len(&self) -> u32 {
    match *self {
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0
    }  
  }

  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        // `format!` is similar to `print!` but returns a heap allocated string instead of printing to the console
        format!("{}, {}", head, tail.stringify())
      }
      Nil => {
        format!("nil")
      }
    }
  }
}

fn main() {
  let mut list = List::new();
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);
  println!("linked list has length: {}", list.len());
  println!("{}", list.stringify());
}