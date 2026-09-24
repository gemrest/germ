use super::Node;

/// A parsed Gemtext document.
///
/// # Example
///
/// ```rust
/// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
  inner:                    Vec<Node>,
  append_final_line_ending: bool,
  line_ending:              &'static str,
  unclosed_preformatted:    bool,
}

impl Ast {
  /// Parses Gemtext into an AST.
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  pub fn from_owned(value: &(impl AsRef<str> + ?Sized)) -> Self {
    Self::parse(value.as_ref())
  }

  /// Parses Gemtext into an AST.
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  #[allow(clippy::needless_pass_by_value)]
  pub fn from_string(value: impl Into<String>) -> Self {
    Self::parse(&value.into())
  }

  /// Parses the string representation of a value into an AST.
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ = germ::ast::Ast::from_value(r#"=> gemini://gem.rest/ GemRest"#);
  /// ```
  #[must_use]
  pub fn from_value(value: &(impl ToString + ?Sized)) -> Self {
    Self::parse(&value.to_string())
  }

  fn parse(source: &str) -> Self {
    let mut line_ending = None;
    let mut nodes = Vec::new();
    let mut list_items = Vec::new();
    let mut preformatted_alt_text: Option<String> = None;
    let mut preformatted_lines = Vec::new();
    let mut unclosed_preformatted = false;

    for line_with_ending in source.split_inclusive('\n') {
      let line = if let Some(line) = line_with_ending.strip_suffix("\r\n") {
        line_ending.get_or_insert("\r\n");

        line
      } else if let Some(line) = line_with_ending.strip_suffix('\n') {
        line_ending.get_or_insert("\n");

        line
      } else {
        line_with_ending
      };

      if preformatted_alt_text.is_some() {
        if line.starts_with("```") {
          let alt_text = preformatted_alt_text.take().unwrap();
          let text = if preformatted_lines.is_empty() {
            String::new()
          } else {
            let mut text = preformatted_lines.join(line_ending.unwrap_or("\n"));

            text.push_str(line_ending.unwrap_or("\n"));

            text
          };

          nodes.push(Node::PreformattedText {
            alt_text: (!alt_text.is_empty()).then_some(alt_text),
            text,
          });
          preformatted_lines.clear();
        } else {
          preformatted_lines.push(line);
        }

        continue;
      }

      if let Some(alt_text) = line.strip_prefix("```") {
        flush_list(&mut nodes, &mut list_items);

        preformatted_alt_text = Some(alt_text.to_owned());

        continue;
      }

      if let Some(item) = line.strip_prefix("* ") {
        list_items.push(item.to_owned());

        continue;
      }

      flush_list(&mut nodes, &mut list_items);
      nodes.push(parse_line(line));
    }

    flush_list(&mut nodes, &mut list_items);

    if let Some(alt_text) = preformatted_alt_text {
      unclosed_preformatted = true;

      let mut text = preformatted_lines.join(line_ending.unwrap_or("\n"));

      if source.ends_with('\n') && !preformatted_lines.is_empty() {
        text.push_str(line_ending.unwrap_or("\n"));
      }

      nodes.push(Node::PreformattedText {
        alt_text: (!alt_text.is_empty()).then_some(alt_text),
        text,
      });
    }

    Self {
      inner: nodes,
      append_final_line_ending: source.ends_with('\n')
        && (!unclosed_preformatted || preformatted_lines.is_empty()),
      line_ending: line_ending.unwrap_or("\n"),
      unclosed_preformatted,
    }
  }

  /// Builds an AST from a [`Vec`] of [`Node`]s.
  ///
  /// # Example
  ///
  /// ```rust
  /// // This assertion converts the Gemtext "=> / Home" to an AST tree of one
  /// // node, then converts the AST tree back to Gemtext, and compares it against
  /// // the original Gemtext.
  /// assert_eq!(
  ///   germ::ast::Ast::from_nodes(
  ///     germ::ast::Ast::from_string("=> / Home").inner().to_vec()
  ///   )
  ///   .to_gemtext(),
  ///   "=> / Home"
  /// );
  /// ```
  #[must_use]
  pub const fn from_nodes(nodes: Vec<Node>) -> Self {
    Self {
      inner:                    nodes,
      append_final_line_ending: false,
      line_ending:              "\n",
      unclosed_preformatted:    false,
    }
  }

  /// Serialises the document as Gemtext.
  #[must_use]
  pub fn to_gemtext(&self) -> String {
    let mut gemtext = String::new();

    for (index, node) in self.inner.iter().enumerate() {
      if index > 0 {
        gemtext.push_str(self.line_ending);
      }

      match node {
        Node::Text(text) => gemtext.push_str(text),

        Node::Link { to, text } => {
          gemtext.push_str("=>");

          if !to.is_empty() {
            gemtext.push(' ');
            gemtext.push_str(to);
          }

          if let Some(text) = text {
            gemtext.push(' ');
            gemtext.push_str(text);
          }
        }

        Node::Heading { level, text } => {
          gemtext.push_str(&"#".repeat(*level));

          if !text.is_empty() {
            gemtext.push(' ');
            gemtext.push_str(text);
          }
        }

        Node::List(items) => {
          for (item_index, item) in items.iter().enumerate() {
            if item_index > 0 {
              gemtext.push_str(self.line_ending);
            }

            gemtext.push_str("* ");
            gemtext.push_str(item);
          }
        }

        Node::Blockquote(text) => {
          gemtext.push('>');

          if !text.is_empty() {
            gemtext.push(' ');
            gemtext.push_str(text);
          }
        }

        Node::PreformattedText { alt_text, text } => {
          let is_unclosed_last =
            index + 1 == self.inner.len() && self.unclosed_preformatted;

          gemtext.push_str("```");

          if let Some(alt_text) = alt_text {
            gemtext.push_str(alt_text);
          }

          if is_unclosed_last {
            if !text.is_empty() {
              gemtext.push_str(self.line_ending);
              gemtext.push_str(text);
            }
          } else {
            gemtext.push_str(self.line_ending);
            gemtext.push_str(text);

            if !text.is_empty() && !text.ends_with('\n') {
              gemtext.push_str(self.line_ending);
            }

            gemtext.push_str("```");
          }
        }

        Node::Whitespace => {}
      }
    }

    if self.append_final_line_ending {
      gemtext.push_str(self.line_ending);
    }

    gemtext
  }

  /// Returns the parsed nodes.
  ///
  /// # Example
  ///
  /// ```rust
  /// let _ =
  ///   germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#).inner();
  /// ```
  #[must_use]
  pub const fn inner(&self) -> &Vec<Node> { &self.inner }
}

fn flush_list(nodes: &mut Vec<Node>, items: &mut Vec<String>) {
  if !items.is_empty() {
    nodes.push(Node::List(std::mem::take(items)));
  }
}

fn after_optional_whitespace(text: &str) -> &str {
  match text.chars().next() {
    Some(character) if character.is_whitespace() =>
      &text[character.len_utf8()..],
    _ => text,
  }
}

fn parse_line(line: &str) -> Node {
  if let Some(link_content) = line.strip_prefix("=>") {
    return parse_link(link_content);
  }

  if line.starts_with('#') {
    let level = line.bytes().take_while(|byte| *byte == b'#').count();

    if level <= 3 {
      return Node::Heading {
        level,
        text: after_optional_whitespace(&line[level..]).to_owned(),
      };
    }
  }

  if let Some(quote) = line.strip_prefix('>') {
    return Node::Blockquote(after_optional_whitespace(quote).to_owned());
  }

  if line.is_empty() { Node::Whitespace } else { Node::Text(line.to_owned()) }
}

fn parse_link(link_content: &str) -> Node {
  let target_and_label = link_content.trim_start_matches(char::is_whitespace);
  let target_end = target_and_label
    .find(char::is_whitespace)
    .unwrap_or(target_and_label.len());
  let target = &target_and_label[..target_end];
  let label =
    target_and_label[target_end..].trim_start_matches(char::is_whitespace);

  Node::Link {
    to:   target.to_owned(),
    text: (!label.is_empty()).then(|| label.to_owned()),
  }
}
