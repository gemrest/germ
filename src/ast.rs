//! Build AST trees from Gemtext

mod container;
mod node;

#[cfg(feature = "macros")] mod macros;

pub use {container::Ast, node::Node};
