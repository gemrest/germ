//! This example converts Gemtext into an abstract syntax tree and then back
//! into Gemtext.

fn main() {
  let ast = germ::ast::Ast::from_string(include_str!("example.gmi"));

  print!("{}", ast.to_gemtext());
}
