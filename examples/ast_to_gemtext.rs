//! This example converts Gemtext into an abstract syntax tree and then back
//! into Gemtext, demonstrating both Germ's parsing and generation capabilities.

fn main() {
  // Parse `EXAMPLE_GEMTEXT` into an abstract syntax tree
  let ast = germ::ast::Ast::from_string(germ::EXAMPLE_GEMTEXT);
  // Convert the abstract syntax tree back to Gemtext
  let gemtext = ast.to_gemtext();

  // Print the Gemtext, identical to `EXAMPLE_GEMTEXT`
  println!("{gemtext}");
}
