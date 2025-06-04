//! This example demonstrates Germ's capabilities for converting Gemtext to
//! HTML.

fn main() {
  // Convert the Gemtext to HTML
  let html = germ::convert::from_string(
    germ::EXAMPLE_GEMTEXT,
    &germ::convert::Target::HTML,
  );

  // Write the HTML to a file
  std::fs::write("examples/convert.html", html)
    .expect("could not write to file");
}
