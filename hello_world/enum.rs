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
}