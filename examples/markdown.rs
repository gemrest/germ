//! This example demonstrates Germ's capabilities for converting Gemtext to
//! Markdown.

fn main() {
  let markdown = germ::convert::from_string(
    include_str!("example.gmi"),
    &germ::convert::Target::Markdown,
  );

  println!("{markdown}");
}
