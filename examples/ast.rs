// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

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
