// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2025 Fuwn <contact@fuwn.me>
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
// Copyright (C) 2022-2025 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

use {crate::ast::Node, std::fmt::Write};

pub fn convert(source: &[Node]) -> String {
  let mut markdown = String::new();

  // Since we have an AST tree of the Gemtext, it is very easy to convert from
  // this AST tree to an alternative markup format.
  for node in source {
    match node {
      Node::Text(text) => {
        let _ = writeln!(&mut markdown, "{text}");
      }
      Node::Link { to, text } => markdown.push_str(&text.clone().map_or_else(
        || format!("<{to}>\n"),
        |text| format!("[{text}]({to})\n"),
      )),
      Node::Heading { level, text } => {
        let _ = writeln!(
          &mut markdown,
          "{} {}",
          match level {
            1 => "#",
            2 => "##",
            3 => "###",
            _ => "",
          },
          text
        );
      }
      Node::List(items) => {
        let _ = writeln!(
          &mut markdown,
          "{}",
          items
            .iter()
            .map(|i| format!("- {i}"))
            .collect::<Vec<String>>()
            .join("\n"),
        );
      }
      Node::Blockquote(text) => {
        let _ = writeln!(&mut markdown, "> {text}");
      }
      Node::PreformattedText { alt_text, text } => {
        let _ = writeln!(
          &mut markdown,
          "```{}\n{}```",
          alt_text.clone().unwrap_or_default(),
          text
        );
      }
      Node::Whitespace => markdown.push('\n'),
    }
  }

  markdown
}
