enum Status {
  Foo, 
  Bar
}

enum Work {
  This,
  That
}

fn main() {
  // explicitly `use` each name so they are available without manual scoping
  use Status::{Foo, Bar};
  // automatically `use` each name inside `Work`
  use Work::*;

  // 
  let status = Foo;
  match status {
    // note the lack of scoping, because of the explicit `use` above
    Foo => println!("foo called"),
    Bar => println!("bar called"),
  }

  let work = That;
  match work {
    This => println!("this!"),
    That => println!("that!"),
  }
}