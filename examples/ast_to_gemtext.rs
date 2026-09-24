//! This example converts Gemtext into an abstract syntax tree and then back
//! into Gemtext.

fn main() {
  let ast = germ::ast::Ast::from_string(germ::EXAMPLE_GEMTEXT);

  print!("{}", ast.to_gemtext());
}
