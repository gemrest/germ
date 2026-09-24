//! This example demonstrates Germ's capabilities for parsing Gemtext into an
//! abstract syntax tree.

fn main() {
  let ast = germ::ast::Ast::from_string(germ::EXAMPLE_GEMTEXT);

  for node in ast.inner() {
    println!("{:?}", node);
  }
}
