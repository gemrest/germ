//! This example demonstrates Germ's capabilities for converting Gemtext to
//! HTML.

fn main() {
  let html = germ::convert::from_string(
    germ::EXAMPLE_GEMTEXT,
    &germ::convert::Target::HTML,
  );

  println!("{html}");
}
