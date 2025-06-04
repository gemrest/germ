//! This example demonstrates Germ's capabilities for parsing Gemtext into an
//! abstract syntax tree.

fn main() {
  // Parse `EXAMPLE_GEMTEXT` into an abstract syntax tree
  let ast = germ::ast::Ast::from_string(germ::EXAMPLE_GEMTEXT);
  // Get the nodes of the abstract syntax tree
  let ast_nodes = ast.inner();

  // Print the abstract syntax tree nodes, one-by-one
  for node in ast_nodes {
    println!("{:?}", node);
  }
}
