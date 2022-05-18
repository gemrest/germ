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

//! Build AST trees from Gemtext

/// A Gemtext AST node.
///
/// Each Gemtext line is a `Node`, and some lines can even be grouped together,
/// such as the `Node::List` `Node`!
///
/// # Gemtext Resources
///
/// - [Gemtext Documentation](https://gemini.circumlunar.space/docs/gemtext.gmi)
/// - [Gemtext Cheatsheet](https://gemini.circumlunar.space/docs/cheatsheet.gmi).
/// - [Gemini Specification](https://gemini.circumlunar.space/docs/specification.gmi).
#[derive(Debug, PartialEq)]
pub enum Node {
  /// A text line
  ///
  /// # Example
  ///
  /// ```gemini
  /// This is a text line
  /// ```
  Text(String),
  /// A link line
  ///
  /// # Examples
  ///
  /// ```gemini
  /// => /this-is-the-to This is the text
  ///
  /// => gemini://to.somewhere.link
  /// ```
  Link {
    /// The location that a link line is pointing to
    ///
    /// # Examples
    ///
    /// ```gemini
    /// => /this-is-the-to This is the text
    ///
    /// => gemini://to.somewhere.link
    /// ```
    to:   String,
    /// The text a link line *may* have
    ///
    /// # Examples
    ///
    /// ```gemini
    /// => /this-is-the-to This line has text, unlike the next one.
    ///
    /// => gemini://to.somewhere.link
    /// ```
    text: Option<String>,
  },
  /// A heading line
  ///
  /// # Examples
  ///
  /// ```gemini
  /// # This is a heading
  ///
  /// ## This is a sub-heading
  ///
  /// ### This is a sub-sub-heading
  /// ```
  Heading {
    /// The level of a heading
    ///
    /// # Examples
    ///
    /// ```gemini
    /// # This is a level 1 heading
    ///
    /// ## This is a level 2 sub-heading
    ///
    /// ### This is a level 3 sub-sub-heading
    /// ```
    level: usize,
    /// The text of a heading
    ///
    /// # Examples
    ///
    /// ```gemini
    /// # This is the headings text
    ///
    /// # This is also the headings text
    /// ```
    text:  String,
  },
  /// A collection of sequential list item lines
  ///
  /// # Examples
  ///
  /// ```gemini
  /// * These are
  /// * sequential list
  /// * items.
  /// ```
  List(Vec<String>),
  /// A blockquote line
  ///
  /// # Examples
  ///
  /// ```gemini
  /// > This is a blockquote line
  ///
  /// > This is also a blockquote line
  /// ```
  Blockquote(String),
  /// A preformatted block
  ///
  /// # Examples
  ///
  /// Try to ignore the leading backslash in-front of the triple backticks,
  /// they are there to not confuse the Markdown engine.
  ///
  /// ```gemini
  /// \```This is the alt-text
  /// This is the preformatted block
  ///
  /// This is the rest of the preformatted block
  /// \```
  /// ```
  PreformattedText {
    /// A preformatted blocks alt-text
    ///
    /// # Examples
    ///
    /// Try to ignore the leading backslash in-front of the triple backticks,
    /// they are there to not confuse the Markdown engine.
    ///
    /// ```gemini
    /// \```This is the alt-text
    /// This is the preformatted block
    ///
    /// This is the rest of the preformatted block
    /// \```
    /// ```
    alt_text: Option<String>,
    /// A preformatted blocks content
    ///
    /// # Examples
    ///
    /// Try to ignore the leading backslash in-front of the triple backticks,
    /// they are there to not confuse the Markdown engine.
    ///
    /// ```gemini
    /// \```This is the alt-text
    /// This is the preformatted blocks content
    ///
    /// This is the rest of the preformatted blocks content
    /// \```
    /// ```
    text:     String,
  },
  /// A whitespace line, a line which contains nothing but whitespace.
  Whitespace,
}

/// Build an AST tree from Gemtext.
///
/// # Example
///
/// ```rust
/// germ::ast::build(r#"=> gemini://gem.rest/ GemRest"#); 
/// ```
#[must_use]
pub fn build(source: &str) -> Vec<Node> {
  let mut ast = vec![];
  let mut in_preformatted = false;
  let mut in_list = false;
  let mut lines = source.lines();

  // Iterate over all lines in the Gemtext `source`
  while let Some(line) = lines.next() {
    // Evaluate the Gemtext line and append its AST node to the `ast` tree
    ast.append(&mut evaluate(
      line,
      &mut lines,
      &mut in_preformatted,
      &mut in_list,
    ));
  }

  ast
}

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
    // Match the first character of the Gemtext line to understand the line type
    match line.get(0..1).unwrap_or("") {
      "=" => {
        // If the Gemtext line starts with an "=" ("=>"), it is a link line, so
        // splitting it up should be easy enough.
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

            if rest.is_empty() {
              None
            } else {
              Some(rest)
            }
          },
        });

        break;
      }
      "#" => {
        // If the Gemtext line starts with an "#", it is a heading, so let's
        // find out how deep it goes.
        let level = match line.get(0..3) {
          Some(root) =>
            if root.contains("###") {
              3
            } else if root.contains("##") {
              2
            } else if root.contains('#') {
              1
            } else {
              0
            },
          None => 0,
        };

        nodes.push(Node::Heading {
          level,
          // Here, we are `get`ing the `&str` starting at the `level`-th index,
          // then trimming the start. These operations effectively off the line
          // identifier.
          text: line.get(level..).unwrap_or("").trim_start().to_string(),
        });

        break;
      }
      "*" => {
        // If the Gemtext line starts with an asterisk, it is a list item, so
        // let's enter a list context.
        if !*in_list {
          *in_list = true;
        }

        list_items.push(line.get(1..).unwrap_or("").trim_start().to_string());

        line = lines.next().unwrap();
      }
      ">" => {
        // If the Gemtext line starts with an ">", it is a blockquote, so let's
        // just clip off the line identifier.
        nodes.push(Node::Blockquote(
          line.get(1..).unwrap_or("").trim_start().to_string(),
        ));

        break;
      }
      "`" => {
        // If the Gemtext line starts with a backtick, it is a list item, so
        // let's enter a preformatted text context.
        *in_preformatted = !*in_preformatted;

        if *in_preformatted {
          alt_text = line.get(3..).unwrap_or("").to_string();
          line = lines.next().unwrap();
        } else {
          nodes.push(Node::PreformattedText {
            alt_text: if alt_text.is_empty() {
              None
            } else {
              Some(alt_text)
            },
            text:     preformatted,
          });

          break;
        }
      }
      "" if !*in_preformatted => {
        // If the line has nothing on it, it is a whitespace line, as long as we
        // aren't in a preformatted line context.
        nodes.push(Node::Whitespace);

        break;
      }
      // This as a catchall, it does a number of things.
      _ => {
        if *in_preformatted {
          // If we are in a preformatted line context, add the line to the
          // preformatted blocks content and increment the line.
          preformatted.push_str(&format!("{}\n", line));

          line = lines.next().unwrap();
        } else {
          // If we are in a list item and hit a catchall, that must mean that we
          // encountered a line which is not a list line, so let's stop adding
          // items to the list context.
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
