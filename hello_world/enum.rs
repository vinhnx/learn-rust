enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
  // `match` Rust's pattern matching (switch case in Swift)
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    // casting `c` from inside the enum
    WebEvent::KeyPress(c) => println!("pressed '{}'", c),
    WebEvent::Paste(s) => println!("pasted {}", s),
    WebEvent::Click { x, y } => {
      println!("clicked at x={}, y={}", x, y);
    },
  }
}

// C-like enum

// starts at 0
enum Number {
  Zero,
  One,
  Two,
}

enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 10 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  // `enums can be cast as integers`
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);
  println!("roses are #{:06x}", Color::Red as i32);
  println!("violets are #{:06x}", Color::Blue as i32);
}