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
