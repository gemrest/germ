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

use crate::ast::Node;

pub fn convert(source: Vec<Node>) -> String {
  let mut markdown = String::new();

  // Since we have an AST tree of the Gemtext, it is very easy to convert from
  // this AST tree to an alternative markup format.
  for node in source {
    match node {
      Node::Text(text) => markdown.push_str(&format!("{}\n", text)),
      Node::Link {
        to,
        text,
      } =>
        markdown.push_str(&*text.map_or_else(
          || format!("<{}>\n", to),
          |text| format!("[{}]({})\n", text, to),
        )),
      Node::Heading {
        level,
        text,
      } => {
        markdown.push_str(&format!(
          "{} {}\n",
          match level {
            1 => "#",
            2 => "##",
            3 => "###",
            _ => "",
          },
          text
        ));
      }
      Node::List(items) =>
        markdown.push_str(&format!(
          "{}\n",
          items
            .into_iter()
            .map(|i| format!("- {}", i))
            .collect::<Vec<String>>()
            .join("\n"),
        )),
      Node::Blockquote(text) => markdown.push_str(&format!("> {}\n", text)),
      Node::PreformattedText {
        alt_text,
        text,
      } => {
        markdown.push_str(&format!(
          "```{}\n{}```\n",
          alt_text.unwrap_or_else(|| "".to_string()),
          text
        ));
      }
      Node::Whitespace => markdown.push_str("\n\n"),
    }
  }

  markdown
}
