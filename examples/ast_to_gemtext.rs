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
