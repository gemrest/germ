//! This example demonstrates Germ's capabilities for converting Gemtext to
//! Markdown.

fn main() {
  let markdown = germ::convert::from_string(
    germ::EXAMPLE_GEMTEXT,
    &germ::convert::Target::Markdown,
  );

  println!("{markdown}");
}
