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

use super::Node;

/// An AST structure which contains an AST tree
///
/// # Example
///
/// ```rust
/// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
  inner: Vec<Node>,
}

impl Ast {
  /// Build an AST tree from Gemtext
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  pub fn from_owned(value: &(impl AsRef<str> + ?Sized)) -> Self {
    Self::from_value(value.as_ref())
  }

  /// Build an AST tree from Gemtext
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  #[allow(clippy::needless_pass_by_value)]
  pub fn from_string(value: (impl Into<String> + ?Sized)) -> Self {
    Self::from_value(&value.into())
  }

  /// Build an AST tree from a value
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_value(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  pub fn from_value(value: &(impl ToString + ?Sized)) -> Self {
    let mut ast = vec![];
    let mut in_preformatted = false;
    let mut in_list = false;
    let source = value.to_string();
    let mut lines = source.lines();

    // Iterate over all lines in the Gemtext `source`
    while let Some(line) = lines.next() {
      // Evaluate the Gemtext line and append its AST node to the `ast` tree
      ast.append(&mut Self::evaluate(
        line,
        &mut lines,
        &mut in_preformatted,
        &mut in_list,
      ));
    }

    if source.chars().last().map_or(false, |c| c == '\n') {
      if let Some(last) = ast.last() {
        if !matches!(last, Node::Whitespace) {
          ast.push(Node::Whitespace);
        }
      }
    }

    Self { inner: ast }
  }

  /// Build an AST tree from a [`Vec`] of [`Node`]s
  ///
  /// # Example
  ///
  /// ```rust
  /// // This assertion converts the Gemtext "=> / Home\n" to an AST tree of one
  /// // node, then converts the AST tree back to Gemtext, and compares it against
  /// // the original Gemtext.
  /// assert_eq!(
  ///   germ::ast::Ast::from_nodes(
  ///     germ::gemini_to_ast!("=> / Home").inner().to_vec()
  ///   )
  ///   .to_gemtext(),
  ///   "=> / Home"
  /// );
  /// ```
  #[must_use]
  pub fn from_nodes(nodes: Vec<Node>) -> Self { Self { inner: nodes } }

  #[must_use]
  pub fn to_gemtext(&self) -> String {
    let mut gemtext = String::new();

    for node in &self.inner {
      match node {
        Node::Text(text) => gemtext.push_str(&format!("{text}\n")),
        Node::Link { to, text } => gemtext.push_str(&format!(
          "=> {}{}\n",
          to,
          text.clone().map_or_else(String::new, |text| format!(" {text}")),
        )),
        Node::Heading { level, text } => gemtext.push_str(&format!(
          "{} {}\n",
          match level {
            1 => "#",
            2 => "##",
            3 => "###",
            _ => "",
          },
          text
        )),
        Node::List(items) => gemtext.push_str(&format!(
          "{}\n",
          items
            .iter()
            .map(|i| format!("* {i}"))
            .collect::<Vec<String>>()
            .join("\n"),
        )),
        Node::Blockquote(text) => gemtext.push_str(&format!("> {text}\n")),
        Node::PreformattedText { alt_text, text } =>
          gemtext.push_str(&format!(
            "```{}\n{}```\n",
            alt_text.clone().unwrap_or_default(),
            text
          )),
        Node::Whitespace => gemtext.push('\n'),
      }
    }

    if gemtext.ends_with('\n') && !gemtext.ends_with("\n\n") {
      gemtext.pop();
    }

    gemtext
  }

  /// The actual AST of `Ast`
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ =
  ///   germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#).inner();
  /// ```
  #[must_use]
  pub const fn inner(&self) -> &Vec<Node> { &self.inner }

  #[allow(clippy::too_many_lines)]
  fn evaluate(
    line: &str,
    lines: &mut std::str::Lines<'_>,
    in_preformatted: &mut bool,
    in_list: &mut bool,
  ) -> Vec<Node> {
    let mut preformatted = String::new();
    let mut alt_text = String::new();
    let mut nodes = vec![];
    let mut line = line;
    let mut list_items = vec![];

    // Enter a not-so-infinite loop as sometimes, we may need to stay in an
    // evaluation loop, e.g., multiline contexts: preformatted text, lists, etc.
    loop {
      // Match the first character of the Gemtext line to understand the line
      // type
      match line.get(0..1).unwrap_or("") {
        "=" if !*in_preformatted => {
          // If the Gemtext line starts with an "=" ("=>"), it is a link line,
          // so splitting it up should be easy enough.
          let line = line.get(2..).unwrap();
          let mut split = line
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
            .into_iter();

          nodes.push(Node::Link {
            to:   split.next().expect("no location in link"),
            text: {
              let rest = split.collect::<Vec<String>>().join(" ");

              if rest.is_empty() { None } else { Some(rest) }
            },
          });

          break;
        }
        "#" if !*in_preformatted => {
          // If the Gemtext line starts with an "#", it is a heading, so let's
          // find out how deep it goes.
          let level =
            line.trim_start().chars().take_while(|&c| c == '#').count();

          nodes.push(Node::Heading {
            level,
            // Here, we are `get`ing the `&str` starting at the `level`-th
            // index, then trimming the start. These operations
            // effectively off the line identifier.
            text: line.get(level..).unwrap_or("").trim_start().to_string(),
          });

          break;
        }
        "*" if !*in_preformatted => {
          // If the Gemtext line starts with an asterisk, it is a list item, so
          // let's enter a list context.
          if !*in_list {
            *in_list = true;
          }

          list_items.push(line.get(1..).unwrap_or("").trim_start().to_string());

          if let Some(next_line) = lines.next() {
            line = next_line;
          } else {
            break;
          }
        }
        ">" if !*in_preformatted => {
          // If the Gemtext line starts with an ">", it is a blockquote, so
          // let's just clip off the line identifier.
          nodes.push(Node::Blockquote(
            line.get(1..).unwrap_or("").trim_start().to_string(),
          ));

          break;
        }
        "`" => {
          // If the Gemtext line starts with a backtick, it's a preformatted
          // toggle, so let's enter a preformatted text context.
          *in_preformatted = !*in_preformatted;

          if *in_preformatted {
            alt_text = line.get(3..).unwrap_or("").to_string();

            if let Some(next_line) = lines.next() {
              line = next_line;
            } else {
              break;
            }
          } else {
            nodes.push(Node::PreformattedText {
              alt_text: if alt_text.is_empty() { None } else { Some(alt_text) },
              text:     preformatted,
            });

            break;
          }
        }
        "" if !*in_preformatted => {
          if line.is_empty() {
            // If the line has nothing on it, it is a whitespace line, as long
            // as we aren't in a preformatted line context.
            nodes.push(Node::Whitespace);
          } else {
            nodes.push(Node::Text(line.to_string()));
          }

          break;
        }
        // This as a catchall. It does a number of things.
        _ => {
          if *in_preformatted {
            // If we are in a preformatted line context, add the line to the
            // preformatted blocks content and increment the line.
            preformatted.push_str(&format!("{line}\n"));

            if let Some(next_line) = lines.next() {
              line = next_line;
            } else {
              break;
            }
          } else {
            // If we are in a list item and hit a catchall, that must mean that
            // we encountered a line which is not a list line, so
            // let's stop adding items to the list context.
            if *in_list {
              *in_list = false;

              nodes.push(Node::Text(line.to_string()));

              break;
            }

            nodes.push(Node::Text(line.to_string()));

            break;
          }
        }
      }
    }

    if !list_items.is_empty() {
      nodes.reverse();
      nodes.push(Node::List(list_items));
      nodes.reverse();
    }

    nodes
  }
}
