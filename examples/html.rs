//! This example demonstrates Germ's capabilities for converting Gemtext to
//! HTML.

fn main() {
  let html = germ::convert::from_string(
    include_str!("example.gmi"),
    &germ::convert::Target::HTML,
  );

  println!("{html}");
}
