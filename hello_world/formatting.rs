use std::fmt::{self, Formatter, Display};

struct City {
  name: &'static str,
  lat: f32,
  longi: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
    let longi_c = if self.longi >= 0.0 { "E" } else { "W" };

    write!(f, "{}: {:.3}o{} {:.3}o{}",
      self.name, self.lat.abs(), lat_c, self.longi.abs(), longi_c)
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl fmt::UpperHex for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RGB ({}, {}, {}) {:#X}",
      self.red, self.green, self.blue, self)
  }
}

fn main() {
  for city in [
    City { name: "Chau Doc", lat: 1.213123, longi: 9.82344 },
    City { name: "Vancouver", lat: 49.25, longi: -123.1 },
  ].iter() {
    println!("{}", city);
  }

  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 },
  ].iter() {
    println!("{}", color);
    // println!("{:?}", color);
  }
}