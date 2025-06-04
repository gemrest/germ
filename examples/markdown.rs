//! This example demonstrates Germ's capabilities for converting Gemtext to
//! Markdown.

fn main() {
  // Convert the Gemtext to Markdown
  let html = germ::convert::from_string(
    germ::EXAMPLE_GEMTEXT,
    &germ::convert::Target::Markdown,
  );

  // Write the Markdown to a file
  std::fs::write("examples/convert.md", html).expect("could not write to file");
}
