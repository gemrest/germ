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
#[derive(Debug, PartialEq, Clone, Eq)]
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

impl Node {
  /// Obtain the Gemtext content of a single [`Node`] as a [`String`]
  #[must_use]
  pub fn content(&self) -> String {
    super::Ast::from_nodes(vec![self.to_owned()]).to_gemtext()
  }
}
