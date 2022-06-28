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

pub fn convert(source: &[Node]) -> String {
  let mut html = String::new();

  // Since we have an AST tree of the Gemtext, it is very easy to convert from
  // this AST tree to an alternative markup format.
  for node in source {
    match node {
      Node::Text(text) => html.push_str(&format!("<p>{}</p>", text)),
      Node::Link {
        to,
        text,
      } => {
        html.push_str(&format!(
          "<a href=\"{}\">{}</a><br>",
          to,
          text.clone().unwrap_or_else(|| to.clone())
        ));
      }
      Node::Heading {
        level,
        text,
      } => {
        html.push_str(&format!(
          "<{}>{}</{0}>",
          match level {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            _ => "p",
          },
          text
        ));
      }
      Node::List(items) =>
        html.push_str(&format!(
          "<ul>{}</ul>",
          items
            .iter()
            .map(|i| format!("<li>{}</li>", i))
            .collect::<Vec<String>>()
            .join("\n")
        )),
      Node::Blockquote(text) =>
        html.push_str(&format!("<blockquote>{}</blockquote>", text)),
      Node::PreformattedText {
        text, ..
      } => {
        html.push_str(&format!("<pre>{}</pre>", text));
      }
      Node::Whitespace => {}
    }
  }

  html
}
